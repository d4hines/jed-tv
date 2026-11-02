# TCL 32Q3K bootloader and physical debug research

Research date: 2026-10-30. No TV commands were run by this research agent.

## Findings that directly affect the next experiment

1. The official Q2K/Q3K US quick-start guide shows **one USB-A 2.0 port**, at the top of the side connector stack, immediately above LAN. It also labels the upper HDMI socket **HDMI 1 (SERVICE)** and the lower one HDMI 2 (ARC). The diagram is printed page 9, PDF page 13. Downloaded and visually inspected: `assets/research/manuals/Q2K-Q3K-US-QSG.pdf`, rendered page `assets/research/manuals/Q3K-ports.png`.
2. The main agent found a live `Mstar-udc`, `udc-msb250x`, USB state `adb`, and the boot argument `USB_OTG_SOFTWARE_ID=device`. The TV's init configuration calls `usb_otg_agent device` when ADB is enabled. These observations support an actual USB device-mode experiment on the external USB socket.
3. TCL's published MT9221 USB driver sets peripheral mode from a software ID and directly enables the data-line pull-up. Its peripheral state transition does not require a VBUS-valid input. A short USB connection with only D+, D−, and GND, with VBUS isolated, is therefore a reasonable first experiment. It is **not** a guarantee for the exact running binary or board. No enumeration would remain inconclusive.
4. A related MT9221 schematic gives a candidate service-HDMI UART connection: TV TX on HDMI pin 14, TV RX on pin 2, GND on pin 17. This is **related-board evidence**, not an exact 32Q3K schematic. A passive RX-only observation after voltage verification is the appropriate first test; do not connect an adapter's VCC to the HDMI port.

Official sources:

- [TCL Q3K support/downloads](https://support.tcl.com/en_US/2025-google-tv/q3k-series)
- [Official QSG PDF](https://cdn.shopify.com/s/files/1/0687/7151/2385/files/J0104-000587_R1_Q2KQ3K_US-QSG.pdf?v=1776925417)
- [Official user guide PDF](https://cdn.shopify.com/s/files/1/0687/7151/2385/files/Q2K-Q3K_US.pdf?v=1776914081)

## Published kernel source and the isolated-VBUS USB experiment

Downloaded source files are in `assets/research/manuals`; the directory name reflects this research pass, not that they are manuals. Upstream source revision was fetched from `master`; exact compatibility with the live firmware has not been established.

- [phy-ms-usb.c](https://github.com/TCLOpenSource/mt9221/blob/master/drivers/usb/phy/phy-ms-usb.c): lines 112–138 parse `USB_OTG_SOFTWARE_ID=device`; lines 1358–1378 choose software ID 1 for device mode. Lines 869–885 move from B_IDLE to B_PERIPHERAL based only on ID, without a VBUS-presence gate. Lines 962–967 enable and start the peripheral.
- [msb250x_udc.c](https://github.com/TCLOpenSource/mt9221/blob/master/drivers/usb/gadget/udc/msb250x_udc.c): lines 1437–1449 directly control soft connection/pull-up; lines 1516–1517 leave VBUS session/draw callbacks commented out. `software_connect` is a readable state attribute in lines 1595–1624.
- `phy-ms-usb.c` does contain GPIO VBUS-power control. Its presence does not prove the physical TV port stops sourcing 5 V in every mode. The related schematic also includes optional/bypassed power-switch components. Do not infer safe direct 5 V rail joining from the source.

Suggested breakout connection: USB pin 2 D− to D−, pin 3 D+ to D+, pin 4 GND to GND; pin 1 VBUS open between the devices. Keep the differential pair short. USB TTL UART hardware does not carry USB protocol; use the laptop's ordinary USB host for this experiment.

## Related-board HDMI UART evidence

[TCL 40-MT21T7 schematic mirror](https://es.scribd.com/document/957426398/40-MT21T7-MPB2HG) contains drawing `9221_TCL Design_Schematic_Ver.A_20220324`, title `40-MT21T7-MPA2HG`, sheet 4 HDMI interface. Its heading is HDMI1 (CEC & UART), connector PH30. Short extracted labels:

```text
GND1 2 UART-RX
NC2 14 UART-TX
GND5 17
```

The UART TX net connects through RH48 (470 ohms) to UART0_TX. RX goes via RH47 (1.2 kilohms), RX1, and RH51 (220 ohms) to UART0_RX. Internal HDMI signal names on this connector use HDMI2 even though the external connector section is HDMI1. The second external HDMI section is CEC/ARC.

This evidence was read from the schematic's publicly exposed text; **the actual schematic image has not been visually verified** because available downloads require subscriptions. The official Q3K connector image was visually verified. Public source locators for independent review: web tool reference `turn45view0`, extracted lines 1421–1501. The downloaded public HTML is `assets/research/manuals/MT21T7-schematic-page.html`; it is a page shell, not a downloaded schematic PDF.

The accompanying [MT21TB7 service manual mirror](https://www.scribd.com/document/957426498/Manual-Service-MT21TB7-AG) has an important quality limitation: its alignment prose mixes MT9221 headings with Realtek RT2841/RTICE instructions. Do not use its generic claim of a 5 V serial interface as evidence of this TV's UART logic voltage. It documents an Other → UartEnable setting and 115200/8/N/1 serial parsing, consistent with owner observations below.

The USB section of the related schematic maps the external socket's D+/D− to SoC port 0; Wi-Fi and Bluetooth use separate USB ports. This reinforces that the external USB socket is the plausible gadget port. Exact 32Q3K board identity remains unknown.

## Same-platform owner report: TCL 32S5400, G06 / MT5867

[Owner's root research](https://knowledge-storage.info/2026/10/18/tcl-android-tv32s5400-root-bondriver/) reports a 32-bit Android 11 G06 device, UART at 115200, MBOOT prompt `MT5867 >#`, and enabling UART via Contrast → 1950 → Other → UartEnable. The article describes successful root with 32-bit Magisk 25.2 after failures with other builds. Its October 27 update reports Fastboot flashing after unlocking and vbmeta handling, but does not identify the USB cable/port. It reports USB recovery and partition backups via MBOOT. These remain owner reports for another regional TV.

Its board photo was downloaded and visually inspected: `assets/research/manuals/32S5400-UART-board.png`. It shows underside pads TX, RX, RX1 near an HDMI footprint labeled HDMI2. This fits the related schematic's net/pad names but does not identify our exact board.

[Companion Magisk image repository](https://github.com/Long0823-1/TCL-G06-Magisk-Boot) explicitly limits the published image to the tested environment. It does not establish the original firmware fingerprint/build from the image. Its newer Fastboot success section describes successful normal boot, and explicitly does not present a captured `su -c id` result for that particular report. This image should not be treated as compatible with T221T04 V109 merely because both TVs say G06.

## Other TCL owner reports and applicability

- [TCL 55E17NUS owner report, 2018](https://devcoster.com/blog/): explicitly reports a USB-A-to-A cable working in bootloader mode after failing as Android USB ADB. The author says that bootloader lacked wireless connectivity. Related retail names include U55P6046 and L55P2MUS. This establishes a TCL precedent, not 32Q3K support.
- [TCL 65P7K owner report](https://www.reddit.com/r/androidroot/comments/1mhzfoq/rooting_a_tcl_65p7k_tv/): explicitly reports USB-A-to-A Fastboot and successful bootloader unlocking. The device is Amlogic TC8000, Android 12, kernel 5.15; its boot images and recovery procedures are not applicable to MT9221. The article includes failures after flashing and stock-image recovery, making it useful as a physical connection precedent only.
- A [4PDA S6500 thread](https://4pda.to/forum/index.php?showtopic=948168&st=6880) search excerpt also describes USB-A-to-A Fastboot on 32S65A. Full page fetch was denied, so it is not used as independently inspected proof.

## Other brands and related platforms

The TCL-authored MT9221 training document, [public mirror](https://es.scribd.com/document/614895767/Training-material-of-MT9221-Chassis-Outsourcing-S52), covers 32–43S65 and S5200 boards across MT21K/MT21X/MT21T variants. It labels a service HDMI connector on its older board, but external numbering differs from Q3K. This confirms that shared SoC does not imply identical connector numbering.

Repair-community board listings associate MT21TB7/40-MT21T7-MPB2HG with TCL L40/L43S5400, RCA R40/R43AND, and Hitachi CDH-LE40/43SMART23. Those listings are leads to matching manufacturer documents, not evidence to flash their images onto this TV. Their T221T05 packages differ from our T221T04.

Other search leads mention MT5867 in Xiaomi A2 and Asano TP.SK516S boards. No inspected primary evidence tied those bootloaders or images to this TCL firmware; no procedure from those sources is recommended here.

## Fastboot over Ethernet

[AOSP's Fastboot protocol](https://android.googlesource.com/platform/system/core/+/refs/heads/main/fastboot/README.md) supports USB and TCP/UDP transports. A device must implement the network server and be IP-reachable. Android's Ethernet address and ADB daemon disappearing during a bootloader reboot does not establish such support. No inspected source confirms network Fastboot for this G06 MBOOT. The stronger concrete avenues are physical USB or UART.

## Unresolved before root/flashing

- Whether this exact firmware's bootloader enumerates in USB Fastboot.
- Whether unlocking is permitted and what data it erases.
- Exact original boot/vbmeta/recovery images or a verified backup path.
- Exact 32Q3K recovery image/procedure.
- HDMI UART logic voltage and confirmation that candidate pin 14 carries readable boot output.

Prefer read-only Fastboot variables once connected, or UART boot-log capture and MBOOT help/printenv before writes. Do not extrapolate a specific firmware image, RAM address, partition size, or factory certificate bypass from a different TV.
