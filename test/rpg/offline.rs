use rpgtv::engine::Engine;
#[test]
fn original_lean_story_is_local_deterministic_and_restarts() {
    let mut endings = 0;
    let mut deaths = 0;
    let mut hash = 0xcbf29ce484222325u64;
    for seed in 1..=24 {
        let mut a = Engine::new(seed);
        let mut b = Engine::new(seed);
        assert_eq!(a.screen().phase, "title");
        for (choice, title) in [(0, "The Shore"), (0, "The Iron Door"), (1, "Wreckers!")] {
            a.choose(choice);
            b.choose(choice);
            assert_eq!(a.screen(), b.screen());
            assert_eq!(a.screen().heading, title);
        }
        let before = a.screen();
        a.choose(999999);
        assert_eq!(a.screen(), before);
        a.choose(0);
        b.choose(0);
        assert_eq!(a.screen().phase, "fight");
        assert_eq!(a.screen().foes.len(), 2);
        for _ in 0..200 {
            let view = a.screen();
            assert_eq!(view, b.screen());
            for byte in format!("{seed}:{view:?}\n").bytes() {
                hash = (hash ^ byte as u64).wrapping_mul(0x100000001b3);
            }
            match view.phase.as_str() {
                "dead" => {
                    deaths += 1;
                    break;
                }
                "ended" => {
                    endings += 1;
                    break;
                }
                _ => {
                    assert!(!view.options.is_empty());
                    a.choose(0);
                    b.choose(0);
                }
            }
        }
        assert!(matches!(a.screen().phase.as_str(), "ended" | "dead"));
        a.choose(0);
        assert_eq!(a.screen().phase, "title");
    }
    assert_eq!(endings + deaths, 24);
    println!("TRANSCRIPT {hash:016x}");
    println!("Offline original Lean engine: {endings} endings, {deaths} deaths, deterministic paired sessions, invalid-input preservation and restart");
}
