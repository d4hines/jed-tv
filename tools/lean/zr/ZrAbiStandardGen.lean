import ZrAbi.Standard

def main (args : List String) : IO UInt32 := do
  match args with
  | [directory] =>
    ZrAbi.Standard.write directory
    IO.println "standard-headers: generated string.h and lean/config.h"
    return 0
  | _ =>
    IO.eprintln "usage: zr-standard-headers OUT_INCLUDE_DIRECTORY"
    return 2
