import Init

/- The menu reducer uses scalar fixed-width values so its generated C runs
   on ARM32 without allocating Lean objects or requiring the Lean runtime.
   State/shape wire layout: row/count in bits 0..7, three columns/widths in
   bits 8..23, 24..39, 40..55. Effects occupy output bits 56..63. -/
namespace TV.Menu

@[inline] def clamp (p n : UInt32) : UInt32 :=
  if n == 0 then 0 else if p < n then p else n - 1

@[inline] def move (p n : UInt32) (positive : Bool) : UInt32 :=
  let p := clamp p n
  if positive then clamp (p + 1) n else if p == 0 then 0 else p - 1

@[inline] def effect (key repeated : UInt32) : UInt64 :=
  if repeated != 0 then 0
  else if key == 4 then 1 else if key == 5 then 2 else if key == 6 then 3 else 0

@[export tv_menu_step]
def step (state shape : UInt64) (key repeated : UInt32) : UInt64 :=
  let count := (shape &&& 255).toUInt32
  let n0 := ((shape >>> 8) &&& 65535).toUInt32
  let n1 := ((shape >>> 24) &&& 65535).toUInt32
  let n2 := ((shape >>> 40) &&& 65535).toUInt32
  let row := clamp (state &&& 255).toUInt32 count
  let c0 := clamp ((state >>> 8) &&& 65535).toUInt32 n0
  let c1 := clamp ((state >>> 24) &&& 65535).toUInt32 n1
  let c2 := clamp ((state >>> 40) &&& 65535).toUInt32 n2
  let vertical := if key == 2 then move row count false
                  else if key == 3 then move row count true else row
  let c0 := if (key == 0 || key == 1) && row == 0 then move c0 n0 (key == 1) else c0
  let c1 := if (key == 0 || key == 1) && row == 1 then move c1 n1 (key == 1) else c1
  let c2 := if (key == 0 || key == 1) && row == 2 then move c2 n2 (key == 1) else c2
  vertical.toUInt64 ||| (c0.toUInt64 <<< 8) ||| (c1.toUInt64 <<< 24) |||
    (c2.toUInt64 <<< 40) ||| (effect key repeated <<< 56)

end TV.Menu
