# City Flight on the original G06

2026-11-01: the City Flight port from Jack Hall (main commit 93b2d3b,
registered as app 6 by the merged policy) rendered on the G06/V109.
The tested binary SHA256 was
`63e77502870bbee48847585ac0ff19c52fffb36335bac8b26111d96ba1c08cce`.

A bounded 120-second root-console test used `TV_FB_PATH=/tmp/app.frames`
and the existing PID1-owned display service. The running older PID1 did
not yet include the City Flight menu entry. Its safety guard correctly
refused replacing the supervisor while it owned media resources. No guard
was bypassed and no flash/reboot was performed. The test stopped the menu,
ran the game, and restored the menu automatically. This was a diagnostic
launch, not a test of City Flight's production app confinement.

The world (2,145 buildings) initialized in 4.5 seconds; the Lean simulation
and six-escape safety filter initialized in 3.88 seconds. The game logged
roughly 4.4-5.1 frames/second and 3-4 percent of real-time simulation with
the safety filter enabled. A captured 1280x720 frame was fully opaque and
showed the city, aircraft and HUD. Physical screen/control confirmation
was requested separately; it is not established by the frame capture.

Evidence: `assets/research/city-flight-live/`, with the captured image at
`assets/build/city-flight-live.png`. The original PID1 build remained
`90c15507804e8bd42267a3146eca324f07bb3e2fccef115ba2fdf903a04787dc`.

Local native build/upload scripts now include flighttv and its menu art.
The native build passes. A new launcher containing app 6 is still needed
for normal menu launch on this running TV; that requires the next safe
startup rather than hot-replacing a supervisor owning display resources.
