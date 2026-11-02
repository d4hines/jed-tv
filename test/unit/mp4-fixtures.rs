//! Cross-check the demuxer against ffprobe on the Stage 0 YouTube fixtures.
//! Skips (passes) when the git-ignored fixtures or ffprobe are not available.

use std::path::PathBuf;
use std::process::Command;

fn fixtures_dir() -> Option<PathBuf> {
    if let Ok(d) = std::env::var("YT_FIXTURES") {
        return Some(PathBuf::from(d));
    }
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/yt-stage0");
    let mut dirs: Vec<_> = std::fs::read_dir(root).ok()?.flatten().map(|e| e.path())
        .filter(|p| p.file_name().is_some_and(|n| n.to_string_lossy().starts_with("record-")))
        .map(|p| p.join("fixtures")).filter(|p| p.is_dir()).collect();
    dirs.sort();
    dirs.pop()
}

/// (dts, size, keyframe) per packet, from ffprobe.
fn ffprobe_packets(path: &PathBuf) -> Option<Vec<(i64, usize, bool)>> {
    let out = Command::new("ffprobe")
        .args(["-v", "error", "-select_streams", "0", "-show_entries", "packet=dts,size,flags",
               "-of", "csv=p=0"]).arg(path).output().ok()?;
    if !out.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).lines().filter(|l| !l.is_empty()).map(|l| {
        let f: Vec<&str> = l.split(',').collect();
        (f[0].parse().unwrap(), f[1].parse().unwrap(), f[2].starts_with('K'))
    }).collect())
}

fn check(name: &str) {
    let Some(dir) = fixtures_dir() else { return eprintln!("skip: no fixtures") };
    let path = dir.join(name);
    let Ok(file) = std::fs::read(&path) else { return eprintln!("skip: {name} missing") };
    let Some(want) = ffprobe_packets(&path) else { return eprintln!("skip: ffprobe unavailable") };

    let init = mp4::parse_init(&file).expect("init");
    let index = mp4::parse_sidx(&file, 0).expect("sidx");
    let mut got = Vec::new();
    for s in &index.segments {
        let frag = &file[s.offset as usize..(s.offset + s.size as u64) as usize];
        got.extend(mp4::parse_fragments(frag, &init).expect("fragment"));
    }
    eprintln!("{name}: {:?} timescale {} segments {} samples {}", match &init.codec {
        mp4::Codec::Avc { width, height, profile, level, .. } => format!("avc {width}x{height} p{profile} l{level}"),
        mp4::Codec::Aac { channels, sample_rate, object_type, .. } => format!("aac ot{object_type} {sample_rate}Hz x{channels}"),
        mp4::Codec::Other(t) => format!("{t:?}"),
    }, init.timescale, index.segments.len(), got.len());
    eprintln!("{name}: edit offset {} ({:.4} s)", init.edit_offset,
              init.edit_offset as f64 / init.timescale as f64);

    assert_eq!(got.len(), want.len(), "sample count");
    let (d0, w0) = (got[0].dts as i64, want[0].0);
    for (i, (g, w)) in got.iter().zip(&want).enumerate() {
        assert_eq!(g.data.len(), w.1, "size of sample {i}");
        assert_eq!(g.dts as i64 - d0, w.0 - w0, "dts of sample {i}");
        assert_eq!(g.keyframe, w.2, "keyframe flag of sample {i}");
    }
    // Every segment's start time must equal the first sample time inside it.
    let mut i = 0;
    for s in &index.segments {
        let frag = &file[s.offset as usize..(s.offset + s.size as u64) as usize];
        let n = mp4::parse_fragments(frag, &init).unwrap();
        let first_pts = n.iter().map(|x| x.pts).min().unwrap();
        let scaled = s.start as i128 * init.timescale as i128 / index.timescale as i128;
        assert!((first_pts as i128 - scaled).abs() <= 1 + got[0].pts as i128 - got[0].dts as i128,
                "segment {i} start {scaled} vs first pts {first_pts}");
        assert!(s.starts_with_sap == n[0].keyframe, "segment {i} SAP flag");
        i += 1;
    }
}

#[test]
fn h264_720p_matches_ffprobe() {
    check("136.mp4");
}

#[test]
fn h264_144p_matches_ffprobe() {
    check("160.mp4");
}

#[test]
fn aac_matches_ffprobe() {
    check("140.m4a");
}
