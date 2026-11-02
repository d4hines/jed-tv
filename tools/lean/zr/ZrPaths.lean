import Lean.Data.Json

namespace ZrPaths

/-- Read the literal buildDir used by this repository's Lake configurations.
Host test/tool packages without an override use Lake's normal default. -/
def buildDir (pkg : System.FilePath) : IO System.FilePath := do
  for name in ["lakefile.lean", "lakefile.toml"] do
    let config := pkg / name
    if ← config.pathExists then
      for line in (← IO.FS.readFile config).splitOn "\n" do
        let line := line.trimAscii.toString
        if line.startsWith "buildDir" then
          let parts := line.splitOn "\""
          match parts with
          | [_, path, tail] =>
            unless tail.trimAscii.toString.isEmpty do
              throw (IO.userError s!"{config}: buildDir must be a literal string")
            return pkg / path
          | _ => throw (IO.userError s!"{config}: buildDir must be a literal string")
  return pkg / ".lake/build"

end ZrPaths
