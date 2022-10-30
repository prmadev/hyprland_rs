#[derive(Clone)]
pub struct Binds {
    /// if disabled, will not pass the mouse events to apps / dragging windows around if a keybind has been triggered.
    pass_mouse_when_bound: bool,
    /// in ms, how many ms to wait after a scroll event to allow to pass another one for the binds.
    scroll_event_delay: u32,
    /// If enabled, an attempt to switch to the currently focused workspace will instead switch to the previous workspace. Akin to i3’s auto_back_and_forth.
    workspace_back_and_forth: bool,
    /// If enabled, workspaces don’t forget their previous workspace, so cycles can be created by switching to the first workspace in a sequence, then endlessly going to the previous workspace.
    allow_workspace_cycles: bool,
}
impl Binds {
    pub fn new() -> Binds {
        Binds {
            pass_mouse_when_bound: false,
            scroll_event_delay: 300,
            workspace_back_and_forth: false,
            allow_workspace_cycles: false,
        }
    }

    pub fn run_commands(&mut self) -> Vec<String> {
        const PREFIX: &str = "keyword";
        const SECTION: &str = "binds";
        vec![
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "pass_mouse_when_bound", self.pass_mouse_when_bound
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "scroll_event_delay", self.scroll_event_delay
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "workspace_back_and_forth", self.workspace_back_and_forth
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "allow_workspace_cycles", self.allow_workspace_cycles
            ),
        ]
    }

    pub fn set_pass_mouse_when_bound(&mut self, new_value: bool) {
        self.pass_mouse_when_bound = new_value
    }

    pub fn set_scroll_event_delay(&mut self, new_value: u32) {
        self.scroll_event_delay = new_value
    }
    pub fn set_workspace_back_and_forth(&mut self, new_value: bool) {
        self.workspace_back_and_forth = new_value
    }
    pub fn set_allow_workspace_cycles(&mut self, new_value: bool) {
        self.allow_workspace_cycles = new_value
    }
}
