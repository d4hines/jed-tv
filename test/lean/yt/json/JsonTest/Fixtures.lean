/-! Test inputs compiled into the binary (the transcript runs in QEMU with
no filesystem). Its inputs: tools/lean/yt/Fixtures.lean (json), under assets/. -/
namespace JsonTest.Fixtures
def player1Json : String := include_str "../../../../../assets/build/lean-formats/json/player1.json"
def player1Canon : String := include_str "../../../../../assets/build/lean-formats/json/player1.canon"
def player1Expect : String := include_str "../../../../../assets/build/lean-formats/json/player1.expect"
def player2Json : String := include_str "../../../../../assets/build/lean-formats/json/player2.json"
def player2Canon : String := include_str "../../../../../assets/build/lean-formats/json/player2.canon"
def player2Expect : String := include_str "../../../../../assets/build/lean-formats/json/player2.expect"
def player3Json : String := include_str "../../../../../assets/build/lean-formats/json/player3.json"
def player3Canon : String := include_str "../../../../../assets/build/lean-formats/json/player3.canon"
def player3Expect : String := include_str "../../../../../assets/build/lean-formats/json/player3.expect"
def player4Json : String := include_str "../../../../../assets/build/lean-formats/json/player4.json"
def player4Canon : String := include_str "../../../../../assets/build/lean-formats/json/player4.canon"
def player4Expect : String := include_str "../../../../../assets/build/lean-formats/json/player4.expect"
def player5Json : String := include_str "../../../../../assets/build/lean-formats/json/player5.json"
def player5Canon : String := include_str "../../../../../assets/build/lean-formats/json/player5.canon"
def player5Expect : String := include_str "../../../../../assets/build/lean-formats/json/player5.expect"
def player6Json : String := include_str "../../../../../assets/build/lean-formats/json/player6.json"
def player6Canon : String := include_str "../../../../../assets/build/lean-formats/json/player6.canon"
def player6Expect : String := include_str "../../../../../assets/build/lean-formats/json/player6.expect"
def player7Json : String := include_str "../../../../../assets/build/lean-formats/json/player7.json"
def player7Canon : String := include_str "../../../../../assets/build/lean-formats/json/player7.canon"
def player7Expect : String := include_str "../../../../../assets/build/lean-formats/json/player7.expect"
def player8Json : String := include_str "../../../../../assets/build/lean-formats/json/player8.json"
def player8Canon : String := include_str "../../../../../assets/build/lean-formats/json/player8.canon"
def player8Expect : String := include_str "../../../../../assets/build/lean-formats/json/player8.expect"
def player9Json : String := include_str "../../../../../assets/build/lean-formats/json/player9.json"
def player9Canon : String := include_str "../../../../../assets/build/lean-formats/json/player9.canon"
def player9Expect : String := include_str "../../../../../assets/build/lean-formats/json/player9.expect"
def player10Json : String := include_str "../../../../../assets/build/lean-formats/json/player10.json"
def player10Canon : String := include_str "../../../../../assets/build/lean-formats/json/player10.canon"
def player10Expect : String := include_str "../../../../../assets/build/lean-formats/json/player10.expect"
def player11Json : String := include_str "../../../../../assets/build/lean-formats/json/player11.json"
def player11Canon : String := include_str "../../../../../assets/build/lean-formats/json/player11.canon"
def player11Expect : String := include_str "../../../../../assets/build/lean-formats/json/player11.expect"
def searchJson : String := include_str "../../../../../assets/build/lean-formats/json/search.json"
def searchCanon : String := include_str "../../../../../assets/build/lean-formats/json/search.canon"
def searchExpect : String := include_str "../../../../../assets/build/lean-formats/json/search.expect"
def all : Array (String × String × String × String) := #[
  ("player1", player1Json, player1Canon, player1Expect),
  ("player2", player2Json, player2Canon, player2Expect),
  ("player3", player3Json, player3Canon, player3Expect),
  ("player4", player4Json, player4Canon, player4Expect),
  ("player5", player5Json, player5Canon, player5Expect),
  ("player6", player6Json, player6Canon, player6Expect),
  ("player7", player7Json, player7Canon, player7Expect),
  ("player8", player8Json, player8Canon, player8Expect),
  ("player9", player9Json, player9Canon, player9Expect),
  ("player10", player10Json, player10Canon, player10Expect),
  ("player11", player11Json, player11Canon, player11Expect),
  ("search", searchJson, searchCanon, searchExpect)]
def cases : String := include_str "../../../../../assets/build/lean-formats/json/cases.b64"
def casesExpect : String := include_str "../../../../../assets/build/lean-formats/json/cases.expect"
def bodies : String := include_str "../../../../../assets/build/lean-formats/json/bodies.expect"
end JsonTest.Fixtures
