use std::fmt;

#[derive(Clone)]
pub struct Debug {}
impl Debug {
    pub fn new() -> Debug {
        Debug {}
    }

    pub fn run_commands(&mut self) -> Vec<String> {
        const PREFIX: &str = "keyword";
        const SECTION: &str = "debug";
        vec![
           // format!(
           //     "{PREFIX} {SECTION}:{} {}",
           //     "border_size\n", self.border_size
           // ),
        ]
    }
}
