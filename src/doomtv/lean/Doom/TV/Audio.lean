import Doom.Audio.Sound
import Doom.TV.Platform

/-! The PCM producer owns the sound system independently of rendering.
Only immutable sound commands cross the atomic mailbox. A slow video frame
must not prevent replenishing an 85 ms audio queue. -/
namespace Doom.TV.Audio

abbrev Update := Doom.Audio.Sound.System → Doom.Audio.Sound.System

structure Mailbox where
  updates : Array Update := #[]
  stopping : Bool := false

structure Worker where
  mailbox : IO.Ref Mailbox
  task : Task (Except IO.Error Unit)

partial def produce (mailbox : IO.Ref Mailbox) (sound : Doom.Audio.Sound.System) : IO Unit := do
  let mail ← mailbox.modifyGet fun m => (m, { m with updates := #[] })
  if mail.stopping then return
  let sound := mail.updates.foldl (fun s update => update s) sound
  let queued ← Doom.TV.Platform.audioQueued
  let q := if queued == 0xffffffff then 0 else queued.toNat
  let target := 4096
  if q + 512 > target then
    IO.sleep 2
    produce mailbox sound
  else
    let frames := (target - q) / 512 * 512
    let (sound, pcm) := sound.render frames
    let result ← Doom.TV.Platform.audioWrite pcm
    if result != 0 then
      IO.eprintln s!"audio: write failed ({result}); sound off"
      return
    produce mailbox sound

def Worker.start (sound : Doom.Audio.Sound.System) : IO Worker := do
  let mailbox ← IO.mkRef { : Mailbox }
  let task ← IO.asTask (do
    try produce mailbox sound
    finally mailbox.modify fun m => { m with stopping := true, updates := #[] }) .dedicated
  return { mailbox, task }

def Worker.submit (worker : Worker) (updates : Array Update) : IO Unit := do
  if !updates.isEmpty then
    worker.mailbox.modify fun m =>
      if m.stopping then m else { m with updates := m.updates ++ updates }

def Worker.stop (worker : Worker) : IO Unit := do
  worker.mailbox.modify fun m => { m with stopping := true }
  match worker.task.get with
  | .ok () => pure ()
  | .error e => IO.eprintln s!"audio: worker stopped: {e}"

end Doom.TV.Audio
