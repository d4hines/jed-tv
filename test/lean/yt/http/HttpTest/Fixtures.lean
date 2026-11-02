/-! Test inputs compiled into the binary (the transcript runs in QEMU with
no filesystem). Its inputs: tools/lean/yt/Fixtures.lean (http), under assets/. -/
namespace HttpTest.Fixtures
def searchRequest : String := include_str "../../../../../assets/build/lean-formats/http/search.request.b64"
def searchResponse : String := include_str "../../../../../assets/build/lean-formats/http/search.response.b64"
def searchBody : String := include_str "../../../../../assets/build/lean-formats/http/search.json.b64"
def thumbRequest : String := include_str "../../../../../assets/build/lean-formats/http/thumb.request.b64"
def thumbResponse : String := include_str "../../../../../assets/build/lean-formats/http/thumb.response.b64"
def thumbBody : String := include_str "../../../../../assets/build/lean-formats/http/mqdefault.jpg.b64"
end HttpTest.Fixtures
