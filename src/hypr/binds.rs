use std::fmt;

#[derive(Clone)]
pub struct Binds {}
impl Binds {
    pub fn new() -> Binds {
        Binds {}
    }

    pub fn run_commands(&mut self) -> Vec<String> {
        const PREFIX: &str = "keyword";
        const SECTION: &str = "binds";
        vec![
           // format!(
           //     "{PREFIX} {SECTION}:{} {}",
           //     "border_size\n", self.border_size
           // ),
        ]
    }
}
