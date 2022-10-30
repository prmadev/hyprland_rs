#[derive(Clone)]
pub struct Animation {
    /// enable animations
    enabled: bool,
}
impl Animation {
    pub fn new() -> Animation {
        Animation { enabled: true }
    }

    pub fn run_commands(&mut self) -> Vec<String> {
        const PREFIX: &str = "keyword";
        const SECTION: &str = "animation";
        vec![format!("{PREFIX} {SECTION}:{} {}", "enabled", self.enabled,)]
    }
    pub fn set_enabled(&mut self, new_value: bool) {
        self.enabled = new_value
    }
}
