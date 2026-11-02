/- CPU-owned runtime memory primitives. Their implementations are emitted by
   the typed Lean ABI generator. These are nonvolatile heap accesses; device
   registers and DMA continue to use Low.Mem's ordered volatile accesses. -/
namespace ZrLean.Low.HeaderMem

@[extern "zr_header_ld8"] opaque ld8 (a : USize) : BaseIO UInt8
@[extern "zr_header_ld16"] opaque ld16 (a : USize) : BaseIO UInt16
@[extern "zr_header_ld32"] opaque ld32 (a : USize) : BaseIO UInt32
@[extern "zr_header_ld64"] opaque ld64 (a : USize) : BaseIO UInt64
@[extern "zr_header_ldp"] opaque ldp (a : USize) : BaseIO USize
@[extern "zr_header_ldf64"] opaque ldf64 (a : USize) : BaseIO Float
@[extern "zr_header_ldf32"] opaque ldf32 (a : USize) : BaseIO Float32
@[extern "zr_header_st8"] opaque st8 (a : USize) (v : UInt8) : BaseIO Unit
@[extern "zr_header_st16"] opaque st16 (a : USize) (v : UInt16) : BaseIO Unit
@[extern "zr_header_st32"] opaque st32 (a : USize) (v : UInt32) : BaseIO Unit
@[extern "zr_header_st64"] opaque st64 (a : USize) (v : UInt64) : BaseIO Unit
@[extern "zr_header_stp"] opaque stp (a v : USize) : BaseIO Unit
@[extern "zr_header_stf64"] opaque stf64 (a : USize) (v : Float) : BaseIO Unit
@[extern "zr_header_stf32"] opaque stf32 (a : USize) (v : Float32) : BaseIO Unit

end ZrLean.Low.HeaderMem
