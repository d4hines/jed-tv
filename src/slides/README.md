# Slides

The slideshow is a Lean application behind App API v1. `Slides/Parse.lean`
parses the deck, `Render.lean` lays it out and produces every pixel, and
`Navigation.lean` handles remote keys. The 60-line native shim only copies the
completed BGRA frame into the leased display buffer and transports input events.
The single-threaded event loop runs Lean main on the original thread and disables its unused task workers at link time to
honor the Slides sandbox's one-thread limit. The launcher owns process confinement and Home; Back/Exit cleanly close the
handles and return to it. There is no audio or network dependency.

The visual language is shared with tvinit: public-domain font8x8 BASIC_LEGACY,
integer pixel scaling, dark blue gradient, white/grey type, red accents and square
panels. Body type is 24 pixels, or 16 for dense slides; titles are up to 40 pixels.
The original hackathon deck is unchanged. Its historical claims are not updated
by the port.

```
python3 test/slides/check.py
tools/slides/build.sh armv7-unknown-linux-musleabihf
```

Only what runs on the TV is here; the checks (`Check.lean`, `Preview.lean`, the
scripts, `testdata/`) are in test/slides and the build (`build.sh`, the asset
step `prepare.py`) in tools/slides. The first command checks parsing, layout, navigation, alpha compositing, all 12
slides and the image fixture. It writes PPM previews under
`assets/build/lean-slides/verification`. `python3 test/slides/check-qemu.py` tests the installed app under the real launcher
in headless ARM32 QEMU, including Back/Home and exact host-versus-ARM frame hashes.
This uses the virt32 surrogate kernel; it does not prove real panel initialization.
The build command writes the static TV binary
under `assets/build/lean-slides/armv7-unknown-linux-musleabihf/slides`; set
`PLATFORM_LIB` and `LEAN_ARM` to reuse existing cross-built libraries.
`tools/native-app/build.sh` and `demo/package.nix` install it as `/bin/slides`.

`prepare.py` embeds the Markdown verbatim and converts referenced image assets
with host ffmpeg into bounded RGBA (maximum 1088x420, never enlarged). Its generated
Lean module lives in `assets/build/lean-slides/generated`; there is no Markdown
parser or image decoder in this Python build helper. Lean validates the embedded
deck before the cross build. Build products in `.lake` follow the other Lean apps.

Markdown: `---` separates slides; one initial `# Title`, `## Subtitle`, paragraphs,
`-` and numbered lists (two spaces per nesting level, at most four levels), fenced
code, standalone `![caption](relative/path.png)`, and non-rendered `Note:` lines.
Inline backticks/double backticks, bold, italic, combined bold/italic and escapes
are supported. Adjacent paragraph lines join; item continuation lines join until
a blank line. Malformed input returns line-numbered errors. Pixel typography
covers ASCII; unsupported glyphs use `?`, as in the bitmap renderer. Images retain
color and alpha and are aspect-fitted; code wraps without dropping characters.
The build refuses a deck whose content exceeds the usable height even at the
smaller body size.

Remote: Right/Down/OK/Next/PlayPause advance, Left/Up/Previous go back, digits jump
to a page (combine two digits within 1.5 seconds), and Back/Exit return home.
`slides.start=N` starts at a one-based page. Run it under tvinit; it is an app,
not a standalone PID 1 responsible for mounting devices.
