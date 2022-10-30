use std::fmt;

#[derive(Clone)]
pub struct Gestures {}
impl Gestures {
    pub fn new() -> Gestures {
        Gestures {}
    }

    pub fn run_commands(&mut self) -> Vec<String> {
        const PREFIX: &str = "keyword";
        const SECTION: &str = "gestures";
        vec![
           // format!(
           //     "{PREFIX} {SECTION}:{} {}",
           //     "border_size\n", self.border_size
           // ),
        ]
    }
}
