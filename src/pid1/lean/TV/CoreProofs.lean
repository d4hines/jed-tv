prelude
import Init.Tactics
import Init.Grind

/-! How the theorems in `TV/Core.lean` are proved. Each theorem there ends in
`by cases_on_step`, `by cases_on_apps` and so on; those names are defined
here. Lean checks every proof when it builds `TV.Core`, so you can trust the
statements without reading this file.

Every method either proves its theorem or fails the build: none falls back to
`sorry`. -/

namespace TV.Core

-- The methods name `step`, `run` and friends, which `TV/Core.lean` defines
-- after this file; without hygiene they resolve where they are used.
set_option hygiene false

/-- Close a goal about one step once every branch is split: unfold the policy
    and state helpers and simplify. -/
macro "step_simp" : tactic => `(tactic|
  simp_all [Policy.allow, Policy.grant, Policy.allows, effective, State.alive,
    State.restartOffered, NetState.changing, State.awaits, manifest, Grants.set, Grants.has,
    Capability.isNetwork, Manifest.declares])

/-- Unfold `step` and check every branch. Split on which input it is (and
    which key or choice), unfold the handler, split every `if` and `match`,
    then simplify; what is left needs a case split on an app, an equation
    substituted, or `grind`. -/
macro "cases_on_step" : tactic => `(tactic| first
  | (intro s i
     cases i <;> (first | (cases ‹Key›) | (cases ‹Choice›) | skip)
     all_goals (
       simp only [step, onHome, onKey, onRelease, onTick, onExit, onChoice, onAsk, onNetDone, commit]
       (repeat' split) <;> intros <;> (try step_simp) <;> (try intros) <;> (try subst_vars)
         <;> (try step_simp) <;> (try (cases ‹App› <;> step_simp))
         <;> (try (intro h; subst h; step_simp))
         <;> grind [Policy.allow, Policy.grant, Policy.allows, effective, State.alive,
               State.restartOffered, NetState.changing, State.awaits, manifest, Grants.set,
               Grants.has, Capability.isNetwork, Manifest.declares]))
  | (intro s home online
     simp [step, onChoice, home, online]; done))

/-- Check the statement for every app (or every pair of apps) in the table,
    and for the native table every grant and display as well. -/
macro "cases_on_apps" : tactic => `(tactic| first
  | (intro a b h; cases a <;> cases b <;> simp [manifest] at h ⊢ <;> omega; done)
  | (intro a; cases a <;> first | rfl | (simp [manifest] <;> decide); done)
  | (intro a g d
     rcases g with ⟨i, l, s⟩
     cases a <;> cases d <;> cases i <;> cases l <;> cases s <;> decide))

/-- Unfold the policy's arithmetic and check every app and capability. -/
macro "cases_on_policy" : tactic => `(tactic| first
  | (intro p a c; cases a <;> cases c <;> simp [effective, Manifest.declares, manifest, Grants.has] <;>
       intros <;> assumption; done)
  | (intro p a h; cases a <;> simp [effective, Manifest.declares, manifest, Capability.isNetwork, h]; done)
  | (intro p d a h; cases a <;> simp [confine, appRow, effective, Manifest.declares, manifest,
       Capability.isNetwork, Grants.has, Net.none, h]; done))

/-- Unfold a sandbox row as far as it goes. -/
macro "row_simp" : tactic => `(tactic|
  simp [confine, appRow, daemonRow, teeRow, wifiRow, dhcpRow, manifest, uidBase, appMounts,
    appDevices, dataDir, effective, Net.none, Manifest.declares, Grants.has, Capability.isNetwork])

/-- Unfold a row, split on whether saved data is switched on, and decide what
    is left (comparisons of literal paths and numbers). -/
macro "row_finish" : tactic => `(tactic| (row_simp <;> (repeat' split) <;> (try simp) <;> decide))

macro "cases_on_subject" : tactic => `(tactic| (cases x <;> (try cases ‹App›) <;> (try cases ‹Daemon›)))

/-- Check every sandbox row (or every pair of rows). For fewer grants, compare
    the two rows field by field: only the data directory's source, the hosts
    and the LAN flag depend on the grants. -/
macro "cases_on_rows" : tactic => `(tactic| first
  | (intro a g g' d h
     have hi := h .internet
     have hl := h .localNetwork
     have hs := h .savedData
     simp only [Grants.has] at hi hl hs
     cases a <;> cases d <;> simp only [Confinement.within, appRow, appMounts, manifest, Manifest.declares, Grants.has] <;>
       rcases g with ⟨i, l, s⟩ <;> rcases g' with ⟨i', l', s'⟩ <;> cases s <;> cases s' <;> simp_all; done)
  | (intro p d x
     cases d <;> cases_on_subject <;> (try row_finish) <;> (intro x; cases_on_subject <;> row_finish)))

/-- Follow the parser: three words, the first `connect`, a port that
    `parsePort` accepts (so 1 to 65535) and a host `validHost` accepts. -/
macro "cases_on_parse" : tactic => `(tactic| (
  intro bytes host port h
  have portInRange : ∀ (ds : List Char) (n : Nat), parsePort ds = some n → 0 < n ∧ n < 65536 := by
    intro ds n hn
    simp only [parsePort] at hn
    split at hn
    · exact absurd hn (by simp)
    · split at hn
      · rename_i hc
        simp at hn hc
        omega
      · exact absurd hn (by simp)
  simp only [parseRequest] at h
  split at h
  · split at h
    · split at h
      · split at h
        · rename_i valid
          simp only [Option.some.injEq, Request.connect.injEq] at h
          obtain ⟨rfl, rfl⟩ := h
          exact ⟨valid, portInRange _ _ (by assumption)⟩
        · exact absurd h (by simp)
      · exact absurd h (by simp)
    · exact absurd h (by simp)
  · exact absurd h (by simp)))

/-- Induction over the run. Where the PIN changed, `pinChangeNeedsPin` says
    the input was `changePin` with the PIN current at that point. -/
macro "induction_on_run" : tactic => `(tactic| (
  intro s inputs h
  induction inputs generalizing s with
  | nil => simp [run] at h
  | cons i is ih =>
    by_cases first : (step s i).1.policy.pin = s.policy.pin
    · have ⟨before, after, new, eq⟩ := ih (step s i).1 (by simpa [run, first] using h)
      exact ⟨i :: before, after, new, by simp [run, eq]⟩
    · have ⟨new, eq⟩ := pinChangeNeedsPin s i first
      exact ⟨[], is, new, by simp [run, eq]⟩))

/-- Induction over a run from start-up with the TV offline. Starting offline
    emits nothing; after that, the first step that asks for the link to come
    up is, by `linkUpOnlyWithPin`, the user going online with the PIN current
    at that point. -/
macro "induction_on_boot" : tactic => `(tactic| (
  intro saved inputs op offline h
  have fromAnyState : ∀ (s : State) (inputs : List Input), .network op true ∈ runOutputs s inputs →
      ∃ before after, inputs = before ++ .choose (.goOnline (run s before).policy.pin) :: after := by
    intro s inputs h
    induction inputs generalizing s with
    | nil => simp [runOutputs] at h
    | cons i is ih =>
      simp only [runOutputs, List.mem_append] at h
      cases h with
      | inl here => exact ⟨[], is, by simp [run, linkUpOnlyWithPin s i op here]⟩
      | inr later =>
        have ⟨before, after, eq⟩ := ih (step s i).1 later
        exact ⟨i :: before, after, by simp [run, eq]⟩
  simp [bootOutputs, offline] at h
  exact fromAnyState _ _ h))

/-- Every writable mount of every app, against every mount of the TEE helper. -/
macro "cases_on_shared_mounts" : tactic => `(tactic| (
  intro p d a m m' hm hw hm'
  cases a <;> cases d <;>
    simp [confine, appRow, appMounts, teeRow, dataDir, manifest, uidBase, effective,
      Manifest.declares, Grants.has] at hm hm' <;>
    (repeat' rcases hm with rfl | hm) <;> (repeat' rcases hm' with rfl | hm') <;> simp_all <;>
    (try split) <;> (first | decide | simp_all | (intro h; injection h with h; revert h; decide))))

/-- Unfold the Linux model over every row. -/
macro "cases_on_permits" : tactic => `(tactic| (intros; rename_i a; cases a <;> simp_all [permits,
  confine, appRow, appMounts, appDevices, daemonRow, teeRow, wifiRow, dhcpRow, dataDir, manifest,
  uidBase, effective, under, Manifest.declares, Grants.has, Target.uid, Capability.isNetwork]))

/-- Writes go only through writable mounts; for an app that is its data
    directory, if saved data is in effect. -/
macro "writes_stay_in_own_data" : tactic => `(tactic| (
  intro p d a path h
  simp only [permits, confine, appRow, appMounts] at h
  cases hb : ((effective p a).has .savedData && (manifest a).declares .savedData) <;>
    simp only [hb] at h <;> cases d <;> cases a <;> simp_all))

/-- No app's device table has the TEE or eMMC nodes. -/
macro "devices_by_table" : tactic => `(tactic| (
  intro p d a w
  cases w <;> cases d <;> cases a <;> simp [permits, confine, appRow, appDevices, under] <;> decide +kernel))

/-- An app's own sockets follow its `lan` part, which follows its effective grants. -/
macro "connections_by_grants" : tactic => `(tactic| (
  intro p d a to h
  cases a <;> simp_all [permits, confine, appRow, effective, Manifest.declares, manifest,
    Grants.has, Capability.isNetwork]))

/-- Signals reach only the same user, and every process has its own user. -/
macro "signals_by_users" : tactic => `(tactic| (
  intro p d x to h
  simp only [permits] at h
  cases to with
  | pid1 => cases x <;> rename_i y <;> cases y <;> simp_all [Target.uid, confine, appRow,
      daemonRow, teeRow, wifiRow, dhcpRow, manifest, uidBase]
  | subject y =>
    have := subjectsHaveTheirOwnUsers p d y x (by simpa [Target.uid] using h)
    simp [this]))

/-- The TEE helper's only writable mount is its store. -/
macro "tee_writes_by_table" : tactic => `(tactic| (
  intro p d path h
  simp_all [permits, confine, daemonRow, teeRow]))

end TV.Core
