use std::fmt;

#[derive(Clone)]
pub struct Animation {}
impl Animation {
    pub fn new() -> Animation {
        Animation {}
    }

    pub fn run_commands(&mut self) -> Vec<String> {
        const PREFIX: &str = "keyword";
        const SECTION: &str = "animation";
        vec![
           // format!(
           //     "{PREFIX} {SECTION}:{} {}",
           //     "border_size\n", self.border_size
           // ),
        ]
    }
}
