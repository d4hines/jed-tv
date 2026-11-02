# Playing YouTube on the TV from the phone app: Lounge and DIAL (2026-11-01)

Goal (owner): use the stock YouTube phone app to play things on our TV, which
runs our own userspace (Lean PID 1 plus the Lean YouTube app). This directory
holds the research and a working host-side prototype. Nothing here touches the
TV or adb.

## Summary

Ranked by whether they work with today's phone app on an uncertified screen:

1. **"Link with TV code" (YouTube Lounge API only).**
   - The TV shows a 12-digit code and the phone enters it once. After that the
     TV is a linked screen, controlled through YouTube's servers.
   - It needs only outbound HTTPS to www.youtube.com. The phone can be on any
     network, so this works with today's USB-Ethernet-plus-proxy setup.
   - **Verified live here** with a stand-in remote: pairing, play, pause,
     seek, volume and next, 8/8 checks (details below).
   - Widely used: SmartTube, yt-cast-receiver, Playlet.
   - **Build this first.**
2. **DIAL (SSDP discovery on the home LAN), then the same Lounge session.**
   - DIAL only delivers a `pairingCode`. Everything after it is route 1's
     machinery. So DIAL adds to the Lounge client; it doesn't replace it.
   - Others report the stock Android app 21.40.163 lists and launches
     uncertified DIAL receivers (2026-10-12, 2026-10-29).
   - iOS: no first-hand test found.
   - It needs the TV on the phone's LAN: multicast UDP 1900 plus an inbound
     TCP port. Not possible behind the Mac today.
   - The prototype implements it; on this Mac it is tested on loopback only.
     Whether the owner's phone lists it is the open check.
3. **Google Cast (Chromecast protocol): not feasible.** Official senders
   require a Google-signed device certificate (rleroi/cast-dial-notes).

## 1. What works today: evidence

"Verified" means I ran it from this Mac on 2026-11-01 against live YouTube.
Everything else is reported by others; dates are those of the report or commit.

### Verified here (2026-11-01, residential IP)

- Lounge screen endpoints work for a new, unknown screen:
  - `generate_screen_id`
  - `get_lounge_token_batch`: 14-day token
  - `bc/bind` session open
  - `get_pairing_code`: 12-digit code
  - `register_pairing_code`: accepted a UUID code, HTTP 200 with an empty body.
- The full loop through YouTube's relay works, using `fake_remote.py` (a
  pyytlounge-style REMOTE_CONTROL client) as the phone:
  - "TV code" path: `get_screen` with the 12-digit code, then join, then
    setPlaylist, pause, play, seekTo, setVolume, next, getNowPlaying, then
    disconnect. **8/8 checks passed.**
  - The screen received every command and the remote received every state
    reply.
  - Evidence: `assets/research/dial-lounge/e2e-code-20261101T090812Z/`.
- DIAL launch path on loopback works:
  - Sequence: unicast M-SEARCH, then `GET /dd.xml` (Application-URL), then
    `GET /apps/YouTube`, then `POST /apps/YouTube` with
    `pairingCode=<uuid>&theme=cl&dialLaunch=watch`, answered 201 + Location.
  - The screen then calls `register_pairing_code` (200). The remote's
    `get_screen` resolved that code about 2 s later. Then the same 8/8 script.
  - The alternative shape (`--additional-data`: full additionalData with the
    lounge token, plus `deviceInfo` on bind) also passed 8/8. In that shape
    the remote joins without a POST.
  - Evidence: `e2e-dial-loopback-*` under the same assets directory.
- Soak: 6.5 minutes idle with the session open.
  - The server ended the long poll after 4 min 56 s; the reader reconnected
    on the same SID without error.
  - The TV code refreshed at 5 minutes.
  - Evidence: `soak-20261101T091316Z/`.
- **Not verified:**
  - the official YouTube app on a phone (no phone here);
  - multicast SSDP on the home LAN (deliberately not bound by me);
  - iOS.

### Reported by others (with dates)

**DIAL still works on Android, no certification:**

- **SmartTube PR #6324**, rleroi, opened 2026-10-12, applied by the
  maintainer 2026-10-28 as commit e7c6626.
  - Quote: "works with the **official YouTube app** (verified with stock
    YouTube 21.40 on Play services)".
  - The launch body was `pairingCode=...&theme=cl&dialLaunch=watch`. The TV
    registers the code, then gets `remoteConnected` followed by `setPlaylist`.
  - Tested from an Android phone to a Chromecast with Google TV.
  - https://github.com/yuliskov/SmartTube/pull/6324,
    https://github.com/yuliskov/MediaServiceCore/pull/55
- **rleroi/cast-dial-notes** `spec/dial.md`, 2026-10-12: a hardware capture
  from a Mac probe.
  - Quote: "Both Morphe YouTube 21.20.405 … and stock YouTube 21.40.163 with
    official Play Services list the DIAL device in the cast dialog".
  - Phone sequence: M-SEARCH (`MX: 1`), then GET the device description, then
    `GET /apps/YouTube` polled about every 10 s while the cast dialog is open.
  - On selection: `POST /apps/YouTube` with `Origin:
    package:com.google.android.youtube`,
    `Content-Type: text/plain; charset="utf-8"` and body
    `pairingCode=<code>&theme=cl&dialLaunch=browse|watch`.
  - The phone requires "exactly one `Application-URL` response header".
  - Its iOS claim comes from decompiled code, not from a device test.
  - https://github.com/rleroi/cast-dial-notes
- **luimu64/kodi-yt-caster** commit eefaa61, 2026-10-29.
  - Advertising `additionalData` with only a `screenId` made the phone launch
    and then poll forever.
  - Once a `loungeToken` was present, the phone dropped the session with
    `receiverIdentityMatchStatus=DOES_NOT_MATCH_RECEIVER`.
  - Fixed by serving the full set (brand, model, screenId, theme, deviceId,
    loungeToken, loungeTokenRefreshIntervalMs) and sending `deviceInfo`
    (`dialAdditionalDataSupportLevel: supported`) on the bind.
  - Quote: "Verified on tv-vadelma: cast target appears, connects, plays."
    Phone platform not stated.
- **Playlet (Roku)** maintainer, 2024-02-10 (MediaServiceCore issue #10): if
  `additionalData` carries a loungeToken, "the phone will not send a pairing
  code".

**Lounge ("Link with TV code"):**

- **pyytlounge** (remote side) is actively maintained: v3.6.0 on 2026-10-30.
  iSponsorBlockTV uses it daily against real TVs.
- **SmartTube** "Link with TV code" is long-standing. Its MediaServiceCore
  lounge code was last changed 2026-10-28.
- **yt-cast-receiver** 2.1.1 (last commit 2026-03-19) supports manual pairing.

**Changes and possible blocks seen in the Lounge API:**

- `expiration` changed from a number to a string. Reported 2026-02-23
  (ytcast #14), fixed 2026-03-03.
- `loungeScreenDisconnected {"reason":"compromisedReceiver"}` was reported
  for a certified Samsung TV on 2026-10-17 (iSponsorBlockTV #508). The
  official phone app also failed with it. No reports against homebrew screens.
- Random `get_screen` 404s: the maintainer called it a "known issue",
  2026-04-08.
- SmartTube users report degraded code-linked casting in recent phone app
  versions, Aug–Oct 2026 (#6047, #6225). Downgrading the app fixed it for
  one user.

**Not found:**

- any allow-list of certified DIAL devices;
- YouTube's DIAL-registry Origin list (dial-multiscreen.org registry pages
  return 404);
- a 2025–2026 first-hand iOS DIAL test.

**Network caveats people report:**

- Guest Wi-Fi, client isolation and "multicast enhancement" block DIAL; use
  the TV code then (tube2upnp, decky-video-pip, both Oct 2026).

## 2. Protocol: the Lounge screen side (enough to implement it)

All endpoints are on `https://www.youtube.com`. Forms are
`application/x-www-form-urlencoded`. No cookies, API key or login are needed.

### Identity and pairing

| step | request | reply |
|---|---|---|
| screen ID (once, persist it) | `GET /api/lounge/pairing/generate_screen_id` | plain text, 26 chars `[0-9a-z]` |
| lounge token | `POST /api/lounge/pairing/get_lounge_token_batch` with `screen_ids=<id>` | JSON `{"screens":[{screenId, loungeToken (119 chars), loungeTokenLifespanMs 1209600000 (14 days), refreshIntervalInMillis 1123200000, expiration (ms; may be a string), …}]}` |
| TV code | `POST /api/lounge/pairing/get_pairing_code?ctx=pair` with `access_type=permanent&app=<app>&lounge_token=<tok>&screen_id=<id>&screen_name=<name>&device_id=<uuid>` | plain text, 12 digits; show as `XXX XXX XXX XXX`; refresh every 5 min while unpaired |
| DIAL code | `POST /api/lounge/pairing/register_pairing_code` with `access_type=permanent&app&pairing_code=<code from DIAL POST>&screen_id&screen_name&device_id` | 200, empty body |
| (remote side) | `POST /api/lounge/pairing/get_screen` with `pairing_code=<code>` | JSON `{"screen":{screenId, loungeToken, name, …}}`; 404 until registered |

- `device_id` is a random UUID kept with the screen ID.
- `app`:
  - The prototype uses `lb-v4`, as SmartTube does and as cast-dial-notes saw
    the receiver register. It is the official TV app's label.
  - yt-cast-receiver uses `ytcr`. Either is accepted.
- Use `theme=cl` for YouTube; YouTube Music uses `m`.

### Bind channel (BrowserChannel, `VER=8`)

Common query parameters on every `/api/lounge/bc/bind` request:

- `device=LOUNGE_SCREEN`
- `id=<device_id>`
- `name=<screen name>`
- `app=<app>`
- `theme=cl`
- `capabilities=dsp,mic,dpa`
- `mdxVersion=2`
- `loungeIdToken=<tok>`
- `VER=8`, `v=2`, `t=1`
- `zx=<12 random [a-z0-9]>`

Optionally add `deviceInfo=<JSON>`, `cst=m` and `obfuscatedGaiaId=` (needed
for the DIAL additionalData shape, see below).

1. **Open:** `POST bind?…&RID=<random 5 digits>&CVER=1`, body `count=0`.
   - Reply: one chunk holding:
     - `[0,["c","<SID>","",8]]`
     - `[1,["S","<gsessionid>"]]`
     - `[2,["loungeStatus",{…}]]`
     - `[3,["getNowPlaying"]]`
     - `[4,["getDiscoveryDeviceId"]]`
   - Keep SID, gsessionid and the highest array ID (AID).
2. **Back channel (long poll):**
   `GET bind?…&RID=rpc&SID=<SID>&gsessionid=<gs>&CI=0&AID=<last AID>&TYPE=xmlhttp`.
   - The response stays open and streams chunks.
   - `[n,["noop"]]` arrives about every 30 s (measured). Treat more than 60 s
     of silence as a dead connection.
   - The server ends the response after about 5 minutes (measured: 4 min
     56 s). Reconnect with the same SID and gsessionid and the updated AID
     (verified in the soak run).
3. **Forward channel (state/replies):**
   `POST bind?…&SID&gsessionid&RID=<previous RID + 1>&AID=<last AID>`.
   - Body: `count=<k>&ofs=<maps sent so far on this session>`, then for each
     map i, `req<i>__sc=<name>` (two underscores) and
     `req<i>_<field>=<value>`.
   - Reply: `7\n[1,13,0]`. Ignore it.
4. **Errors:**
   - `400 Unknown SID` or `410 Gone`: open a new session (step 1).
   - `401`: get a new lounge token, then step 1.
   - Back off on network errors.
5. **On exit:** send `loungeScreenDisconnected`, then
   `POST bind?…&SID…&RID…` with body `TYPE=terminate&ui=`.

**Framing:**

- The response is `<decimal length>\n<JSON array>\n`, repeated. The length
  counts everything after its own newline, including the trailing newline.
- In every observed message the server sends **pure ASCII**: non-ASCII is
  escaped as `\uXXXX`, including surrogate pairs. A screen name
  "Dande TV – prüfung 📺" came back as `– prüfung 📺`.
  So bytes and characters agree. The JSON decoder must handle surrogate pairs.
- Arrays can span several lines. The prototype splits on string-aware bracket
  matching and only cross-checks the length.
- Each entry is `[arrayId, [name, payload?]]`.
- Payload values are almost all **strings**, including numbers: `"state":"1"`,
  `"currentTime":"0"`. Some fields are JSON-in-a-string (`devices`,
  `eventDetails`, `device`).

### Incoming commands (seen live, plus those the reference screens handle)

| name | payload (observed keys) | screen action / reply |
|---|---|---|
| `loungeStatus` | `devices` (JSON string; list with `type` LOUNGE_SCREEN / REMOTE_CONTROL, `name`, `id`, `app`, `capabilities`, `receiverIdentityMatchStatus`), `queueId`, `connectionEventDetails`, `cachedSessionState` | track connected remotes; reply `onHasPreviousNextChanged`, `nowPlaying`, `onAutoplayModeChanged` |
| `remoteConnected` / `remoteDisconnected` | the remote's device record (`name`, `id`, `app`, `theme`, `capabilities`, `ui`) | reply full state on connect |
| `getNowPlaying` | none | `nowPlaying` |
| `getDiscoveryDeviceId` | none | `setDiscoveryDeviceId` with `discoveryDeviceId` and `loungeDeviceId`, both our device_id. Field names from SmartTube's CommandParams. The server answers `onSetDiscoveryDeviceId`. Use the same UUID as the DIAL UDN. |
| `setPlaylist` | `videoId`, `currentTime`, `currentIndex`, `videoIds` (comma list), `listId` (= queueId `RQ…`), `eventDetails`, `sourceDeviceType`; from real phones also `ctt`, `params` | **play `videoId` from `currentTime`**; reply `nowPlaying` + `onStateChange` |
| `setVideo` | `videoId`, `currentTime` (older senders) | same |
| `updatePlaylist` | `videoIds`, `listId` | update queue; `nowPlaying` |
| `play` / `pause` / `stopVideo` | none | `onStateChange` |
| `seekTo` | `newTime` (s) | `onStateChange` |
| `next` / `previous` | none | load the next/previous item in `videoIds` |
| `setVolume` | `volume` (0–100), `muted` | `onVolumeChanged` |
| `getVolume`, `getSubtitlesTrack` | none | `onVolumeChanged`, `onSubtitlesTrackChanged` |
| `setAutoplayMode` | `autoplayMode` | `onAutoplayModeChanged` (we say `UNSUPPORTED`) |
| `getPartyGamesMode`, `dpadCommand`, `voiceCommand`, `setSubtitlesTrack`, `onUserActivity` | various | ignore for now |
| `noop` | none | keepalive |

### State the screen reports (forward-channel maps)

- **`nowPlaying`:**
  - Fields: `videoId`, `currentTime`, `state`, `duration`, `loadedTime`,
    `seekableStartTime`, `seekableEndTime`, `cpn` (any 16-char token),
    `listId`, `currentIndex`, `ctt`, `params`.
  - Send it empty when idle.
- **`onStateChange`:** `state`, `currentTime`, `duration`, `loadedTime`,
  `seekableStartTime`, `seekableEndTime`, `cpn`.
- **`onVolumeChanged`:** `volume`, `muted`.
- **`onHasPreviousNextChanged`:** `hasPrevious`, `hasNext`.
- **`onAutoplayModeChanged`:** `autoplayMode`.
- **`state` values:** the YouTube player numbers that real TVs report
  (pyytlounge): -1 unstarted, 0 ended, 1 playing, 2 paused, 3 buffering,
  5 cued. Our flow: setPlaylist, then 3, then 1 once loaded.
- Send on changes, not periodically; the phone interpolates the time.

## 3. Protocol: the DIAL server side

As used by the stock app in the 2026-10-12 capture; the reply shape follows
SmartTube's DialServer.

- **SSDP**:
  - The phone multicasts `M-SEARCH * HTTP/1.1` to 239.255.255.250:1900 with
    `MAN: "ssdp:discover"`, `MX: 1` and
    `ST: urn:dial-multiscreen-org:service:dial:1`.
  - Reply by unicast to the sender's address and port, within MX:
    ```
    HTTP/1.1 200 OK
    CACHE-CONTROL: max-age=1800
    EXT:
    LOCATION: http://<lan ip>:<port>/dd.xml
    SERVER: Linux/4.19 UPnP/1.1 dande-tv/0
    ST: urn:dial-multiscreen-org:service:dial:1
    USN: uuid:<udn>::urn:dial-multiscreen-org:service:dial:1
    BOOTID.UPNP.ORG: 1
    ```
  - NOTIFY alive/byebye is optional; SmartTube sends none.
- **Device description:**
  - `GET /dd.xml` returns a UPnP `root`
    (`urn:schemas-upnp-org:device-1-0`) containing `deviceType`
    `urn:dial-multiscreen-org:device:dial:1`, `friendlyName` (shown in the
    cast list), `manufacturer`, `modelName` and `UDN uuid:<udn>`.
  - Response header: **exactly one** `Application-URL: http://<ip>:<port>/apps/`.
- **App status:** `GET /apps/YouTube`, polled about every 10 s while the cast
  dialog is open. Two shapes are known to work:
  - **Default (SmartTube):**
    ```xml
    <service xmlns="urn:dial-multiscreen-org:schemas:dial" dialVer="1.7">
      <name>YouTube</name><options allowStop="false"/><state>stopped|running</state>
    </service>
    ```
    No additionalData, so the phone POSTs a pairingCode.
  - **Alternative (kodi-yt-caster):** always `running`, `dialVer="2.1"`, with
    `<link rel="run" href="run"/>`.
    - `<additionalData>` holds brand, model, screenId, theme, deviceId,
      loungeToken and loungeTokenRefreshIntervalMs.
    - The bind must then carry `deviceInfo` with
      `dialAdditionalDataSupportLevel: supported`.
    - The phone joins directly with the token.
    - **Never advertise a partial set** (screenId only): that is the
      DOES_NOT_MATCH_RECEIVER failure.
- **Launch:** `POST /apps/YouTube`.
  - Headers: `Origin: package:com.google.android.youtube`,
    `Content-Type: text/plain; charset="utf-8"`.
  - Body: `pairingCode=<code>&theme=cl&dialLaunch=watch|browse`. Older
    senders also sent `v=<videoId>&t=<s>` (plaincast ignores them).
    `theme=m` means YouTube Music.
  - Answer `201 Created` with
    `Location: http://<ip>:<port>/apps/YouTube/run` (200 if already running).
    Then register the code (section 2) as soon as possible; the phone polls
    `get_screen`.
  - Accept any Origin; log it.
- **DELETE** `/apps/YouTube/run`: stop. Harmless to accept.

**How DIAL and Lounge connect:** the pairingCode in the DIAL POST is generated
by the phone. The screen attaches it to its own screen ID via
`register_pairing_code`. The phone resolves it with `get_screen` to the screen
ID and lounge token, then joins the lounge. From then on it is identical to
the TV-code path. Live timing on loopback: register 0.1 s after the POST,
first successful `get_screen` about 2 s after. The SmartTube capture measured
`remoteConnected` +2.3 s and `setPlaylist` +3.3 s from the tap.

## 4. The prototype (this directory; Python standard library only, no packages)

| file | what |
|---|---|
| `lounge.py` | Lounge client library: pairing endpoints, `BindSession` (open / long poll / send / terminate), `ChunkParser` |
| `screen.py` | the screen: persistent identity, TV code, command printer, simulated player; `--dial` adds the DIAL server |
| `dial.py` | SSDP responder + HTTP DIAL server; logs every request (all headers, launch body) |
| `fake_remote.py` | phone stand-in for self-tests (`--code` or `--dial`); 8 checks |

- State lives in `assets/research/dial-lounge/screen-state.json` (screen ID
  and device ID; keep it, or paired phones forget the TV).
- Each run writes `assets/research/dial-lounge/runs/<UTC>/transcript.jsonl`
  with every request, raw chunk and command. Lounge tokens are shortened in
  the transcript.

### Run it

```
cd research/dial-lounge
python3 screen.py                  # TV code only (works on any network)
python3 screen.py --dial           # plus DIAL on the Mac's Wi-Fi address
python3 screen.py --dial --additional-data   # alternative DIAL shape
python3 screen.py --dial --ip 127.0.0.1      # loopback only (what I tested)
```

- `--dial` binds UDP 1900 (shared, `SO_REUSEPORT`) and TCP 56790 on the
  default-route address only (here en0). It refuses 192.168.77.x (the TV
  link).
- Port 8090 is already used by the TV proxy on this Mac, hence 56790.
- The sockets exist only while the process runs. Ctrl-C sends byebye,
  closes them, and tells YouTube the screen left.
- The macOS firewall is on, so expect a prompt "accept incoming network
  connections" for Python: click **Allow**. Without it the phone cannot reach
  the DIAL HTTP port.
- Other flags:
  - `--name` sets the name on the phone.
  - `--state <file>` gives a fresh identity.
  - `--seconds N` exits by itself after N seconds.

### Manual test for the owner (needs the phone)

**A. TV code (the main route):**

1. `python3 screen.py`. Note the code in the box, e.g. `076 706 486 798`.
2. On the phone: YouTube > You (profile) > Settings (gear) > **Watch on TV** >
   **Enter TV code** (wording varies by app version). Type the 12 digits and
   tap Link.
   - Expect `<- remoteConnected …` and `REMOTE CONNECTED: <phone name>`.
3. Open any video on the phone, tap the Cast icon, and pick
   "Dande TV (prototype)".
   - Expect `<- setPlaylist {…}` and then
     `>>> PLAY VIDEO <videoId> from <t>s https://youtu.be/<videoId>`.
   - The ID should match the phone's video (Share > Copy link).
   - The phone should show the video as playing on the TV.
   - The duration is a fixed simulated 10:00, so the phone's bar may not match
     the real length.
4. Pause on the phone: expect `>>> PAUSE at Ns`. Play: `>>> RESUME at Ns`.
   Drag the seek bar: `>>> SEEK to Ns`.
5. Volume keys or slider while casting: `>>> VOLUME n`. The phone may not send
   this for every app version.
6. Disconnect in the cast menu: `remote disconnected`. Then Ctrl-C.

If step 2 is rejected or step 3 never shows the TV, keep the run's transcript
directory (the path is printed at the top).

**B. DIAL (is the TV listed without a code?):**

1. The phone must be on the same Wi-Fi as the Mac (not a guest network).
   - On iOS, check Settings > Privacy > Local Network > YouTube.
2. Use a fresh identity, so a linked screen from test A doesn't confuse the
   result: `python3 screen.py --dial --state /tmp/dial-test.json --name "Dande DIAL test"`.
   Allow the firewall prompt.
3. Open YouTube on the phone and tap the Cast icon.
   - Expect `SSDP M-SEARCH from <phone ip> … ST=urn:dial-multiscreen-org:service:dial:1`.
   - Then `DIAL GET /dd.xml`, then `DIAL GET /apps/YouTube` repeating while
     the dialog is open.
   - **Is "Dande DIAL test" in the list?** That is the main result. Please
     note Android or iOS and the app version.
4. Select it.
   - Expect `DIAL POST /apps/YouTube … Origin=package:com.google.android.youtube`.
   - Then `DIAL LAUNCH body: pairingCode=…&theme=cl&dialLaunch=…`, then
     `register_pairing_code -> HTTP 200`, then `REMOTE CONNECTED`, then
     commands as in A.
5. If it is listed but never connects, repeat with `--additional-data`.
6. If you see no M-SEARCH at all, the firewall, the network or the iOS
   permission is blocking it.

## 5. Fit with our design (issue 12)

### Where the parts go

- **YouTube app: the Lounge client.** It owns the screen identity (screen ID,
  device ID, lounge token), the bind session, the TV-code UI and playback.
  - It needs only outbound HTTPS to `www.youtube.com:443`, a host already in
    its table.
  - Concurrency: one long-lived long poll plus forward-channel POSTs, i.e. two
    connections (reuse keep-alive for the forward channel).
  - It fits the poll-driven `Yt.Net.Conn` + `Yt.Tls` + `Yt.Http` stack. The
    long poll is just another fd in the loop. The parser is about 50 lines
    (length line plus bracket scan).
  - Today it goes through the existing CONNECT tunnel; later it uses a fd
    from the PID 1 broker.
  - It needs no new syscalls: everything is read/write/poll on sockets it
    already has.
- **PID 1: DIAL.** DIAL exists to *launch* an app that isn't running, which
  is the launcher's job.
  - PID 1 answers SSDP and HTTP, and keeps the "YouTube running?" state (it
    knows it already).
  - On `POST /apps/YouTube` it validates the body and either launches YouTube
    or forwards to the running instance. YouTube then registers the code with
    its own screen identity.
  - With the default DIAL shape, PID 1 holds **no YouTube secrets**: no token
    and no screen ID. That is a reason to prefer it over the additionalData
    shape, which would need the app to push its lounge token to PID 1.
  - PID 1 should generate the per-install UUID, use it as the DIAL UDN, and
    pass it to YouTube as the lounge `device_id`. Then `setDiscoveryDeviceId`
    matches the DIAL entry.
- **Persistence:** screen ID and device ID must survive reboots, or every
  phone has to re-link. That needs a small per-app persistent store from
  PID 1 (an opaque blob of a few hundred bytes), or PID 1 stores the UUID and
  the app stores the screen ID in its workdir if that persists.
- **Castable only while the app runs (stage A).**
  - On the TV-code route, the TV is online in the phone's list only while
    some process holds a bind session.
  - The simplest choice, consistent with "Home provably goes home" (Home
    kills the app), is to keep the session in the YouTube app. Then the TV
    is castable while YouTube is open.
  - DIAL (stage B) restores "cast when nothing is open" on the LAN by having
    PID 1 launch the app.
  - A headless background lounge service would also do it without a LAN, but
    adds a third process class. I would not start there.

### Capabilities and seccomp

| who | needs | notes |
|---|---|---|
| YouTube app | outbound TCP/TLS to www.youtube.com (2 long-lived connections) | already allowed (`TV.Apps.network 1`); unchanged once the broker hands out fds for `(www.youtube.com, 443)` |
| PID 1 (DIAL) | UDP socket bound to `0.0.0.0:1900` with `SO_REUSEADDR` and `IP_ADD_MEMBERSHIP 239.255.255.250` on the LAN interface; `recvfrom`/`sendto`; one TCP listen socket on `<lan ip>:<port>` (`listen`/`accept`) | PID 1 is unconfined (uid 0 gets allow-all in `TV.Sandbox`), so no filter change. The real cost is that PID 1 parses untrusted LAN input |
| optional DIAL helper | inherited pre-bound fds only: `recvfrom`/`sendto`/`accept4`/`read`/`write`/`poll`/`close`; own UID; no `socket`/`bind`/`connect` | puts the LAN parsing outside PID 1; PID 1 gets only a validated "launch" message |

- **Sandbox observations:**
  - The current network allowlist (calls 55–66) already lets network apps
    `bind`, `setsockopt` and `recvfrom`. So the YouTube app could today bind
    UDP 1900 and join the multicast group. It cannot `listen`/`accept`
    (284/285/366 are absent).
  - The Lounge client needs none of `bind`, `getsockname`, `recvfrom` or
    `sendto`. They can go when the stage-4 broker lands, leaving
    read/write/poll/close/recvmsg on given fds.
- **Hardening for DIAL code wherever it runs:**
  - caps: 2 KiB UDP datagram, 8 KiB headers, 16 KiB body (SmartTube's cap);
  - a 10 s per-connection deadline and a handful of concurrent connections;
  - exactly three routes: `GET /dd.xml`, `GET|POST /apps/YouTube`,
    `DELETE /apps/YouTube/run`;
  - accept the pairing code only if it matches `[A-Za-z0-9_-]{1,64}`.
  - These all make good Lean lemmas (total parser, bounded buffers, "a DIAL
    POST can launch only app 1", "never while YouTube is disabled by the
    PIN", "Home still preempts").

### Network placement

- **TV code:** none beyond today's. The phone can even be on cellular.
- **DIAL:** the TV must be on the phone's broadcast/multicast domain. It must
  receive multicast on 239.255.255.250:1900 and accept inbound TCP from the
  phone. Options:
  1. The TV on the home LAN itself (its own Ethernet or Wi-Fi under our
     userspace).
  2. For development, a DIAL relay on the Mac. `screen.py --dial`'s server,
     instead of registering the code itself, would forward the POST body over
     the USB link to the TV's PID 1. The Mac answers SSDP on Wi-Fi; the TV
     stays on 192.168.77.0/24.

  macOS cannot easily bridge Wi-Fi to the USB link. Client isolation, guest
  networks and IGMP snooping without a querier all break SSDP.

### Proposed PID 1 ↔ YouTube message protocol

- **Don't overload the existing input socket.** fd 3 is a SOCK_DGRAM pair
  carrying 16-byte `struct input_event`s. Existing apps (Doom via libplatform)
  read fixed 16-byte datagrams, so new types would confuse them.
- Instead, give apps that hold a `cast` capability a second socketpair:
  - `AF_UNIX`, `SOCK_SEQPACKET`, both directions;
  - announced as `tvapp.control=4` / `TVAPP_CONTROL=4`.

Every message has an 8-byte header, `magic "TVC1"`, `u16 type`, `u16 len`,
then `len` payload bytes (max 1024).

| dir | type | payload | meaning |
|---|---|---|---|
| PID 1 → app | 1 `CAST_LAUNCH` | `pairingCode=<code>&theme=<cl\|m>&dialLaunch=<watch\|browse>[&v=<videoId>&t=<s>]`, re-encoded by PID 1 from validated fields only | a phone asked over DIAL; register the code (`register_pairing_code`) and make sure the bind session is open |
| PID 1 → app | 2 `CAST_IDENTITY` | `deviceId=<uuid>` | sent once at launch; use as lounge `device_id` (equal to the DIAL UDN) |
| app → PID 1 | 3 `CAST_STATE` | `online=0\|1` | optional: the lounge session is up; PID 1 can answer DIAL `running`/`stopped` honestly |

- **Launch flow:**
  - App not running: PID 1 replies `201 Created` with Location, launches app
    1, then sends `CAST_IDENTITY` and `CAST_LAUNCH` into the pair before the
    child reads it. SEQPACKET buffers, so the order is kept.
  - App running: reply `200` and send `CAST_LAUNCH`.
  - Launch not allowed (YouTube disabled by PIN, or policy says don't
    preempt the current app): reply `403` and send nothing.
- With argv/env instead of the socket, the pairing code would be readable in
  `/proc/<pid>/cmdline`. It is a short-lived bearer secret for the phone's
  session, so keep it on the socket.

## 6. Risks

- **YouTube changes or gates the Lounge for unknown screens.** Precedents
  already exist:
  - the `compromisedReceiver` disconnect reason (seen on a certified TV,
    2026-10-17);
  - `receiverIdentityMatchStatus` checks (seen 2026-10-29);
  - the `expiration` type change (2026-02).
  - Using `app=lb-v4` (the official TV app's label) could become a liability
    if it gets attested; `ytcr` is the fallback.
- **The phone app regresses for linked screens:** SmartTube reports,
  Aug–Oct 2026. Our live check used a pyytlounge-style remote, not the
  official app.
- **DIAL:** works on Android per reports from 2026-10. iOS is unverified. It
  depends on the home network (multicast, isolation) and, on iOS, the Local
  Network permission. No authentication: anyone on the LAN can make the TV
  launch YouTube.
- **PID 1 attack surface** if DIAL parsing runs inside it (see the helper
  option).
- **Terms of service:** this is an unofficial screen; the same caveat as the
  VISIONOS player client.
- **Prototype limits:**
  - playback is simulated with a fixed 600 s duration;
  - token refresh happens only on 401 (14-day tokens);
  - the soak run was about 6.5 minutes.
