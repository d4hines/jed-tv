import TV.Audio.Proofs

/-! The main theorems' axioms, printed fresh on every check. Allowed: propext,
Classical.choice, Quot.sound, and the certificate axioms bv_decide and
native_decide add (`*._native.*`). Anything else, in particular sorryAx, fails
test/lean/tv/check-audio.py. -/
#print axioms TV.Audio.Gain.scale_never_exceeds_ceiling
#print axioms TV.Audio.Gain.zero_gain_is_silent
#print axioms TV.Audio.Gain.gain_above_ceiling_is_capped
#print axioms TV.Audio.Resample.output_frames_match_rate
#print axioms TV.Audio.Resample.feed_48k_is_identity
#print axioms TV.Audio.Resample.chunks_do_not_change_output
#print axioms TV.Audio.Resample.consumed_is_inverse
#print axioms TV.Audio.Session.Order.cleanup_in_order
#print axioms TV.Audio.Session.Order.poisoned_lease_refuses_open
