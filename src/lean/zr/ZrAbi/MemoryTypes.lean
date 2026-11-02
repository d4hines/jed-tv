import ZrAbi.C

namespace ZrAbi.MemoryTypes
open C

/- A raw address may designate storage whose C effective type is a pointer,
   scalar, float or object header. Compiler alias annotations preserve these
   machine-memory semantics when raw and typed field accesses are mixed. -/
private def types : List (String × String) := [
  ("uint8_t","zr_alias_u8"), ("uint16_t","zr_alias_u16"),
  ("uint32_t","zr_alias_u32"), ("uint64_t","zr_alias_u64"),
  ("size_t","zr_alias_word"), ("float","zr_alias_f32"), ("double","zr_alias_f64")
]

def rawType (type : CType) : CType :=
  match type with
  | .named name => match types.find? (·.1 == name) with
    | some (_,aliasName) => .named aliasName
    | none => type
  | _ => type

def header : Header := [.guard "ZR_RAW_MEMORY_TYPES" (types.map fun (type,name) =>
  .typedef (.named type) name [.mayAlias])]

end ZrAbi.MemoryTypes
