prelude
import Init.Data.UInt
import TV.Apps
set_option compiler.extract_closed false

/- PID 1 owns these values. Apps receive neither the PIN nor a settings writer.
   Four decimal digits are represented as a number in [0,10000). -/
namespace TV.Settings
@[export tv_settings_change_pin]
def changePin (current supplied next : UInt32) : UInt32 :=
  if supplied == current && next < 10000 then next else current

@[export tv_settings_enabled]
def enabled (mask app : UInt32) : UInt32 :=
  if TV.Apps.valid app && (mask &&& (1 <<< (app - 1))) != 0 then 1 else 0

@[export tv_settings_disable]
def disable (mask app : UInt32) : UInt32 :=
  if TV.Apps.valid app then mask &&& (~~~(1 <<< (app - 1))) else mask

@[export tv_settings_enable]
def enable (mask app current supplied : UInt32) : UInt32 :=
  if supplied == current && TV.Apps.valid app then mask ||| (1 <<< (app - 1)) else mask

theorem wrong_pin_cannot_change (current supplied next : UInt32)
    (h : supplied ≠ current) : changePin current supplied next = current := by
  simp [changePin, h]

theorem wrong_pin_cannot_enable (mask app current supplied : UInt32)
    (h : supplied ≠ current) : enable mask app current supplied = mask := by
  simp [enable, h]

theorem invalid_pin_cannot_replace (current supplied next : UInt32)
    (h : ¬ next < 10000) : changePin current supplied next = current := by
  simp [changePin, h]

theorem correct_pin_changes (current next : UInt32) (h : next < 10000) :
    changePin current current next = next := by
  simp [changePin, h]
end TV.Settings
