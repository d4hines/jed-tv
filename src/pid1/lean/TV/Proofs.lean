import Std.Tactic.BVDecide
import TV.Menu
import TV.Settings

namespace TV.Menu

theorem clamp_bounded (p n : UInt32) (h : n ≠ 0) : clamp p n < n := by
  unfold clamp
  bv_decide

theorem clamp_identity (p n : UInt32) (h : p < n) : clamp p n = p := by
  unfold clamp
  bv_decide

theorem move_bounded (p n : UInt32) (direction : Bool) (h : n ≠ 0) :
    move p n direction < n := by
  unfold move clamp
  bv_decide

theorem repeated_has_no_effect (key repeated : UInt32) (h : repeated ≠ 0) :
    effect key repeated = 0 := by
  unfold effect
  simp [h]

end TV.Menu

namespace TV.Settings
 theorem changed_pin_stays_four_digits (current supplied next : UInt32)
     (h : current < 10000) : changePin current supplied next < 10000 := by
   unfold changePin
   split <;> simp_all
end TV.Settings

namespace TV.Settings
 theorem disable_removes_access (mask app : UInt32)
     (h : app = 1 ∨ app = 2 ∨ app = 3 ∨ app = 5 ∨ app = 6) :
     enabled (disable mask app) app = 0 := by
   have hz (bit : UInt32) : mask &&& (~~~bit) &&& bit = 0 := by bv_decide
   rcases h with rfl | rfl | rfl | rfl | rfl <;> simp [enabled, disable, TV.Apps.valid, hz]
end TV.Settings

namespace TV.Settings
 theorem retired_app_has_no_access (mask : UInt32) : enabled mask 4 = 0 := by
   simp [enabled, TV.Apps.valid]
 theorem retired_app_cannot_be_enabled (mask current supplied : UInt32) :
     enable mask 4 current supplied = mask := by
   simp [enable, TV.Apps.valid]
end TV.Settings

namespace TV.Apps
/-- The launch identity policy never assigns root UID/GID to a valid app.
Root-owned TEE files still rely on the separately tested Linux access checks. -/
theorem valid_app_identity_is_nonroot (app : UInt32) (h : valid app = true) :
    uid app ≠ 0 ∧ gid app ≠ 0 := by
  unfold uid valid gid at *
  bv_decide
end TV.Apps
