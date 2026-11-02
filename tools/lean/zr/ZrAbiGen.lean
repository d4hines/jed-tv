import ZrAbi.Standard
import ZrAbi.Runtime
import ZrAbi.Platform
import ZrAbi.Fast

/- Generate ABI/layout declarations and primitive lowering from Lean source.
   The bindings file contains only exported names and scalar arities from the
   compiler output; it cannot supply executable C or alter the ABI schema. -/
def main (args : List String) : IO UInt32 := do
  match args with
  | [out, bindingFile] =>
    let lines := (← IO.FS.readFile bindingFile).splitOn "\n"
    let mut bindings : List ZrAbi.Runtime.Binding := []
    for line in lines do
      if !line.isEmpty then
        match line.splitOn "\t" with
        | [name,arity] =>
          let some n := arity.toNat? | throw (IO.userError s!"invalid ABI arity: {line}")
          if !name.startsWith "lean_" then throw (IO.userError s!"invalid ABI binding: {line}")
          if !(ZrAbi.RuntimeDecls.functions.any (·.name == name)) then
            throw (IO.userError s!"ABI binding has no declared signature: {name}")
          bindings := bindings ++ [(name,n)]
        | _ => throw (IO.userError s!"invalid ABI binding row: {line}")
    ZrAbi.Standard.write out
    IO.FS.writeFile s!"{out}/lean/lean.h" (ZrAbi.C.renderHeader (ZrAbi.Runtime.header bindings))
    IO.FS.writeFile s!"{out}/zr_platform.h" (ZrAbi.C.renderHeader ZrAbi.Platform.header)
    IO.FS.writeFile s!"{out}/lean/zr_fastpath.h" (ZrAbi.C.renderHeader ZrAbi.Fast.shim)
    IO.FS.writeFile s!"{out}/lean/zr_fastpath_abi.generated.h" (ZrAbi.C.renderHeader ZrAbi.Fast.bridges)
    return 0
  | _ =>
    IO.eprintln "usage: zr-abi-gen INCLUDE_DIR BINDINGS.tsv"
    return 2
