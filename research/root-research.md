# Root and factory-debug research, 2026-10-30

**Later live result:** stock-service uid0 command execution and raw firmware
readback now work. The missing step was factory process registration using the
stock signed token and matching process name. See [ROOT_READBACK_NOTES.txt](ROOT_READBACK_NOTES.txt)
for the validated procedure, evidence, SELinux limits and backup location.
The earlier observations below describe the investigation before that result.

Target: owner-provided TCL 32Q3K, G06 / MT5867 / MT9221, Android 11, kernel 4.19.116+, V8-T221T04-LF1V109 AR33. This agent performed web research and offline analysis only; live observations below were collected by the parent agent.

## Concrete findings from this TV

- The extracted `FactorySettings.apk` implements a real factory menu. `assets/research/factory-decompiled/sources/com/tcl/factory/def/DEFS.java:35` defines its password as `1950`.
- `MainMenu` accepts integer extra `select`; default 0 shows the password dialog. `select=1` selects the design menu, `2` the production menu, `3` also enables warm-up, `4` service menu, `5` hotel menu, `6` after-sales menu, `7` recording menu. Avoid experimenting with these values: some have side effects.
- The normal design menu has `Other` at index 2. `DModeOtherMenu.java` contains `UartEnable` at index 1. Its display reads `FactoryManager.doGetUartStatus()`, and left/right invokes `doOpenSerialPort()` or `doCloseSerialPort()`. The APK proves the control exists on this firmware, beyond analogy with other TVs.
- `ThirdParty.executeShell(command)` calls `FactoryMenuAdapter.rootSeeker(command)`, which invokes `FactoryManager.setInformation(126, command)`. The constant is confusingly named `HdcpKey20`; it is 126 in `TVDefine.java:60`. Existing usages include `sync`, copying a boot-animation file, and reading eMMC information into `/tmp/fac-EMMC.txt`. This establishes a privileged command-dispatch feature, but not that shell UID2000 may call it. `FactoryManager` is in `com.tcl.tvmanager.jar`; its wrappers lead to `com.tcl.tosapi.atv.TFactoryApi.native_doSetInfo(126, command)`. No Java permission check was found in this path. The native implementation and access controls remain unresolved. Likely JNI library names, referenced by other classes in the jar, are `libcom_tcl_tv_jni.so` and `libcom_tcl_tv_jni.tcl.so`.
- Separate `FactoryManager.doShellExecute(string)` calls operate named commands such as NVM initialization; they must not be assumed to be ordinary shell execution.
- `assets/research/device/init/vendor/hw/init.mt5867.usb.rc` explicitly changes the USB role: `sys.usb.config=adb` runs `/vendor/bin/usb_otg_agent device`; `none` or `mtp` selects host. This is actual firmware support for a USB device role. It does not identify the physical port or prove bootloader Fastboot connectivity.
- The root-owned `factory_svc` runs `/vendor/factory/bin/factory_svc` in SELinux `factory_service`; `tcl_sitatvservice` runs `/vendor/tvos/bin/run.sh` as root in its own SELinux domain. Merely being a factory service or listening on Binder does not make it callable from ADB shell.

## Related-model evidence

The closest public report is a TCL 32S5400, also G06/MT5867, Android 11, 32-bit ARM, kernel 4.19.116. Its owner reports Magisk 25.2 success, initially via MBOOT and later Fastboot. UART is enabled in factory menu Other/UartEnable and operated at 115200 baud. The report describes reading partitions into RAM and exporting to a USB drive through MBOOT, which could provide stock boot/vbmeta backups if the corresponding console is available here. RAM addresses and sizes are unit-specific. The author explicitly limits the report to the tested unit; the downloadable patched boot lacks a verified original fingerprint. Use this report to identify mechanisms, not as a compatible image source. [Owner report](https://knowledge-storage.info/2026/10/22/tcl-32s5400-tvt-mt5867magisk-root/) and [image repository](https://github.com/Long0823-1/TCL-G06-Magisk-Boot).

Another author reports root on TCL 75P8K and 55C6K through a service UART at an HDMI connector. This uses MT5896/merak and Android 14, so its wiring and boot commands are not transferable without independent board evidence. The project's own record says reliable UART receive was not established, reducing its value as a console-debugging reference. [Original toolkit rooting notes](https://github.com/lukasa1993/tcl-tv-toolkit/blob/main/docs/rooting.md).

Older TCL/Thomson S69 research describes `sitatvservice` and an accessible boot UART, but on a non-Android Linux 3.x system. Its possible service-level remote root route was a hypothesis, not a demonstrated exploit. [Original research](https://github.com/prototux/TCL-TV-reverse-engineering/blob/master/README.md).

## Mali and other non-flashing routes

The published TCL 50C721 Mali root chain targets a Realtek platform, Linux 4.14.198+, and V8-R51MT02-LF1V903. It relies on `/dev/rtkmem`, Realtek service behavior, and fixed library/kernel offsets. The target here lacks `/dev/rtkmem`; the full chain is not directly applicable. Its own compatibility document says GPU identity or an old driver string is insufficient and contains inconsistent r26/r28 revision records. [Project](https://gitlab.com/c4psu13/mali-pagecache-root/-/blob/main/README.md), [compatibility limits](https://gitlab.com/c4psu13/mali-pagecache-root/-/blob/main/docs/COMPATIBLE-DEVICES.md).

The target's observed GLES string is `Mali-G31 ... v1.r25p1-01bet0.Bifrost-20201221.52e1f09`. This is the userspace driver string, not proof of kernel-driver patch status. CVE-2022-22706 nominally covers Bifrost r0p0-r35p0; vendors can backport fixes. A useful follow-up is obtaining the running kernel's Mali module or matching firmware for static inspection. Current shell attempts to read module files and sysfs driver version were denied. [CVE record](https://cveawg.mitre.org/api/cve/CVE-2022-22706), [Android bulletin](https://source.android.com/docs/security/bulletin/2023-06-01).

MediaTek's MT9221 advisories include IOMMU CVE-2023-20811 and TVAPI CVE-2024-20001/20002. The documented attacks require System execution privileges, so these are not established direct routes from shell UID2000. No compatible working shell-to-root chain was found. [MediaTek August 2023](https://www.mediatek.com/product-security-bulletin/August-2023), [MediaTek February 2024](https://www.mediatek.com/product-security-bulletin/February-2024).

TCL publishes an MT9221 4.19.116 source tree. The inspected recursive tree contains MStar platform code and GPU-related glue, but no complete Mali kbase driver tree. Kernel version matching alone does not prove this source matches the running binary or all vendor modules. [TCL source index](https://github.com/TCLOpenSource/TCL_Kernel_OpenSource), [MT9221 source](https://github.com/TCLOpenSource/mt9221).

## Next inspections

1. Pull and inspect `libcom_tcl_tv_jni.so` / `libcom_tcl_tv_jni.tcl.so` if readable; Java wrappers are in `assets/research/tvmanager-decompiled`. Locate JNI registration for `TFactoryApi.native_doSetInfo`.
2. Inspect Binder or JNI dispatch and authorization around `setInformation(126,...)`; if legitimately callable, the initial probe should only report `id`/SELinux status to a temporary file. No probe has been performed by this agent. The parent reports the factory service name is visible in `service list`, but shell access to that service was denied; direct Binder availability is therefore not established.
3. Read factory menu UartEnable status and boot-code version. Establish the physical service connector from this board's manual/schematic before wiring UART.
4. Obtain stock boot/vbmeta through an exact recovery package, a supported bootloader readback, or a demonstrated privileged backup path before image mutation.

For kernel development after root, the actual extracted config has `CONFIG_KEXEC=y`, `CONFIG_MODULES=y`, and no `CONFIG_MODULE_SIG`. Those are useful capabilities, subject to SELinux/capability checks and matching ABI; they do not grant root by themselves. `CONFIG_DEVMEM` is disabled.

## Native API tracing and prepared read-only helper

Both JNI variants and `libsitatv.so` were successfully pulled from `/system_ext/lib` by the parent. Native registration maps `TFactoryApi.native_doSetInfo` to Thumb function 0x1abc0 in `libcom_tcl_tv_jni.so`. Its `info=126` branch invokes `tos_factory_set_system_info(26, string, strlen(string))`. That function in `libsitatv.so` obtains service `sbinder_factory` through the library's custom `sita_android` Binder implementation and sends transaction 74: interface token, int32 type, int32 byte length, and a Parcel blob. Its driver path is `/dev/sbinder`, not ordinary `/dev/binder`. Thus inability to find the Android service `com.tcl.factory.service` does not settle access to this API. Parent reports `/dev/sbinder` and `/dev/tshmem` are both mode0666; actual open/ioctl permission still needs checking.

The native getters were independently traced: UART status calls `tos_factory_get_func_enable(0, &byte)`, and boot-code version calls `tos_factory_get_version(2, buffer, 64)`. Prepared `FactoryReadOnly.java` / `factory-readonly.jar` only invokes these two getters, accepts no arguments, and does not invoke shell execution or settings writes. The parent ran it under `app_process32`; it returned `doGetUartStatus=false` and `doGetBootCodeVersion=V8-S9221T01-LB1V001`. Logs confirm root `sitatvservice` received the UART getter (transaction29) and warned that the client PID was `UNREGISTERED`. The false UART value is therefore not reliable proof that the menu toggle is off. The helper initializes JNI and middleware client state by loading stock `TvPlayerApi`, as the stock manager does; observed log output shows client proxy/mqueue initialization.

Prepared `FactoryIdentityProbe.java` / `factory-identity-probe.jar` at parent request. It accepts no arguments and only dispatches the fixed command `/system/bin/id > /data/local/tmp/tcl-root-probe.txt` through selector126. This agent did not execute the probe. The factory app's registration path is `TvFunctionApi.systemAuthorized(String)` -> `tos_system_authorized(0,key,strlen(key))`; MainMenu uses a fixed on-device TSA_KEY. Client authorization checks occur server-side and remain to be assessed.

Saved native evidence: `native-setinfo-disassembly.txt`, `native-factory-set-system-info-disassembly.txt`, and `native-factory-symbols.txt`. No assumption about server authorization or the backend meaning of selector26 should be promoted to confirmed root until the owner device returns credentials.
