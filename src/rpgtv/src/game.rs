//! Display data emitted by the in-process Lean adventure engine.
#[derive(Clone, Debug, PartialEq)]
pub struct Opt {
    pub label: String,
    pub help: String,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mood {
    Normal,
    Dead,
    Ended,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Bar {
    pub name: String,
    pub hp: i64,
    pub max: i64,
    pub alive: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Screen {
    pub phase: String,
    pub adventure: String,
    pub hero: Bar,
    pub heading: String,
    pub mood: Mood,
    pub body: String,
    pub picture: Option<String>,
    pub foes: Vec<Bar>,
    pub side_title: String,
    pub side: Vec<String>,
    pub options: Vec<Opt>,
    pub error: Option<String>,
}
impl Default for Screen {
    fn default() -> Self {
        Self {
            phase: String::new(),
            adventure: String::new(),
            hero: Bar {
                name: String::new(),
                hp: 0,
                max: 0,
                alive: true,
            },
            heading: String::new(),
            mood: Mood::Normal,
            body: String::new(),
            picture: None,
            foes: vec![],
            side_title: String::new(),
            side: vec![],
            options: vec![],
            error: None,
        }
    }
}
