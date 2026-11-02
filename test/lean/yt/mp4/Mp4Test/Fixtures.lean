/-! Test inputs compiled into the binary (the transcript runs in QEMU with
no filesystem). Its inputs: tools/lean/yt/Fixtures.lean (mp4), under assets/. -/
namespace Mp4Test.Fixtures
def m136 : String := include_str "../../../../../assets/build/lean-formats/mp4/136.mp4.b64"
def m140 : String := include_str "../../../../../assets/build/lean-formats/mp4/140.m4a.b64"
def d136 : String := include_str "../../../../../assets/build/lean-formats/mp4/136.dump"
def d140 : String := include_str "../../../../../assets/build/lean-formats/mp4/140.dump"
end Mp4Test.Fixtures
