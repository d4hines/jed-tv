//! The YouTube protocol found in Stage 0 (research/yt-stage0/PROTOCOL.md):
//! POST /youtubei/v1/player twice (the first refusal hands out visitorData),
//! then plain ranged GETs on the returned URLs.

use crate::http;
use serde_json::{json, Value};
use std::io;

const UA: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 15_7_3) AppleWebKit/605.1.15 \
                  (KHTML, like Gecko) Version/26.0 Safari/605.1.15";

#[derive(Debug, Clone)]
pub struct Format {
    pub itag: u32,
    pub url: String,
    pub mime: String,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub bitrate: u64,
    pub content_length: u64,
    pub init_range: (u64, u64),
    pub index_range: (u64, u64),
}

pub struct Video {
    pub title: String,
    pub length_s: u64,
    pub formats: Vec<Format>,
}

fn player_body(video: &str, visitor: Option<&str>) -> Vec<u8> {
    let mut client = json!({
        "clientName": "VISIONOS", "clientVersion": "1.02",
        "deviceMake": "Apple", "deviceModel": "RealityDevice17,1",
        "osName": "visionOS", "osVersion": "26.5.23O471", "hl": "en", "userAgent": UA,
    });
    if let Some(v) = visitor {
        client["visitorData"] = json!(v);
    }
    json!({"context": {"client": client}, "videoId": video,
           "contentCheckOk": true, "racyCheckOk": true}).to_string().into_bytes()
}

fn post_player(base: &str, video: &str, visitor: Option<&str>) -> io::Result<Value> {
    let url = format!("{base}/youtubei/v1/player?prettyPrint=false");
    let r = http::request("POST", &url,
                          &[("Content-Type", "application/json"), ("User-Agent", UA)],
                          &player_body(video, visitor))?;
    if r.status != 200 {
        return Err(io::Error::other(format!("player HTTP {}", r.status)));
    }
    serde_json::from_slice(&r.body).map_err(io::Error::other)
}

fn range(v: &Value) -> (u64, u64) {
    let n = |k: &str| v[k].as_str().and_then(|s| s.parse().ok()).unwrap_or(0);
    (n("start"), n("end"))
}

/// Resolve a video ID to its adaptive formats with direct URLs.
pub fn resolve(base: &str, video: &str) -> io::Result<Video> {
    let mut reply = post_player(base, video, None)?;
    if reply["playabilityStatus"]["status"] != "OK" {
        let visitor = reply["responseContext"]["visitorData"].as_str()
            .ok_or_else(|| io::Error::other("refused and no visitorData"))?.to_owned();
        reply = post_player(base, video, Some(&visitor))?;
    }
    let status = &reply["playabilityStatus"];
    if status["status"] != "OK" {
        return Err(io::Error::other(format!("not playable: {} {}", status["status"], status["reason"])));
    }
    let u = |v: &Value| v.as_u64().unwrap_or(0) as u32;
    let formats = reply["streamingData"]["adaptiveFormats"].as_array().into_iter().flatten()
        .filter_map(|f| Some(Format {
            itag: f["itag"].as_u64()? as u32,
            url: f["url"].as_str()?.to_owned(),
            mime: f["mimeType"].as_str().unwrap_or("").to_owned(),
            width: u(&f["width"]),
            height: u(&f["height"]),
            fps: u(&f["fps"]),
            bitrate: f["bitrate"].as_u64().unwrap_or(0),
            content_length: f["contentLength"].as_str().and_then(|s| s.parse().ok()).unwrap_or(0),
            init_range: range(&f["initRange"]),
            index_range: range(&f["indexRange"]),
        }))
        .collect();
    let d = &reply["videoDetails"];
    Ok(Video {
        title: d["title"].as_str().unwrap_or("").to_owned(),
        length_s: d["lengthSeconds"].as_str().and_then(|s| s.parse().ok()).unwrap_or(0),
        formats,
    })
}

/// The team's target: H.264 video in MP4, at most `max_height`, highest such.
pub fn pick_video(formats: &[Format], max_height: u32) -> Option<&Format> {
    formats.iter()
        .filter(|f| f.mime.starts_with("video/mp4") && f.mime.contains("avc1")
                && f.width > 0 && f.width <= 1280 && f.height > 0
                && f.height <= max_height && f.fps > 0 && f.fps <= 30)
        .max_by_key(|f| (f.height, f.bitrate))
}

/// AAC in MP4 (itag 140 in practice).
pub fn pick_audio(formats: &[Format]) -> Option<&Format> {
    formats.iter()
        .filter(|f| f.mime.starts_with("audio/mp4") && f.mime.contains("mp4a"))
        .max_by_key(|f| f.bitrate)
}

/// Ranged GET of bytes [start, end] (inclusive) from a format URL.
pub fn fetch_range(url: &str, start: u64, end: u64) -> io::Result<Vec<u8>> {
    let r = http::request("GET", url, &[("Range", &format!("bytes={start}-{end}"))], &[])?;
    if r.status != 206 && r.status != 200 {
        return Err(io::Error::other(format!("media HTTP {}", r.status)));
    }
    Ok(r.body)
}

/// One search result.
#[derive(Debug, Clone)]
pub struct SearchItem {
    pub id: String,
    pub title: String,
    pub channel: String,
    pub length: String,
}

impl SearchItem {
    /// YouTube's fixed-URL 320x180 thumbnail (16:9, no letterbox bars).
    pub fn thumb_url(&self) -> String {
        format!("https://i.ytimg.com/vi/{}/mqdefault.jpg", self.id)
    }
}

fn text_of(v: &Value) -> String {
    v["simpleText"].as_str().map(str::to_owned)
        .or_else(|| v["runs"].as_array().map(|r| r.iter().filter_map(|x| x["text"].as_str()).collect()))
        .unwrap_or_default()
}

fn collect_videos(v: &Value, out: &mut Vec<SearchItem>) {
    match v {
        Value::Object(m) => {
            if let Some(r) = m.get("videoRenderer") {
                if let Some(id) = r["videoId"].as_str() {
                    out.push(SearchItem {
                        id: id.to_owned(),
                        title: text_of(&r["title"]),
                        channel: text_of(&r["ownerText"]),
                        length: text_of(&r["lengthText"]),
                    });
                }
            }
            m.values().for_each(|x| collect_videos(x, out));
        }
        Value::Array(a) => a.iter().for_each(|x| collect_videos(x, out)),
        _ => {}
    }
}

/// Anonymous search: one POST to /youtubei/v1/search with the WEB client
/// identity (verified 2026-10-31: no visitor ID or token needed; the WEB
/// reply includes video lengths). Live streams (no length) are skipped.
pub fn search(base: &str, query: &str) -> io::Result<Vec<SearchItem>> {
    let body = json!({
        "context": {"client": {"clientName": "WEB", "clientVersion": "2.20260708.00.00",
                               "hl": "en", "gl": "US"}},
        "query": query,
    }).to_string();
    let url = format!("{base}/youtubei/v1/search?prettyPrint=false");
    let r = http::request("POST", &url, &[("Content-Type", "application/json"), ("User-Agent", UA)],
                          body.as_bytes())?;
    if r.status != 200 {
        return Err(io::Error::other(format!("search HTTP {}", r.status)));
    }
    let v: Value = serde_json::from_slice(&r.body).map_err(io::Error::other)?;
    let mut out = Vec::new();
    collect_videos(&v, &mut out);
    out.retain(|i| !i.length.is_empty());
    let mut seen = std::collections::HashSet::new();
    out.retain(|i| seen.insert(i.id.clone()));
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn selection_respects_both_panel_dimensions_and_frame_rate() {
        let good = Format { itag: 136, url: String::new(), mime: "video/mp4; codecs=avc1".into(),
            width: 1280, height: 720, fps: 30, bitrate: 1, content_length: 1,
            init_range: (0, 0), index_range: (0, 0) };
        for (width, height, fps) in [(3840, 720, 30), (1280, 720, 60), (0, 720, 30),
                                     (1280, 0, 30), (1280, 720, 0)] {
            let invalid = Format { width, height, fps, bitrate: 100, ..good.clone() };
            let formats = [invalid, good.clone()];
            assert_eq!(pick_video(&formats, 720).unwrap().bitrate, 1);
        }
        assert!(pick_video(&[good], 480).is_none());
    }
}
