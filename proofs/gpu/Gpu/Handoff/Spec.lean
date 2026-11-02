/-!
# Renderer-to-presenter frame handoff (a sequence lock): specification

A renderer (the *writer*) publishes frames into a shared record that a
presenter (the *reader*) reads, without locks.  The record holds a
sequence counter and `n` fields.  The protocol:

* **Writer**, to publish one frame: add 1 to the counter (it becomes
  odd: "write in progress"), write the fields one at a time, then add 1
  again (it becomes even: "frame complete").
* **Reader**: read the counter (`c1`), read the fields one at a time,
  read the counter again (`c2`).  Accept the fields only if `c1` is
  even, `c1` is not `0`, and `c1 = c2`.

## The model

The two sides take turns in any order chosen by a *schedule*, a list of
`Actor`s.  Each entry runs one step of the named side.  `run` applies the
whole schedule from the initial state.  The reader does one read attempt;
the writer publishes the frames of its script one after another.

**The one trusted assumption**: each step reads or writes a single
counter or field value atomically, and all steps happen in one global
order (sequential consistency).  This stands in for the acquire/release
atomics of the real code.  Everything else is defined below and is
checked by the proofs in `Gpu.Handoff.Proofs`.

A frame is a function from field index to value; only the indices below
the field count `n` matter.  `fieldsOf n f` lists those `n` values.
-/

namespace Gpu.Handoff

/-- A frame: the value of each field, by field index.  Only indices
below the field count matter. -/
abbrev Frame := Nat → Nat

/-- The frame whose fields are all `0`; the initial content of the record. -/
def zeroFrame : Frame := fun _ => 0

/-- The first `n` fields of a frame, as a list. -/
def fieldsOf (n : Nat) (f : Frame) : List Nat :=
  (List.range n).map f

/-- The shared record: a sequence counter and the field values. -/
structure Shared where
  seq : Nat
  fields : Nat → Nat

/-- Where the writer is: between frames, or writing a frame with field
`next` the next one to write. -/
inductive WriterPhase where
  | idle
  | writing (next : Nat)
deriving Repr, DecidableEq

/-- The writer: its script of frames to publish (in order), how many it
has completed, and its current phase. -/
structure Writer where
  frames : List Frame
  done : Nat
  phase : WriterPhase

/-- Where the reader is: not started; after reading `c1` with the fields
read so far in `got`; or finished with the second counter read `c2`. -/
inductive ReaderPhase where
  | start
  | reading (c1 : Nat) (got : List Nat)
  | finished (c1 : Nat) (got : List Nat) (c2 : Nat)
deriving Repr, DecidableEq

/-- The whole system: field count, shared record, writer, reader. -/
structure System where
  fieldCount : Nat
  shared : Shared
  writer : Writer
  reader : ReaderPhase

/-- The initial state: counter `0`, all fields `0`, writer idle with its
script, reader not started. -/
def init (n : Nat) (frames : List Frame) : System :=
  { fieldCount := n
    shared := ⟨0, zeroFrame⟩
    writer := ⟨frames, 0, .idle⟩
    reader := .start }

/-- One writer step.
* idle, frames left: counter `+1` (now odd), start writing field `0`;
* idle, no frames left: nothing;
* writing field `i < n`: write field `i` of the current frame, move to `i + 1`;
* writing with `i = n` (all fields written): counter `+1` (now even), frame done. -/
def writerStep (s : System) : System :=
  match s.writer.phase with
  | .idle =>
    if s.writer.done < s.writer.frames.length then
      { s with shared := { s.shared with seq := s.shared.seq + 1 }
               writer := { s.writer with phase := .writing 0 } }
    else s
  | .writing i =>
    if i < s.fieldCount then
      let fr := s.writer.frames.getD s.writer.done zeroFrame
      { s with shared := { s.shared with
                           fields := fun j => if j = i then fr i else s.shared.fields j }
               writer := { s.writer with phase := .writing (i + 1) } }
    else
      { s with shared := { s.shared with seq := s.shared.seq + 1 }
               writer := { s.writer with phase := .idle, done := s.writer.done + 1 } }

/-- One reader step.
* not started: read the counter as `c1`;
* reading with fewer than `n` fields read: read the next field;
* reading with all `n` fields read: read the counter as `c2`, finish;
* finished: nothing. -/
def readerStep (s : System) : System :=
  match s.reader with
  | .start => { s with reader := .reading s.shared.seq [] }
  | .reading c1 got =>
    if got.length < s.fieldCount then
      { s with reader := .reading c1 (got ++ [s.shared.fields got.length]) }
    else
      { s with reader := .finished c1 got s.shared.seq }
  | .finished .. => s

/-- Which side takes the next step. -/
inductive Actor where
  | writer
  | reader
deriving Repr, DecidableEq

/-- Run one step of the named side. -/
def step (s : System) (who : Actor) : System :=
  match who with
  | .writer => writerStep s
  | .reader => readerStep s

/-- Run a whole schedule, one step at a time, from state `s`. -/
def run (sched : List Actor) (s : System) : System :=
  sched.foldl step s

/-- The reader's acceptance check: finished, `c1` even, `c1` not `0`, `c1 = c2`. -/
def accepts : ReaderPhase → Bool
  | .finished c1 _ c2 => c1 % 2 = 0 && c1 ≠ 0 && c1 = c2
  | _ => false

/-- A wrong check that skips the second counter read.  Used only to show
why the second read is needed. -/
def acceptsNaive : ReaderPhase → Bool
  | .finished c1 _ _ => c1 % 2 = 0 && c1 ≠ 0
  | _ => false

/-- Which frame of the script an even counter value `c` names: counter
`2 * k` means `k` frames are complete, the newest being script entry `k - 1`. -/
def frameIndex (c : Nat) : Nat := c / 2 - 1

/-! ## Unit tests

Two fields; frame 1 is `[10, 11]`, frame 2 is `[20, 21]`. -/

/-- Script frame 1: fields `10, 11`. -/
def frame1 : Frame := fun j => 10 + j
/-- Script frame 2: fields `20, 21`. -/
def frame2 : Frame := fun j => 20 + j
/-- The example initial state: two fields, two frames to publish. -/
def exInit : System := init 2 [frame1, frame2]

/-- Writer publishes a full frame: begin, field 0, field 1, end. -/
def publishOne : List Actor := [.writer, .writer, .writer, .writer]
/-- Reader does a full read: `c1`, field 0, field 1, `c2`. -/
def readOne : List Actor := [.reader, .reader, .reader, .reader]

#guard fieldsOf 2 frame1 == [10, 11]
#guard fieldsOf 2 frame2 == [20, 21]
#guard fieldsOf 0 frame1 == []
#guard frameIndex 2 == 0
#guard frameIndex 4 == 1

#guard exInit.shared.seq == 0
#guard (run publishOne exInit).shared.seq == 2
#guard (run publishOne exInit).writer.done == 1
#guard fieldsOf 2 (run publishOne exInit).shared.fields == [10, 11]
#guard (run (publishOne ++ publishOne) exInit).shared.seq == 4
#guard fieldsOf 2 (run (publishOne ++ publishOne) exInit).shared.fields == [20, 21]
-- a third publish does nothing: the script is used up
#guard (run (publishOne ++ publishOne ++ publishOne) exInit).shared.seq == 4
#guard (run [.writer] exInit).shared.seq == 1
#guard (run [.writer] exInit).writer.phase == .writing 0

#guard (run [.reader] exInit).reader == .reading 0 []
#guard (run readOne exInit).reader == .finished 0 [0, 0] 0
#guard accepts (run readOne exInit).reader == false        -- counter 0: no frame yet
#guard (run (publishOne ++ readOne) exInit).reader == .finished 2 [10, 11] 2
#guard accepts (run (publishOne ++ readOne) exInit).reader == true
#guard accepts (.finished 2 [10, 11] 3) == false
#guard accepts (.finished 3 [10, 11] 3) == false
#guard accepts (.reading 2 []) == false
#guard acceptsNaive (.finished 2 [10, 21] 3) == true
#guard acceptsNaive (.finished 3 [10, 11] 3) == false

/-- A torn read: the reader reads field 0 of frame 1, then the writer
publishes frame 2, then the reader reads field 1 (now of frame 2). -/
def tornSchedule : List Actor :=
  publishOne ++ [.reader, .reader] ++ [.writer, .writer, .writer] ++ [.reader, .reader]

#guard (run tornSchedule exInit).reader == .finished 2 [10, 21] 3
#guard accepts (run tornSchedule exInit).reader == false
#guard acceptsNaive (run tornSchedule exInit).reader == true

end Gpu.Handoff
