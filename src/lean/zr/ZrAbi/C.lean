/- Typed C syntax for generated freestanding interoperability headers.
   This module has no raw declaration, statement or function-body escape. -/
namespace ZrAbi.C

inductive CType where
  | named (name : String)
  | ptr (type : CType)
  | const (type : CType)
  | volatile (type : CType)
  | atomic (type : CType)
  | array (type : CType) (length : Option Nat)
  | fnPtr (result : CType) (args : List CType)

inductive UnaryOp where
  | neg | pos | not | bitNot | addr | deref | preInc | preDec | postInc | postDec

inductive BinaryOp where
  | add | sub | mul | div | mod | eq | ne | lt | le | gt | ge
  | and | or | bitAnd | bitOr | bitXor | shl | shr | assign
  | addAssign | subAssign | mulAssign | andAssign | orAssign | xorAssign | shlAssign | shrAssign

inductive Expr where
  | ident (name : String)
  | num (value : Nat) (suffix : String := "")
  | str (value : String)
  | call (function : Expr) (args : List Expr)
  | cast (type : CType) (value : Expr)
  | unary (op : UnaryOp) (value : Expr)
  | binary (op : BinaryOp) (lhs rhs : Expr)
  | member (value : Expr) (name : String)
  | arrow (value : Expr) (name : String)
  | index (value index : Expr)
  | cond (condition yes no : Expr)
  | sizeof (type : CType)
  | offsetof (type : CType) (field : String)
  | defined (name : String)
  | initializer (fields : List (Option String × Expr))
  | stringize (name : String)
  | splice (values : List Expr)

def join (separator : String) (values : List String) : String :=
  String.intercalate separator values

/-- Render a C declarator, including arrays and pointer-to-function fields. -/
partial def renderTypeDecl : CType → String → String
  | .named name, declarator => name ++ (if declarator.isEmpty then "" else " " ++ declarator)
  | .const (.ptr type), declarator => renderTypeDecl type ("*const " ++ declarator)
  | .const type, declarator => "const " ++ renderTypeDecl type declarator
  | .volatile (.ptr type), declarator => renderTypeDecl type ("*volatile " ++ declarator)
  | .volatile type, declarator => "volatile " ++ renderTypeDecl type declarator
  | .atomic type, declarator => "_Atomic(" ++ renderTypeDecl type "" ++ ") " ++ declarator
  | .ptr type, declarator =>
    let declarator := "*" ++ declarator
    match type with
    | .array .. => renderTypeDecl type ("(" ++ declarator ++ ")")
    | _ => renderTypeDecl type declarator
  | .array type length, declarator =>
    renderTypeDecl type (declarator ++ "[" ++ (length.map toString |>.getD "") ++ "]")
  | .fnPtr result args, declarator =>
    renderTypeDecl result ("(*" ++ declarator ++ ")(" ++
      (if args.isEmpty then "void" else join ", " (args.map (fun type => renderTypeDecl type ""))) ++ ")")

def renderType (type : CType) : String := renderTypeDecl type ""

def unaryToken : UnaryOp → String
  | .neg => "-" | .pos => "+" | .not => "!" | .bitNot => "~"
  | .addr => "&" | .deref => "*" | .preInc | .postInc => "++" | .preDec | .postDec => "--"

def binaryToken : BinaryOp → String
  | .add => "+" | .sub => "-" | .mul => "*" | .div => "/" | .mod => "%"
  | .eq => "==" | .ne => "!=" | .lt => "<" | .le => "<=" | .gt => ">" | .ge => ">="
  | .and => "&&" | .or => "||" | .bitAnd => "&" | .bitOr => "|" | .bitXor => "^"
  | .shl => "<<" | .shr => ">>" | .assign => "="
  | .addAssign => "+=" | .subAssign => "-=" | .mulAssign => "*="
  | .andAssign => "&=" | .orAssign => "|=" | .xorAssign => "^="
  | .shlAssign => "<<=" | .shrAssign => ">>="

def quote (text : String) : String :=
  let escaped := text.replace "\\" "\\\\" |>.replace "\"" "\\\"" |>.replace "\n" "\\n" |>.replace "\r" "\\r" |>.replace "\t" "\\t"
  "\"" ++ escaped ++ "\""

partial def renderExpr : Expr → String
  | .ident name => name
  | .num value suffix => toString value ++ suffix
  | .str text => quote text
  | .call function args => renderExpr function ++ "(" ++ join ", " (args.map renderExpr) ++ ")"
  | .cast type value => "((" ++ renderType type ++ ")(" ++ renderExpr value ++ "))"
  | .unary op value =>
    match op with
    | .postInc | .postDec => "((" ++ renderExpr value ++ ")" ++ unaryToken op ++ ")"
    | _ => "(" ++ unaryToken op ++ "(" ++ renderExpr value ++ "))"
  | .binary op lhs rhs => "(" ++ renderExpr lhs ++ " " ++ binaryToken op ++ " " ++ renderExpr rhs ++ ")"
  | .member value name => "(" ++ renderExpr value ++ ")." ++ name
  | .arrow value name => "(" ++ renderExpr value ++ ")->" ++ name
  | .index value index => "(" ++ renderExpr value ++ ")[" ++ renderExpr index ++ "]"
  | .cond condition yes no => "(" ++ renderExpr condition ++ " ? " ++ renderExpr yes ++ " : " ++ renderExpr no ++ ")"
  | .sizeof type => "sizeof(" ++ renderType type ++ ")"
  | .offsetof type field => "__builtin_offsetof(" ++ renderType type ++ ", " ++ field ++ ")"
  | .defined name => "defined(" ++ name ++ ")"
  | .initializer fields => "{ " ++ join ", " (fields.map fun (name, value) =>
      (name.map (fun n => "." ++ n ++ " = ") |>.getD "") ++ renderExpr value) ++ " }"
  | .stringize name => "#" ++ name
  | .splice values => join ", " (values.map renderExpr)

structure Field where
  name : String
  type : CType
  width : Option Nat := none

structure Param where
  name : String
  type : CType

inductive Stmt where
  | return_ (value : Option Expr)
  | expr (value : Expr)
  | local (type : CType) (name : String) (value : Option Expr := none)
  | if_ (condition : Expr) (yes no : List Stmt)
  | while_ (condition : Expr) (body : List Stmt)
  | for_ (init : Option Expr) (condition : Option Expr) (step : Option Expr) (body : List Stmt)
  | block (body : List Stmt)
  | break_
  | continue_

def indent (level : Nat) : String := String.ofList (List.replicate (level * 2) ' ')

partial def renderStmt (level : Nat) : Stmt → String
  | .return_ value => indent level ++ "return" ++ (value.map (fun x => " " ++ renderExpr x) |>.getD "") ++ ";\n"
  | .expr value => indent level ++ renderExpr value ++ ";\n"
  | .local type name value => indent level ++ renderTypeDecl type name ++
      (value.map (fun x => " = " ++ renderExpr x) |>.getD "") ++ ";\n"
  | .if_ condition yes no =>
      indent level ++ "if (" ++ renderExpr condition ++ ") {\n" ++
      join "" (yes.map (renderStmt (level+1))) ++ indent level ++ "}" ++
      (if no.isEmpty then "\n" else " else {\n" ++ join "" (no.map (renderStmt (level+1))) ++ indent level ++ "}\n")
  | .while_ condition body =>
      indent level ++ "while (" ++ renderExpr condition ++ ") {\n" ++
      join "" (body.map (renderStmt (level+1))) ++ indent level ++ "}\n"
  | .for_ init condition step body =>
      indent level ++ "for (" ++ (init.map renderExpr |>.getD "") ++ "; " ++
      (condition.map renderExpr |>.getD "") ++ "; " ++ (step.map renderExpr |>.getD "") ++ ") {\n" ++
      join "" (body.map (renderStmt (level+1))) ++ indent level ++ "}\n"
  | .block body => indent level ++ "{\n" ++ join "" (body.map (renderStmt (level+1))) ++ indent level ++ "}\n"
  | .break_ => indent level ++ "break;\n"
  | .continue_ => indent level ++ "continue;\n"

inductive RecordKind where | struct_ | union_

inductive Storage where | plain | extern_ | static_ | staticInline

inductive Attribute where | packed | aligned (bytes : Nat) | noreturn | alwaysInline | malloc | weak | mayAlias

def renderAttribute : Attribute → String
  | .packed => "__attribute__((packed))"
  | .aligned n => "__attribute__((aligned(" ++ toString n ++ ")))"
  | .noreturn => "__attribute__((noreturn))"
  | .alwaysInline => "__attribute__((always_inline))"
  | .malloc => "__attribute__((malloc))"
  | .weak => "__attribute__((weak))"
  | .mayAlias => "__attribute__((__may_alias__))"

inductive Decl where
  | typedef (type : CType) (name : String) (attrs : List Attribute := [])
  | prototype (result : CType) (name : String) (params : List Param)
      (storage : Storage := .plain) (attrs : List Attribute := [])
  | function (storage : Storage) (result : CType) (name : String) (params : List Param) (body : List Stmt)
  | record (kind : RecordKind) (name : String) (fields : List Field) (typedefName : Option String := none) (attrs : List Attribute := [])
  | define (name : String) (value : Expr)
  | flag (name : String)
  | macro (name : String) (params : List String) (value : Expr)
  | include (name : String) (system : Bool := true)
  | pragmaOnce
  | ifdef (name : String) (yes no : List Decl)
  | if_ (condition : Expr) (yes no : List Decl)
  | guard (name : String) (body : List Decl)
  | forward (kind : RecordKind) (name : String)
  | attributeMacro (name : String) (attrs : List Attribute)
  | variable (storage : Storage) (type : CType) (name : String) (value : Option Expr := none)

def renderParams (params : List Param) : String :=
  if params.isEmpty then "void"
  else join ", " (params.map (fun p => renderTypeDecl p.type p.name))

partial def renderDecl : Decl → String
  | .typedef type name attrs => "typedef " ++ renderTypeDecl type name ++
      (if attrs.isEmpty then "" else " " ++ join " " (attrs.map renderAttribute)) ++ ";\n"
  | .prototype result name params storage attrs =>
      (match storage with | .plain => "" | .extern_ => "extern " | .static_ => "static " | .staticInline => "static inline ") ++
      (if attrs.isEmpty then "" else join " " (attrs.map renderAttribute) ++ " ") ++
      renderTypeDecl result (name ++ "(" ++ renderParams params ++ ")") ++ ";\n"
  | .function storage result name params body =>
      (match storage with | .plain => "" | .extern_ => "extern " | .static_ => "static " | .staticInline => "static inline ") ++
      renderTypeDecl result (name ++ "(" ++ renderParams params ++ ")") ++ " {\n" ++
      join "" (body.map (renderStmt 1)) ++ "}\n"
  | .record kind name fields typedefName attrs =>
      (if typedefName.isSome then "typedef " else "") ++
      (match kind with | .struct_ => "struct" | .union_ => "union") ++
      (if name.isEmpty then "" else " " ++ name) ++ " {\n" ++
      join "" (fields.map (fun f => "  " ++ renderTypeDecl f.type f.name ++
        (f.width.map (fun n => " : " ++ toString n) |>.getD "") ++ ";\n")) ++ "}" ++
      (if attrs.isEmpty then "" else " " ++ join " " (attrs.map renderAttribute)) ++
      (typedefName.map (fun n => " " ++ n) |>.getD "") ++ ";\n"
  | .define name value => "#define " ++ name ++ " " ++ renderExpr value ++ "\n"
  | .flag name => "#define " ++ name ++ "\n"
  | .macro name params value => "#define " ++ name ++ "(" ++ join ", " params ++ ") " ++ renderExpr value ++ "\n"
  | .include name system => "#include " ++ (if system then "<" ++ name ++ ">" else quote name) ++ "\n"
  | .pragmaOnce => "#pragma once\n"
  | .ifdef name yes no => "#ifdef " ++ name ++ "\n" ++ join "" (yes.map renderDecl) ++
      (if no.isEmpty then "" else "#else\n" ++ join "" (no.map renderDecl)) ++ "#endif\n"
  | .if_ condition yes no => "#if " ++ renderExpr condition ++ "\n" ++ join "" (yes.map renderDecl) ++
      (if no.isEmpty then "" else "#else\n" ++ join "" (no.map renderDecl)) ++ "#endif\n"
  | .guard name body => "#ifndef " ++ name ++ "\n#define " ++ name ++ "\n" ++
      join "" (body.map renderDecl) ++ "#endif\n"
  | .forward kind name => (match kind with | .struct_ => "struct " | .union_ => "union ") ++ name ++ ";\n"
  | .attributeMacro name attrs => "#define " ++ name ++ " " ++ join " " (attrs.map renderAttribute) ++ "\n"
  | .variable storage type name value =>
      (match storage with | .extern_ => "extern " | .static_ => "static " | _ => "") ++
      renderTypeDecl type name ++ (value.map (fun e => " = " ++ renderExpr e) |>.getD "") ++ ";\n"

abbrev Header := List Decl

def renderHeader (header : Header) : String :=
  "/* Generated from Lean typed declarations; edit the Lean source. */\n" ++ join "" (header.map renderDecl)

end ZrAbi.C
