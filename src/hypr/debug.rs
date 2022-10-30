#[derive(Clone)]
pub struct Debug {
    /// print the debug performance overlay. Disable VFR for accurate results.
    overlay: bool,
    /// (epilepsy warning!) flash areas updated with damage tracking
    damage_blink: bool,
    /// self-explanatory
    disable_logs: bool,
    /// disables
    disable_time: bool,
}
impl Debug {
    pub fn new() -> Debug {
        Debug {
            overlay: false,
            damage_blink: false,
            disable_logs: false,
            disable_time: false,
        }
    }

    pub fn run_commands(&mut self) -> Vec<String> {
        const PREFIX: &str = "keyword";
        const SECTION: &str = "debug";
        vec![
            format!("{PREFIX} {SECTION}:{} {}", "overlay", self.overlay),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "damage_blink", self.damage_blink
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "disable_logs", self.disable_logs
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "disable_time", self.disable_time
            ),
        ]
    }

    pub fn set_overlay(&mut self, new_value: bool) {
        self.overlay = new_value;
    }

    pub fn set_damage_blink(&mut self, new_value: bool) {
        self.damage_blink = new_value;
    }
    pub fn set_disable_logs(&mut self, new_value: bool) {
        self.disable_logs = new_value;
    }
    pub fn set_disable_time(&mut self, new_value: bool) {
        self.disable_time = new_value;
    }
}
