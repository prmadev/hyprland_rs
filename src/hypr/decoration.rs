use std::fmt;

#[derive(Clone)]
pub struct Decoration {}
impl Decoration {
    pub fn new() -> Decoration {
        Decoration {}
    }

    pub fn run_commands(&mut self) -> Vec<String> {
        const PREFIX: &str = "keyword";
        const SECTION: &str = "decoration";
        vec![
           // format!(
           //     "{PREFIX} {SECTION}:{} {}",
           //     "border_size\n", self.border_size
           // ),
        ]
    }
}
