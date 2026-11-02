# V109 cold AUDIO/AOUT/PCM dependencies

Offline audit, 2026-11-01. No TV access, module loading, reboot, or runtime changes. This extends the prior `TEE_USERSPACE_DEPENDENCIES.md` audio finding by resolving the TEE-type field, external firmware filenames, and audio TA.

## Result

The ordinary PCM Open/Write path does not itself need a vendor audio daemon, AudioFlinger, or Audio HAL. Cold `MI_AUDIO_Init` is broader: it enters the stock Utopia audio initialization, loads DSP firmware, and conditionally calls the audio OP-TEE trusted application. The original G06 command line selects OP-TEE. Removing all vendor normal-world userspace therefore requires either a working replacement for the TEE supplicant RPC service or further hardware evidence that bootloader initialization legitimately skips this branch. Warm audible success alone does not establish either.

The kernel contains an alternate non-TEE boot configuration, but this is a whole-platform choice, not a demonstrated safe per-audio override on this board. No fake initialization flags, kernel-memory patch, or secure-mode downgrade is proposed here.

## Required external DSP files

The stock DT node `/Mstar-sys/firmware/audio` contains:

```
path = "/vendor/lib/utopia/audio_bin/";
```

`HAL_AUDSP_DspLoadCode` obtains this through `MDrv_Sys_GetDTSValue`, then builds `%s%s` paths and directly opens/reads the files through `MsFS_Fopen`/`MsFS_Fread`. This is not a userspace firmware-helper request. The compiled `sExternalFirmwareInfo` table has four enabled entries, with exact expected sizes:

| File under `/vendor/lib/utopia/audio_bin/` | Bytes |
|---|---:|
| aucode_adec_r2.bin | 2,181,164 |
| aucode_adec_r2_MS12V22.bin | 1,625,588 |
| aucode_asnd_r2.bin | 1,224,064 |
| aucode_asnd_r2_MS12V22.bin | 1,499,500 |

The loader attempts to open/read all four enabled entries before choosing the decoder/sound images. Thus all four should remain available in the first minimal cold image; a smaller set has not been established. The memory-map configuration also remains required; `MI_AUDIO_Init` obtains DSP memory through `MI_SYS_GetMmapLayout` and sets DSP bases before `MApi_AUDIO_Initialize`.

Read-only extraction preserved these four files and the audio TA under `assets/research/audio-agent-direct/cold-firmware/`. `manifest.json` records source image, exact source/runtime paths, lengths and SHA256 hashes. Original images were not modified. Every firmware length matches both the original vendor directory entry and the kernel's compiled expected length.

## Exact cold call chain

```
mik: MI_AUDIO_Init @0x9a8e4
  -> MApi_AUDIO_WritePreInitTable @call0x9b288
  -> configure output/path, audio INI and memory-map parameters
  -> MApi_AUDIO_Initialize @call0x9bf8c
utpa2k: _MApi_AUDIO_Initialize @0x337264
  -> MDrv_AUDIO_SHM_Init / MDrv_AUDIO_Mutex_Init
  -> MDrv_AUDIO_PCM_Init @call0x3372fc
  -> test MDrv_AUDIO_GET_INIT_FLAG
  -> if fresh: resolve TEE type, initialize registers/memory
  -> MDrv_SOUND_Init @call0x3374ac
  -> HAL_SOUND_Init2 @0x3b3c7c
     -> HAL_AUDSP_DspLoadCode(32) @call0x3b3c94
     -> HAL_AUDIO_SeSystemLoadCode @0x39a9b8
        -> HAL_AUDSP_DspLoadCode(74) @call0x39aa18
```

Code74 enters the external firmware loading path at `0x3b9258..0x3b946c`. The later code chooses normal/MS12 image variants using `g_AudioVars2+0x4d4`. It can skip decoder reload if a preexisting flag at offset9 indicates that code was already loaded. This is an existing legitimate conditional; it is not authority to write the flag ourselves.

`MI_PCM_Init @0x1ed024` creates mutex/pipe infrastructure, registers the PCM module and clears its slot table. It does not itself load the DSP files. The called `HAL_AUDIO_PCM_Init @0x39d984` sets up PCM memory/instance infrastructure; its inspected body has no TEE or filesystem call. AOUT initialization creates output routing/amp infrastructure and can invoke GPIO initialization; this is distinct from the AD82111 userspace register programming already replaced in Rust.

## TEE field resolved

The earlier unknown `g_AudioVars2+0x24f8` is populated by `_MApi_AUDIO_Initialize`:

- `0x337410`: `MDrv_SYS_ReadKernelCmdLine`.
- `0x337418`: `MDrv_SYS_GetTEEInfo`.
- `0x337454..0x337460`: store returned type at offset0x24f8.

`MDrv_SYS_GetTEEInfo @0x236d0` maps `optee` to2, `nuttx` to1, otherwise0. The archived physical G06 command line in `assets/research/kexec/load-check/20261031T233605.957346Z/cmdline.txt` explicitly has `tee_mode=optee`.

At `HAL_AUDSP_DspLoadCode+0x890`, type<=1 uses a direct normal-world memory copy. Type>1 reaches `MDrv_AUDIO_TEE_R_Send_Cmd(3, ...)` at0x3b96f8. That wrapper dispatches type2 to `MDrv_AUDIO_OPTEE_R_Send_Cmd`; type1 returns success without that OP-TEE call. The OP-TEE wrapper initializes context, opens the audio TA, invokes the command, closes the session and finalizes context. The kernel TEEC implementation uses `tee_client_open_session` for its newer OP-TEE branches, so removing userspace `libteec` alone does not remove the RPC dependency.

### Is there a supported override?

The exact stock kernel function `TEE_MODE_INFO_setup @0xc1110980` recognizes `optee`, `nuttx`, `secarm`, and `no-tee-ns`; its exported `TEEINFO_TYPTE` feeds Utopia. Utopia's `MDrv_SYS_ReadKernelCmdLine` converts types1/2/3 to nuttx/optee/optee, and everything else to notee. Thus the kernel contains a compiled non-TEE platform path. No documented per-audio control or runtime setter was found. Changing the boot choice has unverified consequences for reserved/protected DSP memory and the existing secure-world configuration. It cannot be called a validated conservative fix on this TV.

## Audio trusted application and supplicant scope

Audio TA UUID: `778ee68e-6ee0-11e8-adc0-fa7ae01bbebc`.

The vendor filename omits the final UUID hyphen:

```
/mnt/vendor/tvservice/glibc/optee_armtz/778ee68e-6ee0-11e8-adc0fa7ae01bbebc.ta
```

It is145,328 bytes in the original `tvservice.img`; the extracted copy's SHA256 is in the manifest. Its container begins `YCED`, not ELF, and is opaque to this normal-world call-graph audit. Do not infer its internal filesystem/RPMB usage from the normal-world command wrapper.

The existing `/system/bin/tee-supplicant` binary is preserved as `assets/research/native-vendor-stack/tee-supplicant`. It imports libteec.so, libc++.so, libc.so, libm.so and libdl.so, opens `/dev/teepriv%zu`, and contains TA-file loading, shared-memory mapping, filesystem and RPMB support. Config `/vendor/tvconfig/config/optee/teefs_path.ini` sets:

```
ta_dir_path=/mnt/vendor/tvservice/glibc/optee_armtz
ta_test_dir_path=/mnt/vendor/tmp/optee_armtz
teefs_path=/vendor/tvcertificate
```

A Rust replacement can own the normal-world receive/respond loop, shared-memory lifecycle and allowlisted TA loading while retaining opaque DSP/TA firmware. However, the exact minimum RPC command set for audio alone is **not proven** by this static audit. TA loading and shared-memory handling are the first capabilities to implement; persistent storage, RPMB, socket or other RPC requirements must be observed before claiming they can be omitted. Do not implement guessed successful replies for unsupported RPCs.

The discriminating future test is a cold owning-PID1 run with each INIT bracketed in the log and a tracing supplicant transport that records RPC command IDs/argument shapes and TA paths. On a block, preserve task stack/wait-channel evidence. The existing full supplicant can serve as a controlled baseline during development; no such test was run here.

## Evidence files

Generated disassembly in `assets/research/audio-agent-direct/`: `MI_AUDIO_Init.txt`, `_MApi_AUDIO_Initialize.txt`, `HAL_AUDIO_PCM_Init.txt`, `HAL_SOUND_Init2.txt`, `HAL_AUDIO_SeSystemLoadCode.txt`, `HAL_AUDSP_DspLoadCode.txt`, `MDrv_SYS_GetTEEInfo.txt`, `MDrv_SYS_ReadKernelCmdLine.txt`, `MDrv_AUDIO_TEE_R_Send_Cmd.txt`, `MDrv_AUDIO_OPTEE_R_Send_Cmd.txt`, `_MDrv_SYS_TEEC_Open.txt`, and `TEE_MODE_INFO_setup.txt`.

Inputs are the exact preserved V109 mik/utpa2k modules, original vendor/tvservice partitions, stock DT dump and archived physical-device command line. Assembly addresses are module-relative except the kernel TEE parser's virtual address. Indirect Utopia dispatch and trusted-application internals remain limits of static analysis.
