# TEE and vendor userspace dependencies of the owned display/audio path

Offline audit, 2026-11-01. No hardware access, processes stopped, module loads,
TEE calls, or configuration changes were performed for this audit.

## Decision

Do not remove tee-supplicant from the next cold-start attempt on the assumption
that unprotected RGB/PCM automatically avoids TEE. The stock MI_DISP_Init path
includes HDCP key initialization. That incidental dependency can potentially be
removed with existing configuration, without changing the RGB scanout API.

Normal direct RGB presentation and normal PCM enqueue have no direct TEE call
in the inspected paths. This is narrower than proving a fully cold boot without
TEE: Utopia indirect dispatch and some initial hardware setup remain unresolved.

## Exact cold display dependency and a reversible way around it

The firmware call chain is:

    mik:MI_DISP_Init
      -> mik:_MI_DISP_XC_Init
      -> xcker:MI_DISP_IMPL_XC_Init_EX
      -> hdcp2x:HDCP2_LoadKey(1)
      -> hdcp2x:hdcp2x_initHdcpTeeService
      -> utpa2k:MDrv_SYS_TEEC_{InitializeContext,OpenSession,...}

`MI_DISP_IMPL_XC_Init_EX` calls HDCP2_LoadKey at 0x19d04. At 0x19d08 it
compares the boolean return against zero; the branch at 0x19d0c skips the
subsequent HDCP2_InitHDMI call when key loading returns false. The HDMI init
call itself is at 0x19d60.

`HDCP2_LoadKey` checks DISABLE_LOAD_RX_KEY for argument 1, or
DISABLE_LOAD_TX_KEY for argument 0 (configGetInt at 0x20074 / 0x2009c).
If that value equals 1, the branch at 0x200e4 bypasses the TEE initializer
at 0x20140. The skipped path returns false: status 0x80010001 is converted
using MVN/LSR at 0x2023c. This also prevents the caller's HDMI HDCP2 init.
These are ordinary firmware configuration keys and module parameters, not
speculative new flags: `config/hdcp/hdcp2.ini` has both keys at 0, lines 45-46.
IMPORTANT: module init then unconditionally overwrites the config values from
its charp module parameters, whose compiled defaults are strings "0".
Changing the INI alone is insufficient. The exact .modinfo confirms both
DISABLE_LOAD_RX_KEY and DISABLE_LOAD_TX_KEY parameters. A future isolated
cold load must supply `DISABLE_LOAD_RX_KEY=1 DISABLE_LOAD_TX_KEY=1` when
loading hdcp2x.ko. That is a candidate for removing this particular dependency;
it has NOT been tested. Do not unload an active dependency chain to try this
warm, and do not edit the firmware partition or installed configuration.

Loading hdcp1x.ko itself only initializes logging; its init_module at
0x9e28..0x9eac has no TEE initialization. hdcp2x init_module at
0x240b4..0x24324 initializes its configuration/logging, likewise without
calling the TEE initializer. Their presence as module link dependencies does
not by itself require a userspace supplicant.

HDCP1 is a separate path: `HDCP1xRx_APP_FillKey` calls its TEE initializer
at 0x98a0 even if its key pointer was null (it substitutes a zero buffer).
The path from MI_DISP_Init goes through mi_extin_NotifyDispStatus, which
checks `_bExtInInited` at 0x1862b8 and returns if false, then checks each
HDMI entry before MI_DISP_IMPL_XC_Init_HDMIRx at 0x186330. A minimal cold
configuration that never initializes EXTIN should not take that path.
A broad selector-1 fan-out or starting the vendor daemon can initialize EXTIN
and invalidate that assumption. Avoid that fan-out; use only needed direct
module init calls. Other HDCP1 callers and all Utopia indirect callbacks have
not been exhaustively eliminated.

## SYS, OSD, memory and warm presentation

The exported MI_SYS_Init and MI_OSD_Init functions have no direct TEEC calls.
MI_SYS_Init performs global SYS, mailbox, BDMA, GPIO/configuration, HWCAPS and
amp-module setup. MI_OSD_Init reaches mi_osd_InitHW -> GFX/GOP initialization.
The direct-call graph for MI_OSD_Init contains no TEEC/tee_client calls, but
Utopia's callback dispatch is not represented by those direct edges.

The owned scanout path uses ordinary MMA allocation/import and DMA-BUF mmap,
then MI OSD surfaces/windows. It does not request secure surfaces or protected
video. Existing hardware confirmation of RGB bars is warm-state evidence:
Android/vendor initialization had already happened. It does not establish that
cold SYS/OSD/GOP/MMA setup is independent of a supplicant.

The stock SYS TEEC wrappers choose kernel tee_client APIs for newer OP-TEE
versions and legacy TEEC APIs otherwise. Thus removing userspace libteec from
our static Rust program is distinct from removing tee-supplicant: a kernel
TEE request can still cause an RPC for a trusted application/file operation
that requires a userspace responder.

## Audio

The audio agent's inspected warm path is:

    /dev/mik/pcm -> MI_PCM_Open -> MApi_AUDIO_PCM_Open
    MI_PCM_Write -> MApi_AUDIO_PCM_Write
    MI_AOUT_ConnectInput -> mi_pipe_ConnectInput

It found no direct OP-TEE call in those normal operations. Existing TV
queue/drain and audible-success evidence follows stock hardware startup.
Cold AUDIO initialization is not cleared by that evidence.

In utpa2k, HAL_AUDSP_DspLoadCode (0x3b8d9c) has an explicit TEE firmware-load
path. At 0x3b9630 it reads g_AudioVars2+0x24f8; values <=1 take the direct
copy path, values >1 reach MDrv_AUDIO_TEE_R_Send_Cmd at 0x3b96f8 (command 3).
The field's provenance/current board value has not been resolved, so do not
assume ordinary audio can cold-initialize without TEE. MDrv_AUDIO_SetCommAudioInfo
also contains a command-dependent TEE call at 0x378248. These are separate
from already-proven PCM writes.

## Other TEE uses that should stay outside the minimal RGB/PCM startup

Direct TEEC call sites in mik occur in MI_VIDEO_Init and secure video/SVP
helpers. utpa2k additionally has conditional XC secure-pipeline, AES/crypto,
DMS secure control, VDEC, demux/PVR and descrambler TEE paths. Their existence
does not prove unprotected OSD presentation takes them. Conversely, a full
vendor daemon init invokes far more than our target path and cannot be used
to infer the minimal dependency set.

The earlier native-PID1 experiment really did require starting stock
tee-supplicant to release a blocked midaemon. Keep that observation: it proves
the broad stock startup dependency, but the exact original blocking subsystem
was not identified in this audit.

## Small hardware gates, coordinated by the parent

1. Keep the current working state. Capture the already-running process stack
   or wait channel before any changes, if a TEE block recurs.
2. For the next *cold* owned initialization, use an isolated copied config:
   SYS/FLASH and explicit DISP/DISPOUT/OSD only, EXTIN off, hdcp2x.ko loaded
   with DISABLE_LOAD_RX_KEY=1 DISABLE_LOAD_TX_KEY=1. No selector-1 all-module initialization. Keep diagnostics
   responsive while an init worker is blocked; a userspace timeout alone does
   not cancel an uninterruptible kernel ioctl.
3. Log before/after every module init. On a TEE wait, capture the exact blocked
   task stack and start the preserved stock supplicant only as the controlled
   recovery/diagnostic action. This identifies the remaining dependency without
   inferring it from a black screen.
4. Verify actual RGB output and audible PCM after that clean boot. A warm
   SIGSTOP experiment can establish no immediate helper activity, but cannot
   prove cold independence or absence of a later TEE RPC.

## Reproducible evidence

Exact modules: assets/research/g06-display-offline/vendor/lib/modules/{xcker,
hdcp1x,hdcp2x}.ko and assets/research/firmware-implications/lib/modules/{mik,
utpa2k}.ko. Configuration: assets/research/g06-display-offline/tvconfig-config/.

Generated offline evidence (shared, ignored by Git):
- assets/build/display-agent/xcker-init.txt
- assets/build/display-agent/hdcp-loadkey.txt
- assets/build/display-agent/hdcp-module-init.txt
- assets/build/display-agent/hdcp1-fillkey.txt
- assets/build/display-agent/utpa2k-full.txt
- assets/build/display-agent/tee-call-sites.txt
- assets/build/display-agent/mi-init-callees.json
- assets/research/midaemon-replacement/selector1-direct-calls.json
- assets/research/audio-agent-direct/{kernel-pcm-open,pcm-device-select,
  pcm-ioctl-dispatch}.txt
- assets/research/midaemon-replacement/{pcm-hwdma-open,pcm-hwdma-set,
  pcm-preopen-kernel}.txt

Direct-call graphs are deliberately overapproximations of whole functions;
branch conditions above were checked in the disassembly. Indirect calls remain
a limitation. No claim of cold hardware sufficiency is made here.
