use std::fmt;

#[derive(Clone)]
pub struct Input {}
impl Input {
    pub fn new() -> Input {
        Input {}
    }

    pub fn run_commands(&mut self) -> Vec<String> {
        const PREFIX: &str = "keyword";
        const SECTION: &str = "input";
        vec![
           // format!(
           //     "{PREFIX} {SECTION}:{} {}",
           //     "border_size\n", self.border_size
           // ),
        ]
    }
}
