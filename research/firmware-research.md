# TCL 32Q3K firmware and kernel research — 2026-10-30

**Later live result:** V109 stock-service root execution now provides a verified
raw readback route. Original boot/recovery/vbmeta/MBOOT images have been saved
under `../assets/backups/v109-stock-20261031`; consult its manifest for the full backup
status. See [ROOT_READBACK_NOTES.txt](ROOT_READBACK_NOTES.txt). An update to V122
is no longer needed to obtain a matching stock boot image.

## Result

No verified download of **V8-T221T04-LF1V109** or matching factory recovery package was located. However, the actual TV updater protocol was recovered and a laptop-only request to the official production endpoint returned a **full V8-T221T04-LF1V122 OTA**. Selected images were extracted by HTTP Range reads for offline analysis. Nothing was written to the TV. V122 is not the installed V109 and is not a verified factory recovery path.

The local baseline identifies `G06_2K_US`, Android 11, firmware `V8-T221T04-LF1V109.000732`, build increment `AR33` (display ID `AR2101`), and Linux `4.19.116+`, built 2025-01-18 with Android clang 11.0.1. Parent investigation additionally obtained non-unique model `TCL-NA-MT9221-02`, project ID `17849`, model `32Q3K`, and panel `ST3151B07-1`.

## Closest firmware relatives

* A [32S350G owner's original report](https://www.reddit.com/r/tcltvs/comments/13xypzz) gives `V8-T221T04-LF1V071.000464`, Android 11, Linux `4.19.116+`, and `AR2101`. This establishes a same-platform US predecessor and useful search target; it does not establish identical panel configuration or recoverability.
* A [43Q31K owner's report](https://www.reddit.com/r/tcltvs/comments/1p0wrz2/looks_like_many_models_will_get_android_tv_14/) gives `V8-T221T04-LF1V117.000789`, project ID `17850`. A [later owner reply](https://www.reddit.com/r/tcltvs/comments/1qgtz3e/how_to_get_android_14_tcl_update/) gives V118. These confirm that T221T04 persists in Q31K; neither thread supplies a verified recovery image.
* The [CTA annual TV report](https://www.cta.tech/media/a5yd0h4t/annual-tv_2024-report-final.pdf) lists the 43S350G with `V8-T221T04-LF1V081`.
* T221T01, T221T02, T221T05, and newer 0003T05/0003T06 packages were found, but there is no evidence they are replacements for **T221T04**. They should not be used for this US TV merely because the SoC is related.

## Closest root and backup account

The [32S5400 owner's October 22 account](https://knowledge-storage.info/2026/10/22/tcl-32s5400-tvt-mt5867magisk-root/) documents G06_2K_GB, MT5867, Android 11, Linux 4.19.116, and 32-bit ARM. Its MBOOT console runs at 115200 baud. Factory menu entry is Contrast → 1950; Other → UartEnable enables serial interaction. The owner uses MBOOT partition reads and FAT USB writes to preserve stock partitions, then patches the owner's stock boot with 32-bit Magisk 25.2. Newer Magisk failed on that device. October 27 additions report a Fastboot route as well. This is a near-match account, not a test on our T221T04 build.

The owner's [boot release repository](https://github.com/Long0823-1/TCL-G06-Magisk-Boot) explicitly says the published 32,505,856-byte boot is already patched and its exact original firmware fingerprint is unknown. It includes vbmeta backups, not a factory recovery image. The release cannot replace a stock backup from this unit. Its linked TWRP repository returned 404 during this check.

The original account's read-only backup pattern is `mmc read.p <RAM address> <partition> <size>` followed by `fatwrite usb 0:1 <RAM address> <8.3 filename> <size>`. Addresses, partition names, and sizes must come from this TV's MBOOT memory map and partition table before use. No MBOOT command was executed here.

## Official kernel source

TCL's [official index](https://github.com/TCLOpenSource/TCL_Kernel_OpenSource) maps Android R / Linux 4.19.116 / MT9221 to [TCLOpenSource/mt9221](https://github.com/TCLOpenSource/mt9221), listing S65A, S615, S5205, S5200, F53, S60A and S65AH. It does not explicitly list 32Q3K or V109.

* Snapshot: [`098aa173622fa59a5531b9b01d8f9fc3b0e539f8`](https://github.com/TCLOpenSource/mt9221/commit/098aa173622fa59a5531b9b01d8f9fc3b0e539f8), initial commit 2022-04-19.
* [`Makefile`](https://github.com/TCLOpenSource/mt9221/blob/master/Makefile) declares 4.19.116.
* [`BUILD.md`](https://github.com/TCLOpenSource/mt9221/blob/master/BUILD.md) specifies `.config_mt5867_SMP_arm_android_emmc_nand_utopia2k_iommu` and `ARCH=arm`, confirming the MT9221/MT5867 source relationship.
* The referenced configuration was absent from the inspected root and `arch/arm/configs` listings. The live kernel config, now saved in `assets/research/device/kernel.config`, is more relevant than guessing the missing sample configuration.
* The tree contains `arch/arm/boot/dts/mt5867.dtsi` and MT5867 overlays for TCL C06 EU, Hisense huashan NA/EM/SA, Changhong DVB92, Haier ATVT56DVB, JVC MSD9216, HKC, Skyworth SW5, and Expressluck/Caixun/Sansui. These establish a broader vendor BSP family, not firmware interchangeability or root compatibility.
* The 2022 source predates the running 2025 kernel build. Exact V109 source correspondence, module ABI, boot configuration, panel support, and build completeness remain unverified.

## Official manuals

The [TCL Q3K downloads page](https://support.tcl.com/en_US/q3k-series) provides a [manufacturer Q2K/Q3K user manual](https://cdn.shopify.com/s/files/1/0687/7151/2385/files/Q2K-Q3K_US.pdf?v=1776914081), quick-start guide, measurements, and energy guide. No V109 firmware archive is exposed on that page. Consumer documentation does not establish bootloader recovery or an OTG/service cable pinout.

## OTA discovery work

The actual pulled `assets/research/device/apks/TTVSCore.apk` is 33,391,267 bytes and has four dex files. All dex/config ASCII strings were saved to `assets/research/device/ttvs-all-strings.txt`; looking only at classes.dex misses its updater implementation.

Confirmed strings in the actual TV APK:

* `com.tcl.ttvs.versionUpdateApp.utils.protocl.ProtocolAddrChanger`
* `com.tcl.ttvs.versionUpdateApp.utils.protocl.RequestsXmlUtils`
* `/service/upmp/upgradeIncrInterface`
* `/service/upmp/operateInterface`
* `HUAN-AppId`, `HUAN-RequestId`, `HUAN-Sign`, `HUAN-Timestamp`
* `upmp-na`, `upmp-eu`, `upmp-as`, `upmp-cn`, `upmp-test`
* `/data/fota/` and `/tmp_versionUpdate.zip`

No exact V109 download URL is embedded as plain ASCII. The regional host was Base64-encoded, and was recovered by decompiling ProtocolAddrChanger. Decompiled sources saved in `assets/research/device/`: `ProtocolAddrChanger.java`, `ProtocolConstTools.java`, `RequestsXmlUtils.java`, `SystemInterceptor.java`, `UpdateDigest.java`, and `UpdateByVersion.java`.

The production endpoint is `https://na-filter-upgrade.huan.tv/service/upmp/upgradeIncrInterface`. `query-public-firmware.py` reconstructs the updater XML and request signature, uses the non-unique model/project plus explicit dummy device serial/token, and performs only a metadata request. No real device identifiers or accounts are sent. The endpoint returned HTTP 200 / state `0000`.

Official response saved in `assets/research/firmware-query-result.json`:

| Field | Value |
| --- | --- |
| Firmware | V8-T221T04-LF1V122 |
| Title | TCL-NA-MT9221-02-Amati |
| Version ID | 4231 |
| Full package size | 1,713,801,569 bytes |
| Server MD5 | 44d35a21204ab54691a439d1b941061e |
| Update time | 2026-07-15 16:00:00 |
| Incremental | 0 (full package) |

The server supplies a time-limited signed download URL on `na-global-auth-tclupdate.cedock.com`. Run the metadata script again when it expires. The package's full MD5 was **not verified**, because the full 1.7GB archive was not downloaded.

`inspect-firmware-zip.py` uses bounded HTTP Range requests and validates ZIP CRCs while extracting selected members. First pass downloaded 8,667,458 bytes and extracted boot/dtbo/vbmeta and metadata. Second pass downloaded 12,876,202 bytes and extracted mboot.bin plus upgrade.img. Saved under `assets/research/firmware/V8-T221T04-LF1V122/` with `contents.json`, `range-reads.json`, and locally computed `SHA256SUMS`:

| Member | Extracted bytes |
| --- | ---: |
| boot.img | 32,505,856 |
| dtbo.img | 8,388,608 |
| vbmeta.img | 12,288 |
| vbmeta_system.img | 4,096 |
| mboot.bin | 2,825,143 |
| upgrade.img | 209,715,200 |

The V122 OTA metadata gives `TCL/C06/C06:11/AR2101/AR38:user/release-keys`, security patch `2026-03-05`, and build time 2026-03-22. Its updater-script asserts **C06 in `/oem/oem.prop`**, despite the live Android G06 product name. Parent should check live OEM properties to resolve this before treating the package as install-compatible. The update writes MBOOT, eMMC boot0, OP-TEE, boot, dtb and many other partitions; it is not a boot-only update. `upgrade.img` appears in the archive but is not directly referenced by the extracted updater-script, so its recovery purpose is still an inference.

The alternative `getUpgradeByVerID` endpoint is driven by an `md5id` service/PIN value, not a plain firmware version string. No V109 service code was established.

A community [firmware tracker implementation](https://github.com/FaserF/TCL-Discussion-Telegram/tree/main/scripts/firmware_tracker) describes `na-filter-upgrade.huan.tv` and `na-update.cedock.com`, but its protocol reference disagrees with its live implementation and `build_channel_release()` synthesizes fallback SHA256/CRC values from version strings. Treat this as a source of leads only; its listed checksums and broad compatibility tables are not trustworthy firmware evidence.

Four conservative HEAD probes to inferred `https://na-update.cedock.com/apps/resource2/V8T221T04/...` paths for V109 OTA/recovery and known V114/V117 suffixes all timed out after 20 seconds. Two follow-up HTTP probes for V109 with/without `.000732` also timed out. This demonstrates no usable connection from this laptop, not that those files exist or are absent. No large download resulted from those probes.

## Next evidence needed

### Post-recovery V122 assessment

After recovery and ADB reauthorization, a fresh metadata request still returned
V8-T221T04-LF1V122 for model TCL-NA-MT9221-02 / project 17849 / current V109,
using dummy unique identifiers. The response is saved in
`firmware-v122-recheck.json`: full package, 1,713,801,569 bytes, MD5
`44d35a21204ab54691a439d1b941061e`.

Live `/oem/oem.prop` now confirms `ro.product.name=C06`,
`ro.product.device=C06`, and `ro.product.brand=TCL`, matching all three
identity assertions in V122's updater-script. Extracted
`assets/research/firmware/V8-T221T04-LF1V122/32Q3K-hardware.json` explicitly identifies 32Q3K,
MT9221, RC813, and the ST3151B07-1 panel. This resolves the earlier C06/G06
identity question; it does not establish that an update has succeeded.

Installing this official release is a plausible way to reach a firmware
version whose stock boot/vbmeta files are obtainable. The complete archive
still needs downloading and integrity/signature validation before installation.
It is not a backup of V109 or the unit's unique calibration/certificate data.
The OTA script also writes MBOOT and eMMC boot0. V122's inspected bootloader
retains the USB unlock handler, but actual lock state, USB access and rollback
after an update remain untested. No update was initiated during this assessment.

### Remaining backup work

1. Obtain working Fastboot transport or UART/MBOOT access and inspect supported commands.
2. Back up this unit's boot, vbmeta, recovery, dtbo, partition map, bootloader environment and unique calibration partitions through a verified read path. Preserve unique identifiers locally.
3. Independently establish a matching T221T04 recovery package or demonstrate restoration from the backups before committing a modified boot image.
4. For kernel development, retain the live config and dtbo along with the source snapshot; root alone does not establish custom-kernel boot capability.
