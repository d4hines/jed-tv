prelude
import Init

/- Exercise the exact String/List boundaries used by the production codec.
Separate exports prevent the compiler folding the round trip away. -/
@[export settings_test_chars]
def chars (value : String) : List Char := value.toList

@[export settings_test_string]
def string (value : List Char) : String := String.ofList value
