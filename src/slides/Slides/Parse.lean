import Slides.Deck
namespace Slides
private def dropS (s : String) (n : Nat) := (s.drop n).toString
private def takeS (s : String) (n : Nat) := (s.take n).toString
private def trim (s : String) := s.trimAscii.toString
private def failAt (line : Nat) (msg : String) : Except String α := .error s!"line {line}: {msg}"
/-- Delimiter parser; escaped markers remain literal, code spans remain verbatim. -/
def inline (line : Nat) (s : String) : Except String (Array Span) := do
  let cs := s.toList.toArray
  let mut out := #[]
  let mut buf := ""
  let mut i := 0
  while i < cs.size do
    let c := cs[i]!
    if c == '\\' && i + 1 < cs.size then
      buf := buf.push cs[i+1]!
      i := i+2
    else if c == '`' || (c == '*' && i+1 < cs.size && cs[i+1]! != ' ') then
      if !buf.isEmpty then out := out.push ⟨buf, .plain⟩; buf := ""
      let mut n := 1
      while i+n < cs.size && cs[i+n]! == c && n < 3 do n := n+1
      let mut j := i+n
      let mut found := false
      while j+n ≤ cs.size && !found do
        found := (List.range n).all fun k => cs[j+k]! == c
        if !found then j := j+1
      if !found then return ← failAt line "unclosed inline delimiter"
      let text := String.ofList ((cs.toList.drop (i+n)).take (j-i-n))
      let style := if c == '`' then Style.code else if n == 3 then .boldItalic else if n == 2 then .bold else .italic
      out := out.push ⟨text, style⟩
      i := j+n
    else
      buf := buf.push c
      i := i+1
  if !buf.isEmpty then out := out.push ⟨buf, .plain⟩
  return out
private def nonempty (s : Slide) := !s.title.isEmpty || !s.blocks.isEmpty || !s.notes.isEmpty
/-- Runtime parser shared by the application, build validation and host tests. -/
def parse (source : String) : Except String Deck := do
  let ls := source.splitOn "\n" |>.toArray
  let mut slides := #[]
  let mut s : Slide := {}
  let mut code : Option String := none
  let mut codeLines := #[]
  let mut paragraph := ""
  let mut paraLine := 1
  let mut lastDepth := 0
  let mut continuation := false
  for i in [:ls.size] do
    let line := i+1
    let raw := ls[i]!
    let raw := if raw.endsWith "\r" then takeS raw (raw.length-1) else raw
    let t := trim raw
    if let some lang := code then
      if t.startsWith "```" then
        s := { s with blocks := s.blocks.push (.code lang codeLines) }
        code := none; codeLines := #[]
      else
        codeLines := codeLines.push (raw.replace "\t" "    ")
      continue
    let numbered := t.splitOn ". "
    let itemStart := t.startsWith "- " || (numbered.length > 1 && numbered.head!.toNat?.isSome)
    let boundary := t.isEmpty || t == "---" || t.startsWith "#" || t.startsWith "```" || t.startsWith "Note:" || t.startsWith "![" || itemStart
    if boundary && !paragraph.isEmpty then
      s := { s with blocks := s.blocks.push (.para (← inline paraLine paragraph)) }
      paragraph := ""
    if t == "---" then
      if !nonempty s then return ← failAt line "empty slide"
      slides := slides.push s
      s := { line := line+1 }
      lastDepth := 0
    else if t.isEmpty then continuation := false
    else if t.startsWith "```" then code := some (trim (dropS t 3))
    else if raw.contains '\t' then return ← failAt line "tabs are only allowed inside code"
    else if t.startsWith "###" then return ← failAt line "only title and subtitle headings are supported"
    else if t.startsWith "## " then
      s := { s with blocks := s.blocks.push (.subtitle (← inline line (dropS t 3))) }
    else if t.startsWith "# " then
      if !s.title.isEmpty || !s.blocks.isEmpty then return ← failAt line "title must be first and unique"
      s := { s with title := ← inline line (dropS t 2) }
    else if t.startsWith "Note:" then s := { s with notes := s.notes.push (trim (dropS t 5)) }
    else if t.startsWith "![" then
      let parts := (dropS t 2).splitOn "]("
      if parts.length != 2 || !t.endsWith ")" then return ← failAt line "expected ![caption](path)"
      let path := parts[1]!
      s := { s with blocks := s.blocks.push (.image parts[0]! (takeS path (path.length-1))) }
    else if itemStart then
      let spaces := raw.toList.takeWhile (· == ' ') |>.length
      let depth := spaces/2
      let previousItem := match s.blocks.back? with | some (.item ..) => true | _ => false
      if spaces%2 != 0 || depth > 3 || depth > lastDepth+1 || (!previousItem && depth != 0) then
        return ← failAt line "list indentation must advance by two spaces, at most four levels"
      lastDepth := depth
      continuation := true
      let parts := t.splitOn ". "
      let marker := if t.startsWith "- " then "•" else parts[0]! ++ "."
      let text := if t.startsWith "- " then dropS t 2 else String.intercalate ". " parts.tail
      s := { s with blocks := s.blocks.push (.item depth marker (← inline line text)) }
    else
      if paragraph.isEmpty then
        match if continuation then s.blocks.back? else none with
        | some (.item d m spans) =>
          let more ← inline line t
          s := { s with blocks := s.blocks.pop.push (.item d m (spans ++ #[⟨" ", .plain⟩] ++ more)) }
        | _ => paragraph := t; paraLine := line
      else paragraph := paragraph ++ " " ++ t
  if code.isSome then return ← failAt ls.size "unclosed code fence"
  if !paragraph.isEmpty then s := { s with blocks := s.blocks.push (.para (← inline paraLine paragraph)) }
  if !nonempty s then return ← failAt ls.size "empty slide"
  return { slides := slides.push s }
end Slides
