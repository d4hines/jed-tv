import ZrPaths
import Std.Data.HashMap
import Std.Data.HashSet

/-!
# zr-cgen: the runtime build's passes over the Lean compiler's C output

`zr-build gen` copies the compiler's C for `ZrLean.Low` (OUT/rt) and `ZrTv`
(OUT/tv) and then runs, in this order:

    zr-cgen abi-normalize OUT           drop the redundant `zr_prim_apply_stack` declaration
    zr-cgen header-inline OUT/rt INC    inline the scalar header modules (zr_inline.generated.h,
                                        bindings.tsv for zr-abi-gen)
    zr-cgen fastpath FAST.c OUT.h       the forced-inline array push fast paths
    zr-cgen check OUT                   ZrLow check (after planting violations, to show it catches them)

and for apps (`zr-build app`):

    zr-cgen imports PKG MODULE          the compiled C of MODULE and the package modules it imports

None of them writes C of its own: they delete one declaration, copy and
rename compiler output, lower scalar operations to `zr_prim_*` names and
check it. Each fails closed on any shape it does not recognise.

Functions are found as the compiler prints them: a line `TYPE NAME(PARAMS){`
at column 0, the body up to the line where the braces balance.
-/

namespace ZrCGen

open Std

/-! ## Text -/

def isWord (c : Char) : Bool := c.isAlphanum || c == '_'
def isIdStart (c : Char) : Bool := c.isAlpha || c == '_'
def isSpace (c : Char) : Bool := c == ' ' || c == '\t' || c == '\n' || c == '\r' || c == '\x0b' || c == '\x0c'

def sub (cs : Array Char) (a b : Nat) : String := String.ofList (cs.extract a b).toList

def front (s : String) : Char := s.toList.headD ' '
def back (s : String) : Char := s.toList.getLastD ' '
def dropS (s : String) (n : Nat) : String := (s.drop n).toString
def dropEndS (s : String) (n : Nat) : String := (s.dropEnd n).toString
def trimS (s : String) : String := s.trimAscii.toString
def trimStartS (s : String) : String := s.trimAsciiStart.toString
/-- `TYPE NAME` → `NAME` is a C identifier after at least one space. -/
def spacedIdent (r : String) : Bool :=
  r.length > 0 && isSpace (front r) && let n := trimStartS r; n.length > 0 && isIdStart (front n) && n.all isWord

/-- Identifiers in `s`, as Python's `re.findall("[A-Za-z_][A-Za-z0-9_]*")`
(a run of word characters starting at a digit yields its tail). -/
def idents (s : String) : Array String := Id.run do
  let cs := s.toList.toArray
  let mut out := #[]
  let mut i := 0
  while i < cs.size do
    if isIdStart cs[i]! then
      let mut j := i + 1
      while j < cs.size && isWord cs[j]! do j := j + 1
      out := out.push (sub cs i j)
      i := j
    else i := i + 1
  return out

/-- Rewrite every whole word (`\bWORD`) that `f` maps and that is followed by
optional whitespace and `(`: a call or declaration of that name. -/
def renameCalls (s : String) (f : String → Option String) : String := Id.run do
  let cs := s.toList.toArray
  let mut out := ""
  let mut i := 0
  while i < cs.size do
    if isWord cs[i]! then
      let mut j := i + 1
      while j < cs.size && isWord cs[j]! do j := j + 1
      let w := sub cs i j
      let mut k := j
      while k < cs.size && isSpace cs[k]! do k := k + 1
      let call := k < cs.size && cs[k]! == '('
      out := out ++ (if call && isIdStart cs[i]! then (f w).getD w else w)
      i := j
    else
      out := out.push cs[i]!
      i := i + 1
  return out

/-- The whole words of `s` followed by optional whitespace and `(`. -/
def calls (s : String) : Array String := Id.run do
  let cs := s.toList.toArray
  let mut out := #[]
  let mut i := 0
  while i < cs.size do
    if isWord cs[i]! then
      let mut j := i + 1
      while j < cs.size && isWord cs[j]! do j := j + 1
      let mut k := j
      while k < cs.size && isSpace cs[k]! do k := k + 1
      if k < cs.size && cs[k]! == '(' && isIdStart cs[i]! then out := out.push (sub cs i j)
      i := j
    else i := i + 1
  return out

def countChar (s : String) (c : Char) : Nat := s.foldl (fun n d => if d == c then n + 1 else n) 0

/-- Lines as Python's `str.split("\n")`. -/
def lines (s : String) : List String := s.splitOn "\n"

/-- Index of the first `(`, if any. -/
def firstParen (s : String) : Option Nat := (s.toList.findIdx? (· == '('))

/-- `PRE NAME(` where `PRE` (at least one character, starting with a letter
or `_`, made of word characters and `extra`) ends in a space or `*`: `NAME`,
the text before it, and the text after the `(`. -/
def splitDecl (l : String) (extra : Char → Bool) : Option (String × String × String) := do
  let p ← firstParen l
  let cs := l.toList.toArray
  guard (p > 0 && isIdStart cs[0]!)
  guard ((cs.extract 0 p).all fun c => isWord c || extra c)
  let mut s := p
  while s > 0 && isWord cs[s - 1]! do s := s - 1
  guard (s > 0 && s < p)
  return (sub cs s p, sub cs 0 s, sub cs (p + 1) cs.size)

/-! ## Functions in the compiler's C -/

structure Fn where
  name : String
  sig : String
  body : String
  deriving Inhabited

/-- `TYPE NAME(PARAMS){` at column 0 (`TYPE` of word characters, spaces
and `*`; no `;` in `PARAMS`): `NAME`. -/
def defName (l : String) : Option String := do
  guard (!l.startsWith "//" && !l.startsWith " " && !l.startsWith "\t" && l.endsWith "){")
  let (name, _, rest) ← splitDecl l (fun c => c == ' ' || c == '*')
  guard (!(dropEndS rest 2).contains ';')
  return name

/-- Every function defined in a module's C, in order. -/
def functions (text : String) : Array Fn := Id.run do
  let ls := (lines text).toArray
  let mut out := #[]
  let mut i := 0
  while i < ls.size do
    match defName ls[i]! with
    | some name =>
      let mut depth : Int := 0
      let mut j := i
      let mut body := #[]
      while j < ls.size do
        depth := depth + countChar ls[j]! '{' - countChar ls[j]! '}'
        body := body.push ls[j]!
        j := j + 1
        if depth == 0 then break
      out := out.push { name, sig := ls[i]!, body := "\n".intercalate body.toList }
      i := j
    | none => i := i + 1
  return out

/-- The body without its signature line. -/
def Fn.code (f : Fn) : String :=
  match f.body.splitOn "\n" with
  | _ :: rest => "\n".intercalate rest
  | [] => ""

/-! ## The ZrLow check -/

def scalarTypes : List String := ["uint8", "uint16", "uint32", "uint64", "usize"]
def scalarOps : List String :=
  ["add", "sub", "mul", "div", "mod", "land", "lor", "xor", "shift_left", "shift_right", "complement", "neg",
   "dec_eq", "dec_lt", "dec_le", "log2", "to_uint8", "to_uint16", "to_uint32", "to_uint64", "to_usize"]

/-- `lean_TYPE_OP` for a scalar type and an operation in `ops`. -/
def isScalarOp (ops : List String) (w : String) : Bool :=
  scalarTypes.any fun t => ops.any fun op => w == s!"lean_{t}_{op}"

/-- lean.h's object types, named by inline `sizeof`/`offsetof` of the layout. -/
def layoutTypes : List String :=
  ["lean_ctor_object", "lean_array_object", "lean_sarray_object", "lean_string_object", "lean_closure_object"]

/-- What makes a function not ZrLow: a boxed parameter, being or reading a
closed term, any lean.h call but scalar arithmetic and comparison (and
`lean_box(0)`, Lean's `()`). -/
def checkBody (f : Fn) : Array String := Id.run do
  let mut bad : Array String := #[]
  let params := match f.sig.splitOn "(" with | _ :: rest => "(".intercalate rest | [] => ""
  if params.contains "lean_object" then bad := bad.push "takes a boxed (lean_object) argument"
  if (f.name.splitOn "_init_").length > 1 then bad := bad.push "is a closed term / module constant initialiser"
  for w in idents ("UNIT".intercalate (f.code.splitOn "lean_box(0)")) do
    if w.startsWith "lean_" && w != "lean_object" && !layoutTypes.contains w && !isScalarOp scalarOps w then
      bad := bad.push s!"uses {w}"
    else if w.startsWith "_init_" || (w.splitOn "_closed_").length > 1 then
      bad := bad.push s!"reads the closed term / constant {w}"
  return (bad.toList.eraseDups.toArray.qsort (· < ·))

def lowPrefixes : List String := ["ZrLean_Low_", "ZrTv_"]

def baseName (f : String) : String := (f.splitOn "/").getLast!

/-- Findings for the modules `(file, C text)`. ZrLow modules are the files
named `ZrLean_Low_*` and `ZrTv_*`; every function in them except the
`___boxed` wrappers and the initialiser is checked, and may call only
checked functions among the compiler's. -/
def checkModules (mods : Array (String × String)) : Array String × Nat × Nat := Id.run do
  let parsed := mods.map fun (f, t) => (f, t, functions t)
  let mut whereIs : HashMap String String := {}
  for (f, _, fs) in parsed do
    for fn in fs do
      if !whereIs.contains fn.name then whereIs := whereIs.insert fn.name f
  let low := parsed.filter fun (f, _, _) => lowPrefixes.any fun pre => (baseName f).startsWith pre
  let mut findings : Array String := #[]
  let mut checked : HashSet String := {}
  let mut count := 0
  for (f, text, fs) in low do
    if text.contains "lean_once_cell_t" then
      findings := findings.push s!"{f}: a closed term / module constant (lean_once_cell_t)"
    for fn in fs do
      if !fn.name.endsWith "___boxed" && !fn.name.startsWith "initialize_" then
        checked := checked.insert fn.name
  for (f, _, fs) in low do
    for fn in fs do
      if fn.name.endsWith "___boxed" || fn.name.startsWith "initialize_" then continue
      count := count + 1
      for b in checkBody fn do findings := findings.push s!"{f}: {fn.name} {b}"
      for w in idents fn.code do
        if w.startsWith "lp_" && w != fn.name && whereIs.contains w && !checked.contains w then
          findings := findings.push s!"{f}: {fn.name} calls {w}, which is not ZrLow ({whereIs.getD w ""})"
  if low.isEmpty then findings := findings.push "no ZrLean_Low_*.c modules"
  return (findings, count, low.size)

/-- The C modules of a generated runtime (OUT/rt and OUT/tv, or the
directory itself). -/
def readModules (gen : System.FilePath) : IO (Array (String × String)) := do
  let mut dirs : Array String := #[]
  for d in ["rt", "tv"] do
    if ← (gen / d).isDir then dirs := dirs.push d
  if dirs.isEmpty then dirs := #[""]
  let mut mods := #[]
  for d in dirs do
    let entries ← (if d.isEmpty then gen else gen / d).readDir
    let names := (entries.map (·.fileName)).filter (·.endsWith ".c") |>.qsort (· < ·)
    for n in names do
      let f := if d.isEmpty then n else s!"{d}/{n}"
      mods := mods.push (f, ← IO.FS.readFile (gen / f))
  return mods

/-- Plant each kind of violation into the heap module's real C and require
the check to report it (and nothing on the unplanted C). -/
def selftest (mods : Array (String × String)) : Except String Unit := do
  let (base, _, _) := checkModules mods
  unless base.isEmpty do throw s!"the unplanted C already fails: {base[0]!}"
  let some (heapFile, heap) := mods.find? (baseName ·.1 == "ZrLean_Low_Heap.c")
    | throw "no ZrLean_Low_Heap.c"
  let fns := functions heap
  -- a body line to plant at, a Lean function to call, a scalar export to box
  let some anchor := (lines heap).find? fun l => l.startsWith "v___x_" && l.endsWith " = ((size_t)0ULL);"
    | throw "no anchor line in the heap module"
  let some boxed := fns.find? (·.name.endsWith "___boxed") | throw "no ___boxed wrapper in the heap module"
  let some exp := fns.find? fun f => f.sig.startsWith "LEAN_EXPORT size_t lp_" && (f.sig.splitOn "(size_t ").length > 1
    | throw "no scalar export in the heap module"
  let atAnchor (s : String) := (anchor, s ++ " " ++ anchor)
  let plants : List (String × String × String) :=
    [("a boxed value", atAnchor "lean_object* leak = lean_box(5); (void)leak;"),
     ("a reference count", atAnchor "lean_inc(0);"),
     ("an allocation", atAnchor "lean_object* leak = lean_alloc_ctor(0, 1, 0); (void)leak;"),
     ("a call into other Lean", atAnchor s!"{boxed.name}(0);"),
     ("a closed term", atAnchor "(void)_init_l_planted;"),
     ("a boxed argument", (exp.sig, exp.sig.replace "(size_t " "(lean_object* v_x_, size_t "))]
  for (what, old, new) in plants do
    let planted := mods.map fun (f, t) =>
      (f, if f == heapFile then
            match t.splitOn old with
            | a :: rest => a ++ new ++ old.intercalate rest
            | [] => t
          else t)
    let (found, _, _) := checkModules planted
    if found.isEmpty then throw s!"planted {what}: not caught"

/-! ## abi-normalize -/

/-- The exact redundant declaration: `size_t zr_prim_apply_stack(size_t[ NAME]);`. -/
def isApplyStackDecl (s : String) : Bool :=
  s == "size_t zr_prim_apply_stack(size_t);" ||
  (s.startsWith "size_t zr_prim_apply_stack(size_t " && s.endsWith ");" &&
    let n := dropEndS (dropS s "size_t zr_prim_apply_stack(size_t ".length) 2
    n.length > 0 && isIdStart (front n) && n.all isWord)

/-- Any other declaration of it (fail closed). -/
def isOtherApplyStackDecl (s : String) : Bool := Id.run do
  let parts := s.splitOn "zr_prim_apply_stack"
  if parts.length != 2 || !s.endsWith ";" || s.contains '{' || s.contains '}' then return false
  let pre := parts[0]!
  let post := trimStartS parts[1]!
  return pre.length > 0 && isIdStart (front pre) && pre.all (fun c => isWord c || c == ' ' || c == '\t' || c == '*') &&
    !isWord (back pre) && post.startsWith "(" && (dropEndS post 1).trimAsciiEnd.toString.endsWith ")"

def normalizeText (text : String) : Except String (String × Nat) := do
  let mut keep : Array String := #[]
  let mut n := 0
  for l in lines text do
    let s := trimS l
    if isApplyStackDecl s then n := n + 1
    else if isOtherApplyStackDecl s then throw s!"unexpected alloca scalar declaration: {s}"
    else keep := keep.push l
  return ("\n".intercalate keep.toList, n)

/-- The declaration is removed, calls are kept, other shapes are rejected. -/
def normalizeSelftest : Except String Unit := do
  let expect (input : String) (out : String × Nat) : Except String Unit := do
    if (← normalizeText input) != out then throw s!"abi-normalize selftest: wrong result for {input}"
  expect "size_t zr_prim_apply_stack(size_t);\nv = zr_prim_apply_stack(n);\n" ("v = zr_prim_apply_stack(n);\n", 1)
  expect "size_t zr_prim_apply_stack(size_t bytes);\n" ("", 1)
  expect "v = zr_prim_apply_stack(n);\n" ("v = zr_prim_apply_stack(n);\n", 0)
  expect "size_t unrelated(size_t);\n" ("size_t unrelated(size_t);\n", 0)
  for bad in ["uint32_t zr_prim_apply_stack(size_t);", "size_t zr_prim_apply_stack(size_t, size_t);",
              "void *zr_prim_apply_stack(size_t);", "extern size_t zr_prim_apply_stack(size_t);"] do
    if (normalizeText (bad ++ "\n")).toOption.isSome then throw s!"abi-normalize selftest: accepted {bad}"

partial def cFiles (d : System.FilePath) : IO (Array System.FilePath) := do
  let mut out := #[]
  for e in (← d.readDir).qsort (·.fileName < ·.fileName) do
    if ← e.path.isDir then out := out ++ (← cFiles e.path)
    else if e.fileName.endsWith ".c" then out := out.push e.path
  return out

def abiNormalize (d : System.FilePath) : IO Unit := do
  let mut changes := #[]
  for p in ← cFiles d do
    match normalizeText (← IO.FS.readFile p) with
    | .error e => throw (IO.userError e)
    | .ok (t, n) => if n > 0 then changes := changes.push (p, t, n)
  for (p, t, _) in changes do IO.FS.writeFile p t
  IO.println s!"abi-normalize: {changes.foldl (· + ·.2.2) 0} alloca macro declarations removed in {changes.size} C files"

/-! ## header-inline -/

/-- `lean_TYPE_OP(` (scalar arithmetic) to `zr_prim_TYPE_OP(`. -/
def lowerOps : List String :=
  ["add", "sub", "mul", "div", "mod", "land", "lor", "xor", "shift_left", "shift_right", "complement", "neg",
   "dec_eq", "dec_lt", "dec_le", "to_uint8", "to_uint16", "to_uint32", "to_uint64", "to_usize"]

def lower (code : String) : String :=
  renameCalls code fun w => if isScalarOp lowerOps w then some ("zr_prim_" ++ dropS w 5) else none

def paramTypes : List String :=
  ["size_t", "uint8_t", "uint16_t", "uint32_t", "uint64_t", "int8_t", "int16_t", "int32_t", "int64_t", "float", "double"]

/-- One scalar (or `lean_object*`) parameter `TYPE NAME`. -/
def scalarParam (p : String) : Bool :=
  let p := trimS p
  let rest? : Option String :=
    if p.startsWith "lean_object" then
      let r := trimStartS (dropS p "lean_object".length)
      if r.startsWith "*" then some (dropS r 1) else none
    else (paramTypes.find? fun t => p.startsWith t).map fun t => dropS p t.length
  match rest? with
  | none => false
  | some r => r.length > 0 && isSpace (front r) && let n := trimStartS r; n.length > 0 && n.all isWord

/-- `[LEAN_EXPORT |static ]RESULT NAME(PARAMS){` with scalar parameters:
result type, name, parameter text, arity. -/
def signatureParts (sig : String) : Except String (String × String × String × Nat) := do
  let s := if sig.startsWith "LEAN_EXPORT " then dropS sig 12
    else if sig.startsWith "static " then dropS sig 7 else sig
  let err := s!"unsupported compiler signature: {sig}"
  let some p := firstParen s | throw err
  unless s.endsWith "){" do throw err
  let cs := s.toList.toArray
  let mut b := p
  while b > 0 && isWord cs[b - 1]! do b := b - 1
  let result := trimS (sub cs 0 b)
  unless b < p && result.length > 0 && (sub cs 0 b).all (fun c => isWord c || isSpace c || c == '*') do throw err
  let name := sub cs b p
  let params := sub cs (p + 1) (cs.size - 2)
  let ps := if trimS params == "" || trimS params == "void" then [] else params.splitOn ","
  unless ps.all scalarParam do throw s!"non-scalar compiler signature: {sig}"
  return (result, name, params, ps.length)

/-- A prototype line `[LEAN_EXPORT ]TYPE NAME(PARAMS);`. -/
def isPrototype (l : String) : Bool :=
  let s := if l.startsWith "LEAN_EXPORT " then dropS l 12 else l
  l.endsWith ");" && match splitDecl s (fun c => isSpace c || c == '*') with
    | some (_, _, rest) => !(dropEndS rest 2).contains ';'
    | none => false

/-- The name a prototype declares: the word before its first `(`. -/
def declaredName (proto : String) : String :=
  match firstParen proto with
  | some p => Id.run do
    let cs := proto.toList.toArray
    let mut s := p
    while s > 0 && isWord cs[s - 1]! do s := s - 1
    return sub cs s p
  | none => ""

def headerInline (rt inc : System.FilePath) : IO Unit := do
  let modules := ((← rt.readDir).map (·.fileName)).filter (fun n => n.startsWith "ZrLean_Low_Header" && n.endsWith ".c")
    |>.qsort (· < ·)
  if modules.isEmpty then throw (IO.userError "no compiled scalar header modules")
  let mut fns : Array Fn := #[]
  let mut names : HashSet String := {}
  let mut prototypes : HashSet String := {}
  for m in modules do
    let text := lower (← IO.FS.readFile (rt / m))
    IO.FS.writeFile (rt / m) text
    let parsed := functions text
    for fn in parsed do
      if fn.name.startsWith "initialize_" || fn.name.startsWith "_init_" || fn.name.endsWith "___boxed" then continue
      let bad := checkBody fn
      unless bad.isEmpty do throw (IO.userError s!"{m}: {fn.name}: {bad}")
      if let .error e := signatureParts fn.sig then throw (IO.userError e)
      if names.contains fn.name then throw (IO.userError s!"duplicate scalar symbol {fn.name}")
      names := names.insert fn.name
      fns := fns.push fn
    -- other Low modules and opaque primitives, as the compiler declared them
    let firstDef := parsed.foldl (fun n fn => min n ((text.splitOn fn.sig).head!.length)) text.length
    for l in lines (String.ofList (text.toList.take firstDef)) do
      if isPrototype l then
        prototypes := prototypes.insert (if l.startsWith "LEAN_EXPORT " then dropS l 12 else l)
  let rename (s : String) := renameCalls s fun w => if names.contains w then some ("zr_inline_" ++ w) else none
  let mut out : Array String := #["/* Generated solely from checked scalar Lean compiler output. */", "#pragma once"]
  for p in prototypes.toArray.qsort (· < ·) do
    if !names.contains (declaredName p) then out := out.push p
  let mut parts : Array (Fn × String × String × Nat) := #[]
  for fn in fns do
    let .ok (result, _, params, arity) := signatureParts fn.sig | unreachable!
    parts := parts.push (fn, result, params, arity)
    out := out.push s!"static inline {result} zr_inline_{fn.name}({params});"
  let mut bindings : Array (String × Nat) := #[]
  for (fn, result, params, arity) in parts do
    out := out.push s!"static inline {result} zr_inline_{fn.name}({params})\{\n{rename fn.code}"
    if fn.name.startsWith "zr_low_header_" then
      bindings := bindings.push ("lean_" ++ dropS fn.name "zr_low_header_".length, arity)
  IO.FS.createDirAll inc
  IO.FS.writeFile (inc / "zr_inline.generated.h") ("\n".intercalate out.toList ++ "\n")
  let sorted := bindings.qsort fun a b => a.1 < b.1 || (a.1 == b.1 && a.2 < b.2)
  IO.FS.writeFile (inc / "bindings.tsv") (String.join (sorted.toList.map fun (n, a) => s!"{n}\t{a}\n"))
  IO.println s!"header-inline: {fns.size} scalar functions, {bindings.size} ABI bindings"

/-! ## fastpath -/

/-- The whole definition `LEAN_EXPORT size_t SYMBOL(...){...}` (from a line
start, braces balanced). -/
def definition (source symbol : String) : Except String String := do
  let head := "LEAN_EXPORT size_t " ++ symbol ++ "("
  let cs := source.toList.toArray
  -- the line starts where `head` occurs (a prototype also does: try each)
  let parts := source.splitOn ("\n" ++ head)
  let mut starts : Array Nat := if source.startsWith head then #[0] else #[]
  let mut pos := 0
  for p in parts.dropLast do
    pos := pos + p.length + 1
    starts := starts.push pos
    pos := pos + head.length
  for start in starts do
    -- `\([^;{}]*\)\s*\{`: the first of `;{}` is the `{`, after `)` and spaces
    let mut j := start + head.length
    while j < cs.size && cs[j]! != ';' && cs[j]! != '{' && cs[j]! != '}' do j := j + 1
    let mut k := j
    while k > start + head.length && isSpace cs[k - 1]! do k := k - 1
    unless j < cs.size && cs[j]! == '{' && k > start + head.length && cs[k - 1]! == ')' do continue
    let mut depth := 1
    let mut e := j + 1
    while e < cs.size && depth > 0 do
      if cs[e]! == '{' then depth := depth + 1
      if cs[e]! == '}' then depth := depth - 1
      e := e + 1
    if depth > 0 then throw s!"unclosed function {symbol}"
    return sub cs start e
  throw s!"missing scalar function {symbol}"

/-- A cursor over characters, for the one fixed C shape checked below. -/
def skipWs (cs : Array Char) (i : Nat) : Nat := Id.run do
  let mut i := i
  while i < cs.size && isSpace cs[i]! do i := i + 1
  return i

def lit (cs : Array Char) (i : Nat) (s : String) : Option Nat :=
  if sub cs i (i + s.length) == s then some (i + s.length) else none

def ident (cs : Array Char) (i : Nat) : Option (String × Nat) := Id.run do
  if !(i < cs.size && isIdStart cs[i]!) then return none
  let mut j := i + 1
  while j < cs.size && isWord cs[j]! do j := j + 1
  return some (sub cs i j, j)

/-- `\s*{\s*size_t X;\s*X = SLOW([^;{}]*);\s*return X;\s*}` at `i`. -/
def coldArm (cs : Array Char) (i : Nat) (slow : String) : Bool := Id.run do
  let some i := lit cs (skipWs cs i) "{" | return false
  let some i := lit cs (skipWs cs i) "size_t " | return false
  let some (x, i) := ident cs i | return false
  let some i := lit cs i ";" | return false
  let some i := lit cs (skipWs cs i) (x ++ " = " ++ slow ++ "(") | return false
  let mut e := i
  while e < cs.size && cs[e]! != ';' && cs[e]! != '{' && cs[e]! != '}' do e := e + 1
  -- `[^;{}]*\);`: the first of `;{}` is the `;`, right after `)`
  unless e < cs.size && cs[e]! == ';' && e > i && cs[e - 1]! == ')' do return false
  let some r := lit cs (skipWs cs (e + 1)) ("return " ++ x ++ ";") | return false
  return (lit cs (skipWs cs r) "}").isSome

/-- The variables assigned a branch hint, `X = LEAN_LIKELY(...);` or
`X = zr_prim_fast_likely(...);` (no `;` inside), in order. -/
def hinted (function : String) : Array String := Id.run do
  let cs := function.toList.toArray
  let mut out := #[]
  let mut i := 0
  while i < cs.size do
    match ident cs i with
    | some (x, j) =>
      for h in [" = LEAN_LIKELY(", " = zr_prim_fast_likely("] do
        if let some k := lit cs j h then
          let mut e := k
          while e < cs.size && cs[e]! != ';' do e := e + 1
          if e > k + 1 && e < cs.size && cs[e - 1]! == ')' then out := out.push x
      i := j
    | none => i := i + 1
  return out

/-- The forced-inline copy of the fast path `symbol`, renamed `inlineName`,
after checking it is scalar, calls no other Lean, allocates nothing and its
branch hints lead straight to the slow path. -/
def extract (source symbol inlineName : String) : Except String String := do
  let function ← definition source symbol
  let (signature, body) := match function.splitOn "{" with
    | s :: rest => (s, "{".intercalate rest)
    | [] => ("", "")
  if signature.contains "lean_object" then throw s!"boxed ABI in {symbol}"
  let params := ((signature.splitOn (symbol ++ "(")).getD 1 "").splitOn ")" |>.head!
  let mut types := #[]
  for p in params.splitOn "," do
    let p := trimS p
    let t? := ["size_t", "uint8_t"].find? fun t => p.startsWith t && spacedIdent (dropS p t.length)
    match t? with
    | some t => types := types.push t
    | none => throw s!"unexpected scalar ABI in {symbol}: {p}"
  let expected := #["size_t", if (symbol.splitOn "byte_array").length > 1 then "uint8_t" else "size_t"]
  if types != expected then throw s!"wrong scalar argument ABI in {symbol}: {types}"
  let helpers := (calls body).filter (fun w => w.startsWith "l_" || w.startsWith "lp_") |>.toList.eraseDups
  unless helpers.isEmpty do throw s!"{symbol} has non-inlined Lean helpers: {helpers}"
  let forbidden := (calls body).filter fun w =>
    w.startsWith "lean_alloc" || w.startsWith "lean_inc" || w == "lean_dec" || w.startsWith "lean_dec_ref" ||
    w.startsWith "lean_nat_" || w == "lean_box_usize" || w.startsWith "lean_box_uint"
  unless forbidden.isEmpty do throw s!"runtime allocation/RC/boxing in {symbol}: {forbidden}"
  -- Lean's Bool is uint8_t and clang drops __builtin_expect through the
  -- assignment: put the authored hint back on the branch that uses it.
  let mut function := function
  let slow := dropEndS symbol "_fast".length ++ "_slow"
  for v in hinted function do
    let condition := s!"if ({v} == 0)"
    match function.splitOn condition with
    | [a, rest] =>
      unless coldArm rest.toList.toArray 0 slow do throw s!"branch hint does not lead directly to {slow}"
      function := a ++ s!"if (LEAN_UNLIKELY({v} == 0))" ++ rest
    | _ => throw s!"unrecognized branch-hint use in {symbol}: {v}"
  let (a, b) := match function.splitOn ("LEAN_EXPORT size_t " ++ symbol) with
    | a :: rest => (a, ("LEAN_EXPORT size_t " ++ symbol).intercalate rest)
    | [] => (function, "")
  return a ++ "static inline size_t " ++ inlineName ++ b

def fastpathHeader (source : String) : Except String String := do
  let pairs := [("zr_low_byte_array_push_fast", "zr_inline_byte_array_push"),
                ("zr_low_array_push_fast", "zr_inline_array_push")]
  let fns ← pairs.mapM fun (s, n) => extract source s n
  let decls ← ["zr_low_byte_array_push_slow", "zr_low_array_push_slow"].mapM fun s =>
    match (lines source).find? fun l => l.startsWith ("size_t " ++ s ++ "(") && l.endsWith ");" &&
        !(dropEndS (dropS l (s.length + 8)) 2).any (fun c => c == ';' || c == '{' || c == '}') with
    | some l => pure l
    | none => throw s!"missing slow-path ABI declaration {s}"
  return "/* Generated from ZrLean.Low.Fast's compiler output by zr-cgen fastpath; do not edit. */\n" ++
    "#pragma once\n#include <lean/lean.h>\n\n" ++ "\n".intercalate decls ++ "\n\n" ++ "\n\n".intercalate fns ++
    "\n\n/* Calling-convention bridges and aliases are authored in ZrAbi.Fast. */\n" ++
    "#include <lean/zr_fastpath_abi.generated.h>\n"

/-- Malformed fast paths that must be rejected. -/
def fastpathSelftest (source : String) : Except String Unit := do
  let _ ← fastpathHeader source
  let entry := "LEAN_EXPORT size_t zr_low_array_push_fast("
  let some line := (lines source).find? (fun l => l.startsWith entry && l.endsWith "{")
    | throw "selftest lacks actual compiled fastpath"
  let tests := [("non-inlined helper", source.replace line (line ++ "\nlp_unchecked_helper();")),
                ("allocation", source.replace line (line ++ "\nlean_alloc_ctor(0,0,0);")),
                ("wrong cold arm", source.replace "= zr_low_array_push_slow(" "= zr_low_byte_array_push_slow("),
                ("missing scalar export", source.replace "zr_low_array_push_fast" "missing_array_push_fast")]
  for (what, bad) in tests do
    if (fastpathHeader bad).toOption.isSome then throw s!"selftest did not reject {what}"

/-! ## imports -/

/-- The modules `start` imports transitively within the package at `root`
(only imports whose source file exists there), sorted, including `start`. -/
partial def importClosure (root : System.FilePath) (start : String) : IO (Array String) := do
  let file (m : String) := (m.splitOn ".").foldl (· / ·) root |>.addExtension "lean"
  let mut seen : HashSet String := {}
  let mut todo := [start]
  while !todo.isEmpty do
    let m := todo.head!
    todo := todo.tail!
    if seen.contains m then continue
    seen := seen.insert m
    for l in lines (← IO.FS.readFile (file m)) do
      match (trimS l).splitOn " " |>.filter (· != "") with
      | "import" :: i :: _ => if ← (file i).pathExists then todo := i :: todo
      | _ => pure ()
  return seen.toArray.qsort (· < ·)

end ZrCGen

open ZrCGen

def orFail {α} (r : Except String α) : IO α :=
  match r with
  | .ok a => pure a
  | .error e => throw (IO.userError e)

def main (args : List String) : IO UInt32 := do
  match args with
  | ["abi-normalize", d] => orFail normalizeSelftest; abiNormalize d; return 0
  | ["header-inline", rt, inc] => headerInline rt inc; return 0
  | ["fastpath", src, out] =>
    let source ← IO.FS.readFile src
    orFail (fastpathSelftest source)
    IO.FS.writeFile out (← orFail (fastpathHeader source))
    IO.println "fastpath: PASS (rejects a helper, an allocation, a wrong cold arm, a missing export)"
    return 0
  | ["check", gen] =>
    let mods ← readModules gen
    orFail (selftest mods)
    let (findings, n, files) := checkModules mods
    for f in findings do IO.println s!"FAIL {f}"
    IO.println s!"zrlow-check: {n} functions in {files} ZrLow modules (planted violations caught): {if findings.isEmpty then "PASS" else "FAIL"}"
    return (if findings.isEmpty then 0 else 1)
  | ["imports", root, start] =>
    for m in ← importClosure root start do
      IO.println (((m.splitOn ".").foldl (· / ·) ((← ZrPaths.buildDir (System.FilePath.mk root)) / "ir")).addExtension "c").toString
    return 0
  | _ =>
    IO.eprintln "usage: zr-cgen abi-normalize OUT | header-inline OUT/rt INC | fastpath FAST.c OUT.h | check OUT | imports PKG MODULE"
    return 2
