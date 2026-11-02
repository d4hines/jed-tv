# Stage 0: the minimal YouTube protocol (verified 2026-10-31)

Measured from a residential IP on 2026-10-31 ~20:00Z against two Blender open
movies (Coffee Run `PVGeM40dABA`, Big Buck Bunny 4K `aqz-KE-bpKQ`). Evidence is
under `assets/yt-stage0/` (git-ignored): `census-*`, `record-*`, `minimal-*`.
YouTube changes often; rerun `minimal_probe.py` before trusting this.

## Result

Anonymous playback at any offered resolution (up to 2160p was verified) needs
**two JSON POSTs and plain ranged GETs**. All of these were absent and still
worked: no watch page, cookies, API key, JavaScript engine, proof-of-origin
(PO) token, SABR or login.

## The sequence

Client identity: YouTube's `VISIONOS` client (copied from yt-dlp 2026.10.26).

```
1. POST https://www.youtube.com/youtubei/v1/player?prettyPrint=false
   Content-Type: application/json
   {"context":{"client":{"clientName":"VISIONOS","clientVersion":"1.02",
      "deviceMake":"Apple","deviceModel":"RealityDevice17,1",
      "osName":"visionOS","osVersion":"26.5.23O471","hl":"en",
      "userAgent":"Mozilla/5.0 (Macintosh; Intel Mac OS X 15_7_3) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.0 Safari/605.1.15"}},
    "videoId":"<id>","contentCheckOk":true,"racyCheckOk":true}
   -> 200, playabilityStatus.status = LOGIN_REQUIRED ("confirm you're not a bot")
      but responseContext.visitorData = "<V>"

2. Same POST, plus the visitor ID, either as header `X-Goog-Visitor-Id: <V>`
   or as context.client.visitorData = "<V>" (both verified separately)
   -> 200, playabilityStatus.status = OK, streamingData.adaptiveFormats[]
      every entry has a direct `url` (no signatureCipher, no `n` parameter)

3. GET <format.url>   with only `Range: bytes=a-b`   (no other headers needed)
   -> 206 Partial Content, media bytes
```

Verified unnecessary in step 2: the `X-YouTube-Client-Name/Version` headers.
Step 1 needs no headers at all (urllib's default User-Agent worked). The
replies were uncompressed because no `Accept-Encoding` was sent, so gzip is optional.

## What the reply gives us

`streamingData.expiresInSeconds` = 21540 (~6 h). URLs embed `ip=` and are
bound to the requesting public IP. Each `adaptiveFormats[]` entry has:
`itag, url, mimeType (with codecs), width, height, fps, bitrate,
contentLength, initRange, indexRange, approxDurationMs` (+ audio fields
`audioSampleRate, audioChannels, loudnessDb`). `initRange` is the container
header (MP4 `moov` / WebM header); `indexRange` is the seek index (MP4 `sidx` /
WebM `Cues`), so seeking needs no manifest. `videoDetails` gives title, author,
length, thumbnails.

## Target: 720p max, MP4 only

Team decision 2026-10-31: no more than 720p. Every video has H.264 up to that
height, and H.264/AAC both arrive in MP4, so the player needs only:

| role | itags | container | codec |
|---|---|---|---|
| video | 136 (720p), 135 (480p), 134 (360p), 133 (240p), 160 (144p) | MP4 | H.264 (`avc1`) |
| audio | 140 | MP4 | AAC-LC 44.1 kHz stereo (~130 kbps) |
| later option | 398/397/396 (AV1 720/480/360p) | MP4 | AV1: memory-safe Rust decoder (rav1d) |

Coffee Run is 2.39:1, so its "720p" is 1280x536.

## Client census (yt-dlp 2026.10.26 + Deno, both videos agreed)

| client | result |
|---|---|
| visionos | full ladder to 2160p (AV1/VP9) and 1080p (H.264); no JS, no PO token |
| tv_samsung, web_embedded | full ladder, but need the JS challenge solved (QuickJS-NG would do); no PO token |
| tv, tv_downgraded | broken today ("The page needs to be reloaded") |
| web, web_safari, ios | adaptive formats SABR-only or need PO token |
| mweb, android, android_vr, tv_simply | 360p muxed only; adaptive needs PO token |

Fallback plan if visionos is cut off: tv_samsung/web_embedded plus a small JS
engine for the challenge. If anonymous access closes entirely: login (a
throwaway account) via cookies on a cookie-capable client; visionos does not
take cookies.

## Platform API implied (QEMU track)

DNS, TCP, TLS client (HTTP/1.1 is enough), a JSON parser, ranged GET. No JS
engine, no browser, no Google libraries. For media: an MP4 (fragmented,
`sidx`) demuxer, H.264 + AAC decoders, A/V sync.

## Caveats and open items

- Through mitmproxy, the media GET got 403; direct GETs (Python urllib/OpenSSL,
  yt-dlp) worked. googlevideo may react to how the TLS connection looks.
  Test rustls against the real servers early.
- Two videos, one IP, one afternoon. Not tested: age-restricted, members-only,
  live, music-label content, other regions, how long a visitor ID lasts,
  rate limits, sustained-download throttling (the ~270 MB of fixtures
  downloaded quickly via yt-dlp's 10 MB ranged chunks).
- `serverAbrStreamingUrl` (SABR) is also offered but not needed.

## Tools (research/yt-stage0/)

- `census.py`: matrix of player clients x videos via pinned yt-dlp.
- `record.py`: proxied capture (HAR) + byte-exact fixture downloads
  (`--fixup never`; ffmpeg remux would alter the bytes).
- `har_summary.py`: readable transcript of a HAR.
- `minimal_probe.py`: stdlib-only reproduction of the sequence above.
- `replay.py`: offline fake YouTube with the same two-step behavior, for
  QEMU and CI. Never run tests against real YouTube from eduardo-vm.

Replay example (22 Coffee Run fixtures, all lengths checked against the reply):

```
python3 test/replay.py \
  --player assets/yt-stage0/minimal-PVGeM40dABA-visionos-20261031T200757Z/player.visitor-header.json \
  --fixtures assets/yt-stage0/record-PVGeM40dABA-visionos-20261031T200633Z/fixtures
```
