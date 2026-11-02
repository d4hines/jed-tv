prelude
import Init.Data.UInt

-- Do not lift scalar constants into once-cells requiring runtime initialization.
set_option compiler.extract_closed false

/- Executable child-supervisor policy. Rust supplies Linux observations and
   executes the returned commands; time and deadlines are monotonic nanoseconds.
   Phases: 0 spawning, 1 running, 2 TERM sent, 3 KILL sent, 4 done, 5 failed.
   Events: 0 spawned, 1 Home, 2 Power, 3 tick, 4 child reaped, 5 wait failed.
   Commands: 0 none, 1 TERM, 2 KILL, 3 power off, 4 done, 5 wait failure.
   Scalar exports keep the ARM32 boundary independent of the Lean runtime. -/
namespace TV.Supervisor

@[inline] def result (phase command : UInt32) : UInt64 :=
  phase.toUInt64 ||| (command.toUInt64 <<< 32)

@[export tv_supervisor_step]
def step (phase : UInt32) (deadline now : UInt64) (event repeated : UInt32) : UInt64 :=
  if phase == 4 || phase == 5 then result phase 0
  else if event == 4 then result 4 4
  else if event == 5 then result 5 5
  else if event == 0 && phase == 0 then result 1 0
  else if event == 2 && repeated == 0 then result phase 3
  else if event == 1 && repeated == 0 && phase == 1 then result 2 1
  else if event == 3 && phase == 2 && now > deadline then result 3 2
  else result phase 0

@[export tv_supervisor_deadline]
def nextDeadline (phase : UInt32) (deadline now : UInt64) (event repeated : UInt32) : UInt64 :=
  let r := step phase deadline now event repeated
  if (r >>> 32) == 1 then
    let grace : UInt64 := 1500000000
    if now > 0xffffffffffffffff - grace then 0xffffffffffffffff else now + grace
  else if r.toUInt32 == 2 then deadline else 0

-- Fork results: 0 child/exec, 1 parent/supervise, 2 failure.
@[export tv_supervisor_fork]
def forkResult (pid : UInt32) : UInt32 :=
  if pid == 0 then 0 else if (pid &&& 0x80000000) != 0 then 2 else 1

-- Wait results: 0 poll input, 1 target reaped, 2 retry (orphan or EINTR), 3 fail.
@[export tv_supervisor_wait]
def waitResult (child observed error : UInt32) : UInt32 :=
  if observed == child then 1
  else if observed == 0 then 0
  else if (observed &&& 0x80000000) == 0 || error == 4 then 2 else 3

-- Status kinds: 0 exited, 1 stopped by TERM/KILL, 2 another signal, 3 raw.
-- Preserve the complete raw status in the low word for unexpected wait states.
@[export tv_supervisor_status]
def exitStatus (status : UInt32) : UInt64 :=
  let signal := status &&& 127
  if signal == 0 then ((status >>> 8) &&& 255).toUInt64
  else if signal == 127 then result status 3
  else if signal == 9 || signal == 15 then result signal 1
  else result signal 2

end TV.Supervisor
