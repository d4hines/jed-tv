import ZrAbi.DisplayMemory

def main (args : List String) : IO UInt32 := do
  let [path] := args | throw (IO.userError "usage: display-header OUTPUT")
  IO.FS.writeFile path (ZrAbi.C.renderHeader ZrAbi.DisplayMemory.header)
  return 0
