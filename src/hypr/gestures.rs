#[derive(Clone)]
pub struct Gestures {
    /// enable workspace swipe gesture
    workspace_swipe: bool,
    /// how many fingers for the gesture
    workspace_swipe_fingers: u8,
    /// in px, the distance of the gesture
    workspace_swipe_distance: u16,
    /// invert the direction
    workspace_swipe_invert: bool,
    /// minimum speed in px per timepoint to force the change ignoring cancel_ratio. Setting to 0 will disable this mechanic.
    workspace_swipe_min_speed_to_force: u16,
    /// (0.0 - 1.0) how much the swipe has to proceed in order to commence it. (0.7 -> if > 0.7 * distance, switch, if less, revert)
    workspace_swipe_cancel_ratio: f32,
    /// whether a swipe right on the last workspace should create a new one.
    workspace_swipe_create_new: bool,
}
impl Gestures {
    pub fn new() -> Gestures {
        Gestures {
            workspace_swipe: false,
            workspace_swipe_fingers: 3,
            workspace_swipe_distance: 300,
            workspace_swipe_invert: true,
            workspace_swipe_min_speed_to_force: 30,
            workspace_swipe_cancel_ratio: 0.5,
            workspace_swipe_create_new: true,
        }
    }

    pub fn run_commands(&mut self) -> Vec<String> {
        const PREFIX: &str = "keyword";
        const SECTION: &str = "gestures";
        vec![
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "workspace_swipe", self.workspace_swipe
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "workspace_swipe_fingers", self.workspace_swipe_fingers
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "workspace_swipe_distance", self.workspace_swipe_distance
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "workspace_swipe_invert", self.workspace_swipe_invert
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "workspace_swipe_min_speed_to_force", self.workspace_swipe_min_speed_to_force
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "workspace_swipe_cancel_ratio", self.workspace_swipe_cancel_ratio
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "workspace_swipe_create_new", self.workspace_swipe_create_new
            ),
        ]
    }

    pub fn set_workspace_swipe(&mut self, new_value: bool) {
        self.workspace_swipe = new_value;
    }
    pub fn set_workspace_swipe_fingers(&mut self, new_value: u8) {
        self.workspace_swipe_fingers = new_value;
    }
    pub fn set_workspace_swipe_distance(&mut self, new_value: u16) {
        self.workspace_swipe_distance = new_value;
    }
    pub fn set_workspace_swipe_invert(&mut self, new_value: bool) {
        self.workspace_swipe_invert = new_value;
    }
    pub fn set_workspace_swipe_min_speed_to_force(&mut self, new_value: u16) {
        self.workspace_swipe_min_speed_to_force = new_value;
    }
    pub fn set_workspace_swipe_cancel_ratio(&mut self, new_value: f32) {
        self.workspace_swipe_cancel_ratio = new_value;
    }
    pub fn set_workspace_swipe_create_new(&mut self, new_value: bool) {
        self.workspace_swipe_create_new = new_value;
    }
}
