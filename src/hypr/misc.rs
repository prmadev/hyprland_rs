use std::fmt;

#[derive(Clone)]
pub struct Misc {}
impl Misc {
    pub fn new() -> Misc {
        Misc {}
    }

    pub fn run_commands(&mut self) -> Vec<String> {
        const PREFIX: &str = "keyword";
        const SECTION: &str = "misc";
        vec![
           // format!(
           //     "{PREFIX} {SECTION}:{} {}",
           //     "border_size\n", self.border_size
           // ),
        ]
    }
}
