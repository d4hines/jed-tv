import Memory

/- Compact FDT v17 validation and property editing, entirely compiled Lean.
   All pointers name caller-owned stable RAM. Errors abort boot preparation;
   a failed multi-property edit may leave a partially edited tree.
   Scalar tail recursion avoids the allocating generic While combinator. -/
namespace LeanBoot.Fdt

variable {m : Type → Type} [Monad m] [Memory.Reader m] [Memory.Writer m]
open Memory

def missing : UInt32 := 0xffffffff
@[specialize] private def be (b n o : UInt32) : m UInt32 := do
  return (← readBE32 b n o).toUInt32
@[specialize] private def aligned (n : UInt32) : UInt32 := (n + 3) &&& 0xfffffffc

@[specialize] private partial def strlenLoop (b n origin p : UInt32) : m UInt32 := do
  if p >= n then return missing
  if (← read8 b n p) == 0 then return p - origin
  strlenLoop b n origin (p + 1)
@[specialize] private def strlen (b n o : UInt32) : m UInt32 := strlenLoop b n o o

@[specialize] private partial def equalLoop (a an ao b bn bo len i : UInt32) : m Bool := do
  if i == len then return true
  if (← read8 a an (ao + i)) != (← read8 b bn (bo + i)) then return false
  equalLoop a an ao b bn bo len (i + 1)
@[specialize] private def equal (a an ao b bn bo len : UInt32) : m Bool := do
  if !contains an ao len || !contains bn bo len then return false
  equalLoop a an ao b bn bo len 0

@[specialize] private partial def reserveTerminated (b cap limit p : UInt32) : m Bool := do
  if !contains limit p 16 then return false
  let a ← be b cap p
  let c ← be b cap (p + 4)
  let d ← be b cap (p + 8)
  let e ← be b cap (p + 12)
  if a == 0 && c == 0 && d == 0 && e == 0 then return true
  reserveTerminated b cap limit (p + 16)

@[specialize] private partial def validateTokens (b cap strings stringSize p depth roots : UInt32) : m UInt32 := do
  if !contains strings p 4 then return 8
  let token ← be b cap p
  let p := p + 4
  if token == 1 then
    let len ← strlen b strings p
    if len == missing then return 6
    if depth == 0 && (len != 0 || roots != 0) then return 6
    if depth != 0 && len == 0 then return 6
    if depth >= 64 then return 6
    validateTokens b cap strings stringSize (aligned (p + len + 1)) (depth + 1)
      (if depth == 0 then roots + 1 else roots)
  else if token == 2 then
    if depth == 0 then return 6
    validateTokens b cap strings stringSize p (depth - 1) roots
  else if token == 3 then
    if depth == 0 || !contains strings p 8 then return 7
    let len ← be b cap p
    let name ← be b cap (p + 4)
    let p := p + 8
    if !contains strings p len || name >= stringSize then return 7
    let nameLen ← strlen b (strings + stringSize) (strings + name)
    if nameLen == missing || nameLen == 0 then return 7
    validateTokens b cap strings stringSize (aligned (p + len)) depth roots
  else if token == 9 then
    if depth != 0 || roots != 1 || p != strings then return 8
    return 0
  else if token == 4 then validateTokens b cap strings stringSize p depth roots
  else return 8

@[specialize]
def validate (b cap : UInt32) : m UInt32 := do
  if !addressable b cap || cap < 40 || cap > 0x100000 then return 1
  if (← be b cap 0) != 0xd00dfeed then return 2
  let total ← be b cap 4
  let structureOffset ← be b cap 8
  let strings ← be b cap 12
  let reserve ← be b cap 16
  let version ← be b cap 20
  let compatible ← be b cap 24
  let stringSize ← be b cap 32
  let structSize ← be b cap 36
  if total > cap || version != 17 || compatible != 16 then return 3
  if structureOffset < 40 || structureOffset &&& 3 != 0 || reserve < 40 || reserve &&& 7 != 0 ||
      !contains total structureOffset structSize || !contains total strings stringSize ||
      structureOffset.toUInt64 + structSize.toUInt64 != strings.toUInt64 then return 4
  if !(← reserveTerminated b cap structureOffset reserve) then return 5
  validateTokens b cap strings stringSize structureOffset 0 0

-- Helpers below operate only on a validated tree. The editor preserves the
-- compact structure-before-strings layout and checks every capacity growth.
@[specialize] private def next (b cap p : UInt32) : m UInt32 := do
  let token ← be b cap p
  if token == 1 then return aligned (p + 5 + (← strlen b cap (p + 4)))
  else if token == 3 then return p + 12 + aligned (← be b cap (p + 4))
  else return p + 4

@[specialize] private partial def childLoop (b cap p depth name nameSize off len : UInt32) : m UInt32 := do
  if p >= cap then return missing
  let token ← be b cap p
  if token == 1 then
    if depth == 0 && (← strlen b cap (p + 4)) == len &&
        (← equal b cap (p + 4) name nameSize off len) then return p
    childLoop b cap (← next b cap p) (depth + 1) name nameSize off len
  else if token == 2 then
    if depth == 0 then return missing
    childLoop b cap (← next b cap p) (depth - 1) name nameSize off len
  else childLoop b cap (← next b cap p) depth name nameSize off len

@[specialize] private partial def nodeEndLoop (b cap p depth : UInt32) : m UInt32 := do
  if p >= cap then return missing
  let token ← be b cap p
  if token == 1 then nodeEndLoop b cap (← next b cap p) (depth + 1)
  else if token == 2 then
    if depth == 1 then return p
    nodeEndLoop b cap (← next b cap p) (depth - 1)
  else nodeEndLoop b cap (← next b cap p) depth

@[specialize] private def splice (b cap position oldSize newSize : UInt32) : m UInt32 := do
  let strings ← be b cap 12
  let stringSize ← be b cap 32
  let structSize ← be b cap 36
  let used := strings + stringSize
  if !contains strings position oldSize || oldSize > structSize then return 9
  let newUsed := used.toUInt64 - oldSize.toUInt64 + newSize.toUInt64
  if newUsed > cap.toUInt64 then return 9
  if (← copy b cap (position + newSize) b cap (position + oldSize) (used - position - oldSize)) != 0 then return 9
  let _ ← writeBE32 b cap 4 newUsed.toUInt32
  let _ ← writeBE32 b cap 12 (strings - oldSize + newSize)
  let _ ← writeBE32 b cap 36 (structSize - oldSize + newSize)
  return 0

@[specialize] private partial def segmentEnd (path pathSize off len p : UInt32) : m UInt32 := do
  if p == len then return p
  let c ← read8 path pathSize (off + p)
  if c == 0 then return missing
  if c == 47 then return p
  segmentEnd path pathSize off len (p + 1)

@[specialize] private partial def ensureLoop (b cap path pathSize off len p node depth : UInt32) : m UInt32 := do
  if p == len then return node
  let stop ← segmentEnd path pathSize off len p
  if stop == missing || stop == p || depth >= 64 then return missing
  let size := stop - p
  let found ← childLoop b cap (← next b cap node) 0 path pathSize (off + p) size
  let node ← if found != missing then pure found else do
    let position ← nodeEndLoop b cap (← next b cap node) 1
    let added := 8 + aligned (size + 1)
    if position == missing || (← splice b cap position 0 added) != 0 then return missing
    let _ ← fill b cap position added 0
    let _ ← writeBE32 b cap position 1
    let _ ← copy b cap (position + 4) path pathSize (off + p) size
    let _ ← writeBE32 b cap (position + added - 4) 2
    pure position
  if node == missing then return missing
  if stop == len then return node
  if stop + 1 == len then return missing
  ensureLoop b cap path pathSize off len (stop + 1) node (depth + 1)

@[specialize] private def ensureNode (b cap path pathSize off len : UInt32) : m UInt32 := do
  if len == 0 || (← read8 path pathSize off) != 47 then return missing
  ensureLoop b cap path pathSize off len 1 (← be b cap 8) 1

@[specialize] private partial def findName (b cap strings stringSize src srcSize off len p : UInt32) : m UInt32 := do
  if p >= stringSize then return missing
  let size ← strlen b (strings + stringSize) (strings + p)
  if size == missing then return missing
  if size == len && (← equal b cap (strings + p) src srcSize off len) then return p
  findName b cap strings stringSize src srcSize off len (p + size + 1)

@[specialize] private def nameIndex (b cap src srcSize off len : UInt32) : m UInt32 := do
  let strings ← be b cap 12
  let stringSize ← be b cap 32
  let found ← findName b cap strings stringSize src srcSize off len 0
  if found != missing then return found
  if !contains cap (strings + stringSize) (len + 1) then return missing
  let _ ← copy b cap (strings + stringSize) src srcSize off len
  let _ ← write8 b cap (strings + stringSize + len) 0
  let _ ← writeBE32 b cap 32 (stringSize + len + 1)
  let _ ← writeBE32 b cap 4 (strings + stringSize + len + 1)
  return stringSize

@[specialize] private partial def propertyLoop (b cap strings p depth src srcSize off len : UInt32) : m UInt32 := do
  if p >= strings then return missing
  let token ← be b cap p
  if token == 1 then propertyLoop b cap strings (← next b cap p) (depth + 1) src srcSize off len
  else if token == 2 then
    if depth == 0 then return missing
    propertyLoop b cap strings (← next b cap p) (depth - 1) src srcSize off len
  else
    if token == 3 && depth == 0 then
      let name ← be b cap (p + 8)
      if (← strlen b cap (strings + name)) == len &&
          (← equal b cap (strings + name) src srcSize off len) then return p
    propertyLoop b cap strings (← next b cap p) depth src srcSize off len

@[specialize] private partial def validName (src size off len i : UInt32) : m Bool := do
  if i == len then return true
  let c ← read8 src size (off + i)
  if c == 0 || c == 47 then return false
  validName src size off len (i + 1)

@[specialize] private partial def patchLoop (b cap recipe recipeSize count p i : UInt32) : m UInt32 := do
  if i == count then
    if p != recipeSize then return 10
    return (← validate b cap)
  if !contains recipeSize p 12 then return 10
  let pathSize ← be recipe recipeSize p
  let nameSize ← be recipe recipeSize (p + 4)
  let valueSize ← be recipe recipeSize (p + 8)
  let p := p + 12
  if pathSize > 512 || nameSize == 0 || nameSize > 127 || valueSize > 4096 ||
      !contains recipeSize p (pathSize + nameSize + valueSize) then return 10
  if !(← validName recipe recipeSize (p + pathSize) nameSize 0) then return 10
  let node ← ensureNode b cap recipe recipeSize p pathSize
  if node == missing then return 11
  let index ← nameIndex b cap recipe recipeSize (p + pathSize) nameSize
  if index == missing then return 11
  let found ← propertyLoop b cap (← be b cap 12) (← next b cap node) 0 recipe recipeSize (p + pathSize) nameSize
  let position ← if found == missing then next b cap node else pure found
  let oldSize ← if found == missing then pure 0 else do
    pure (12 + aligned (← be b cap (found + 4)))
  let newSize := 12 + aligned valueSize
  if (← splice b cap position oldSize newSize) != 0 then return 11
  let _ ← writeBE32 b cap position 3
  let _ ← writeBE32 b cap (position + 4) valueSize
  let _ ← writeBE32 b cap (position + 8) index
  let _ ← fill b cap (position + 12) (aligned valueSize) 0
  let _ ← copy b cap (position + 12) recipe recipeSize (p + pathSize + nameSize) valueSize
  patchLoop b cap recipe recipeSize count (aligned (p + pathSize + nameSize + valueSize)) (i + 1)

/-- Recipe: BE32 count, then BE32 path/name/value lengths, unterminated path,
    name, value, and padding to 4 bytes. Only property set/create is supported. -/
@[specialize]
def patch (b cap recipe recipeSize : UInt32) : m UInt32 := do
  if (← validate b cap) != 0 then return 1
  if !addressable recipe recipeSize || recipeSize < 4 || recipeSize > 0x100000 then return 10
  if b < recipe + recipeSize && recipe < b + cap then return 10
  let count ← be recipe recipeSize 0
  if count > 64 then return 10
  patchLoop b cap recipe recipeSize count 4 0

@[specialize] private partial def insertReserve (b cap start size p limit used strings : UInt32) : m UInt32 := do
  if !contains limit p 16 then return 5
  let a ← be b cap p
  let c ← be b cap (p + 4)
  let d ← be b cap (p + 8)
  let e ← be b cap (p + 12)
  if a != 0 || c != 0 || d != 0 || e != 0 then
    insertReserve b cap start size (p + 16) limit used strings
  else
    let _ ← copy b cap (p + 16) b cap p (used - p)
    let _ ← writeBE32 b cap p 0
    let _ ← writeBE32 b cap (p + 4) start
    let _ ← writeBE32 b cap (p + 8) 0
    let _ ← writeBE32 b cap (p + 12) size
    let _ ← writeBE32 b cap 4 (used + 16)
    let _ ← writeBE32 b cap 8 (limit + 16)
    let _ ← writeBE32 b cap 12 (strings + 16)
    return (← validate b cap)

/-- Add a 32-bit physical RAM reservation, retaining existing entries. -/
@[specialize]
def reserve (b cap start size : UInt32) : m UInt32 := do
  if (← validate b cap) != 0 || !addressable start size || size == 0 then return 1
  let strings ← be b cap 12
  let stringSize ← be b cap 32
  let structureOffset ← be b cap 8
  let used := strings + stringSize
  if !contains cap used 16 then return 9
  insertReserve b cap start size (← be b cap 16) structureOffset used strings

end LeanBoot.Fdt
