#[derive(Clone)]
pub struct Animations {
    /// enable animations
    enabled: bool,
}
impl Animations {
    pub fn new() -> Animations {
        Animations { enabled: true }
    }

    pub fn run_commands(&mut self) -> Vec<String> {
        const PREFIX: &str = "keyword";
        const SECTION: &str = "animations";
        vec![format!("{PREFIX} {SECTION}:{} {}", "enabled", self.enabled,)]
    }
    pub fn set_enabled(&mut self, new_value: bool) {
        self.enabled = new_value
    }
}
