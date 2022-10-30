use std::fmt;

#[derive(Clone)]
pub struct Misc {
    /// disables the hyprland logo background. :(
    disable_hyprland_logo: bool,
    /// disables the hyprland splash rendering. (requires a monitor reload to take effect)
    disable_splash_rendering: bool,
    /// disables VFR (variable frame rate) - VFR increases battery life at the expense of possible issues on a few monitors.
    no_vfr: bool,
    /// If DPMS is set to off, wake up the monitors if the mouse moves.
    mouse_move_enables_dpms: bool,
    /// Will make mouse focus follow the mouse when drag and dropping. Recommended to leave it enabled, especially for people using focus follows mouse at 0.
    always_follow_on_dnd: bool,
    /// If true, will make keyboard-interactive layers keep their focus on mouse move (e.g. wofi, bemenu)
    layers_hog_keyboard_focus: bool,
    /// If true, will animate manual window resizes/moves
    animate_manual_resizes: bool,
    /// If true, the config will not reload automatically on save, and instead needs to be reloaded with hyprctl reload. Might save on battery.
    disable_autoreload: bool,
    /// Enable window swallowing
    enable_swallow: bool,
    /// The class regex to be used for windows that should be swallowed (usually, a terminal)
    swallow_regex: String,
    /// Whether Hyprland should focus an app that requests to be focused (an activate request)
    focus_on_activate: bool,
}
impl Misc {
    pub fn new() -> Misc {
        Misc {
            disable_hyprland_logo: false,
            disable_splash_rendering: false,
            no_vfr: true,
            mouse_move_enables_dpms: false,
            always_follow_on_dnd: true,
            layers_hog_keyboard_focus: true,
            animate_manual_resizes: false,
            disable_autoreload: false,
            enable_swallow: false,
            swallow_regex: String::from(""),
            focus_on_activate: true,
        }
    }

    pub fn run_commands(&mut self) -> Vec<String> {
        const PREFIX: &str = "keyword";
        const SECTION: &str = "misc";
        vec![
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "disable_hyprland_logo", self.disable_hyprland_logo
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "disable_splash_rendering", self.disable_splash_rendering
            ),
            format!("{PREFIX} {SECTION}:{} {}", "no_vfr", self.no_vfr),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "mouse_move_enables_dpms", self.mouse_move_enables_dpms
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "always_follow_on_dnd", self.always_follow_on_dnd
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "layers_hog_keyboard_focus", self.layers_hog_keyboard_focus
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "animate_manual_resizes", self.animate_manual_resizes
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "disable_autoreload", self.disable_autoreload
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "enable_swallow", self.enable_swallow
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "swallow_regex", self.swallow_regex
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "focus_on_activate", self.focus_on_activate
            ),
        ]
    }
    pub fn set_disable_hyprland_logo(&mut self, new_value: bool) {
        self.disable_hyprland_logo = new_value;
    }
    pub fn set_disable_splash_rendering(&mut self, new_value: bool) {
        self.disable_splash_rendering = new_value;
    }
    pub fn set_no_vfr(&mut self, new_value: bool) {
        self.no_vfr = new_value;
    }
    pub fn set_mouse_move_enables_dpms(&mut self, new_value: bool) {
        self.mouse_move_enables_dpms = new_value;
    }
    pub fn set_always_follow_on_dnd(&mut self, new_value: bool) {
        self.always_follow_on_dnd = new_value;
    }
    pub fn set_layers_hog_keyboard_focus(&mut self, new_value: bool) {
        self.layers_hog_keyboard_focus = new_value;
    }
    pub fn set_animate_manual_resizes(&mut self, new_value: bool) {
        self.animate_manual_resizes = new_value;
    }
    pub fn set_disable_autoreload(&mut self, new_value: bool) {
        self.disable_autoreload = new_value;
    }
    pub fn set_enable_swallow(&mut self, new_value: bool) {
        self.enable_swallow = new_value;
    }
    pub fn set_swallow_regex(&mut self, new_value: String) {
        self.swallow_regex = new_value;
    }
    pub fn set_focus_on_activate(&mut self, new_value: bool) {
        self.focus_on_activate = new_value;
    }
}
