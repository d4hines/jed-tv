#!/usr/bin/env python3
"""Embed source text and bounded RGBA assets; Markdown semantics remain in Lean."""
import json
import pathlib
import re
import subprocess
import sys
root = pathlib.Path(__file__).resolve().parent
source = pathlib.Path(sys.argv[1]).resolve() if len(sys.argv) > 1 else root / '../../src/slides/hackathon.md'
output = pathlib.Path(sys.argv[2]) if len(sys.argv) > 2 else root / '../../assets/build/lean-slides/generated/SlideAssets.lean'
text = source.read_text()
images = []
for path in dict.fromkeys(re.findall(r'^!\[[^\]]*\]\(([^)]+)\)\s*$', text, re.M)):
    data = subprocess.check_output(['ffmpeg','-nostdin','-v','error','-i',str(source.parent/path),'-frames:v','1','-vf',"scale=w='min(1088,iw)':h='min(420,ih)':force_original_aspect_ratio=decrease",'-pix_fmt','rgba','-f','image2pipe','-c:v','pam','-'])
    header, pixels = data.split(b'ENDHDR\n', 1)
    fields = dict(line.split(b' ',1) for line in header.splitlines()[1:] if b' ' in line)
    w,h = int(fields[b'WIDTH']),int(fields[b'HEIGHT'])
    assert 0 < w <= 1088 and 0 < h <= 420 and len(pixels) == w*h*4
    images.append('⟨'+json.dumps(path)+f', {w}, {h}, decodeHex '+json.dumps(pixels.hex())+'⟩')
output.parent.mkdir(parents=True, exist_ok=True)
output.write_text('import Slides.Deck\nset_option maxRecDepth 100000\nset_option maxHeartbeats 0\nnamespace Slides\ndef source : String := '+json.dumps(text,ensure_ascii=False)+'\ndef images : Array Image := #['+','.join(images)+']\nend Slides\n')
