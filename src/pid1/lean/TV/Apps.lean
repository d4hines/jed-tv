prelude
import Init.Data.UInt
set_option compiler.extract_closed false

/- Fixed application identities and capabilities. IDs are not supplied by apps;
   PID 1 selects one from its installed catalog at launch. -/
namespace TV.Apps
-- 1 ytplay, 2 doomtv, 3 slides, 5 rpgtv (Infinite RPG, offline), 6 flighttv (city flight),
-- 7 axiomtv (Axiom: First Light, a Lean voxel game), 8 snaketv (snake on a torus); ID 4 retired.
@[inline] def valid (app : UInt32) : Bool := app == 1 || app == 2 || app == 3 || app == 5 || app == 6 || app == 7 || app == 8
@[export tv_app_uid] def uid (app : UInt32) : UInt32 :=
  if valid app then 10000 + app else 0
-- An app's network, given its grants (1 internet, 2 local network, 4 saved
-- data), as two bits: 1 its own sockets (YouTube and Doom may have them),
-- 2 connections from PID 1 (YouTube only). TV.Core.appRow is the spec.
@[export tv_app_network] def network (app grants : UInt32) : UInt32 :=
  (if (app == 1 || app == 2) && grants &&& 2 != 0 then 1 else 0) |||
  (if app == 1 && grants &&& 1 != 0 then 2 else 0)
@[export tv_app_threads] def threads (app : UInt32) : UInt32 :=
  if app == 1 then 32 else if app == 2 then 8 else if app == 3 || app == 5 || app == 8 then 1
  else if app == 6 || app == 7 then 4 else 0
end TV.Apps

namespace TV.Apps
@[export tv_app_gid] def gid (_app : UInt32) : UInt32 := 4000
-- Limits are policy, not hardcoded decisions in the native launch mechanism.
@[export tv_app_limit] def limit (app which : UInt32) : UInt64 :=
  if which == 0 then 256 -- descriptor 1023 cannot be recreated after exec
  else if which == 1 then (threads app).toUInt64
  else if which == 2 then 768 * 1024 * 1024
  else if which == 3 then 64 * 1024 * 1024
  else 0
end TV.Apps
