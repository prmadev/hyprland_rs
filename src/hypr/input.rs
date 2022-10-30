use std::fmt;

#[derive(Clone)]
pub struct Input {
    /// Appropriate XKB keymap parameter
    kb_layout: String,
    /// Appropriate XKB keymap parameter
    kb_variant: String,
    /// Appropriate XKB keymap parameter
    kb_model: String,
    /// Appropriate XKB keymap parameter
    kb_options: String,
    /// Appropriate XKB keymap parameter
    kb_rules: String,
    /// If you prefer, you can use a path to an .xkb file.
    kb_file: String,
    /// enable mouse following (focus on enter new window)
    follow_mouse: FollowMouseOpts,
    /// if enabled , focus will follow mouse if changing from tiled to floating and vice versa. FloatToFloat option will also follow mouse on float -> float switches
    float_switch_override_focus: SwitchOverrideFocusOpts,
    /// in repeats per second, the repeat rate for held keys
    repeat_rate: u16,
    /// in ms, the repeat delay (grace period) before the spam
    repeat_delay: u64,
    /// enable natural scroll
    natural_scroll: bool,
    /// lock numlock by default
    numlock_by_default: bool,
    /// force no mouse acceleration, bypasses most of your pointer settings to get as raw of a signal as possible.
    force_no_accel: bool,
    /// set the libinput sensitivity. This HAS to be from -1 to 1, or else it will be clamped.
    sensitivity: f32,
    /// switches RMB and LMB
    left_handed: bool,
    /// set the libinput acceleration profile. Can be one of adaptive, flat.
    accel_profile: AccelProfileOpts,
    /// set the libinput scroll method.
    scroll_method: ScrollMethodOpts,

    touchpad: Touchpad,
    touch_device: TouchDevice,
}

impl Input {
    pub fn new() -> Input {
        Input {
            kb_layout: String::from("us"),
            kb_variant: String::from(""),
            kb_model: String::from(""),
            kb_options: String::from(""),
            kb_rules: String::from(""),
            kb_file: String::from(""),
            follow_mouse: FollowMouseOpts::Full,
            float_switch_override_focus: SwitchOverrideFocusOpts::EnabledFloatTiled,
            repeat_rate: 25,
            repeat_delay: 600,
            natural_scroll: false,
            numlock_by_default: false,
            force_no_accel: false,
            sensitivity: 0.0,
            left_handed: false,
            accel_profile: AccelProfileOpts::Flat,
            scroll_method: ScrollMethodOpts::TwoFingers,
            touchpad: Touchpad::new(),
            touch_device: TouchDevice::new(),
        }
    }

    pub fn run_commands(&mut self) -> Vec<String> {
        const PREFIX: &str = "keyword";
        const SECTION: &str = "input";
        let mut cmd = vec![
            format!("{PREFIX} {SECTION}:{} {}", "kb_layout", self.kb_layout),
            format!("{PREFIX} {SECTION}:{} {}", "kb_variant", self.kb_variant),
            format!("{PREFIX} {SECTION}:{} {}", "kb_model", self.kb_model),
            format!("{PREFIX} {SECTION}:{} {}", "kb_options", self.kb_options),
            format!("{PREFIX} {SECTION}:{} {}", "kb_rules", self.kb_rules),
            format!("{PREFIX} {SECTION}:{} {}", "kb_file", self.kb_file),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "follow_mouse", self.follow_mouse
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "float_switch_override_focus", self.float_switch_override_focus
            ),
            format!("{PREFIX} {SECTION}:{} {}", "repeat_rate", self.repeat_rate),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "repeat_delay", self.repeat_delay
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "natural_scroll", self.natural_scroll
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "numlock_by_default", self.numlock_by_default
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "force_no_accel", self.force_no_accel
            ),
            format!("{PREFIX} {SECTION}:{} {}", "sensitivity", self.sensitivity),
            format!("{PREFIX} {SECTION}:{} {}", "left_handed", self.left_handed),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "accel_profile", self.accel_profile
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "scroll_method", self.scroll_method
            ),
        ];
        cmd.append(&mut self.get_mut_touchpad().run_commands());
        cmd.append(&mut self.get_mut_touch_device().run_commands());
        cmd
    }

    pub fn get_mut_touchpad(&mut self) -> &mut Touchpad {
        &mut self.touchpad
    }

    pub fn get_mut_touch_device(&mut self) -> &mut TouchDevice {
        &mut self.touch_device
    }

    pub fn set_kb_layout(&mut self, new_value: String) {
        self.kb_layout = new_value;
    }
    pub fn set_kb_variant(&mut self, new_value: String) {
        self.kb_variant = new_value;
    }
    pub fn set_kb_model(&mut self, new_value: String) {
        self.kb_model = new_value;
    }
    pub fn set_kb_options(&mut self, new_value: String) {
        self.kb_options = new_value;
    }
    pub fn set_kb_rules(&mut self, new_value: String) {
        self.kb_rules = new_value;
    }
    pub fn set_kb_file(&mut self, new_value: String) {
        self.kb_file = new_value;
    }
    pub fn set_follow_mouse(&mut self, new_value: FollowMouseOpts) {
        self.follow_mouse = new_value;
    }
    pub fn set_float_switch_override_focus(&mut self, new_value: SwitchOverrideFocusOpts) {
        self.float_switch_override_focus = new_value;
    }
    pub fn set_repeat_rate(&mut self, new_value: u16) {
        self.repeat_rate = new_value;
    }
    pub fn set_repeat_delay(&mut self, new_value: u64) {
        self.repeat_delay = new_value;
    }
    pub fn set_natural_scroll(&mut self, new_value: bool) {
        self.natural_scroll = new_value;
    }
    pub fn set_numlock_by_default(&mut self, new_value: bool) {
        self.numlock_by_default = new_value;
    }
    pub fn set_force_no_accel(&mut self, new_value: bool) {
        self.force_no_accel = new_value;
    }
    pub fn set_sensitivity(&mut self, new_value: f32) {
        self.sensitivity = new_value;
    }
    pub fn set_left_handed(&mut self, new_value: bool) {
        self.left_handed = new_value;
    }
    pub fn set_accel_profile(&mut self, new_value: AccelProfileOpts) {
        self.accel_profile = new_value;
    }
    pub fn set_scroll_method(&mut self, new_value: ScrollMethodOpts) {
        self.scroll_method = new_value;
    }
}

#[derive(Clone)]
pub struct TouchDevice {
    /// transform the input from touchdevices. The possible transformations are the same as those of the monitors
    transform: TransformationOpts,
    /// the output to bind touch devices. Empty means unset and will use the current / autodetected.
    output: Option<String>,
}

impl TouchDevice {
    pub fn new() -> TouchDevice {
        TouchDevice {
            transform: TransformationOpts::Normal,
            output: None,
        }
    }

    pub fn run_commands(&mut self) -> Vec<String> {
        const PREFIX: &str = "keyword";
        const SECTION: &str = "input:touchdevice";
        vec![
            format!("{PREFIX} {SECTION}:{} {}", "transform", self.transform),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "output",
                match self.output.to_owned() {
                    None => String::from(""),
                    Some(o) => o.to_owned(),
                }
            ),
        ]
    }

    pub fn set_transform(&mut self, new_value: TransformationOpts) {
        self.transform = new_value;
    }

    pub fn set_output(&mut self, new_value: Option<String>) {
        self.output = new_value;
    }
}

#[derive(Clone)]
pub enum TransformationOpts {
    Normal,
    Rotate90,
    Rotate180,
    Rotate270,
    FLipped,
    FLippedAndRotate90,
    FLippedAndRotate180,
    FLippedAndRotate270,
}

impl fmt::Display for TransformationOpts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TransformationOpts::Normal => write!(f, "0"),
            TransformationOpts::Rotate90 => write!(f, "1"),
            TransformationOpts::Rotate180 => write!(f, "2"),
            TransformationOpts::Rotate270 => write!(f, "3"),
            TransformationOpts::FLipped => write!(f, "4"),
            TransformationOpts::FLippedAndRotate90 => write!(f, "5"),
            TransformationOpts::FLippedAndRotate180 => write!(f, "6"),
            TransformationOpts::FLippedAndRotate270 => write!(f, "7"),
        }
    }
}

#[derive(Clone)]
pub struct Touchpad {
    /// disables the touchpad while typing
    disable_while_typing: bool,
    /// self-explanatory
    natural_scroll: bool,
    /// self-explanatory
    clickfinger_behavior: bool,
    /// self-explanatory
    middle_button_emulation: bool,
    /// self-explanatory
    tap_to_click: bool,
    /// enable dragging with drag lock
    drag_lock: bool,
    /// control the amount of scroll applied
    scroll_factor: f32,
}

impl Touchpad {
    pub fn new() -> Touchpad {
        Touchpad {
            disable_while_typing: true,
            natural_scroll: false,
            clickfinger_behavior: false,
            middle_button_emulation: false,
            tap_to_click: true,
            drag_lock: false,
            scroll_factor: 1.0,
        }
    }

    pub fn run_commands(&mut self) -> Vec<String> {
        const PREFIX: &str = "keyword";
        const SECTION: &str = "input:touchpad";
        vec![
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "disable_while_typing", self.disable_while_typing
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "natural_scroll", self.natural_scroll
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "clickfinger_behavior", self.clickfinger_behavior
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "middle_button_emulation", self.middle_button_emulation
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "tap-to-click", self.tap_to_click
            ),
            format!("{PREFIX} {SECTION}:{} {}", "drag_lock", self.drag_lock),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "scroll_factor", self.scroll_factor
            ),
        ]
    }

    pub fn set_disable_while_typing(&mut self, new_value: bool) {
        self.disable_while_typing = new_value;
    }

    pub fn set_natural_scroll(&mut self, new_value: bool) {
        self.natural_scroll = new_value;
    }
    pub fn set_clickfinger_behavior(&mut self, new_value: bool) {
        self.clickfinger_behavior = new_value;
    }
    pub fn set_middle_button_emulation(&mut self, new_value: bool) {
        self.middle_button_emulation = new_value;
    }
    pub fn set_tap_to_click(&mut self, new_value: bool) {
        self.tap_to_click = new_value;
    }
    pub fn set_drag_lock(&mut self, new_value: bool) {
        self.drag_lock = new_value;
    }
    pub fn set_scroll_factor(&mut self, new_value: f32) {
        self.scroll_factor = new_value;
    }
}

#[derive(Clone)]
pub enum AccelProfileOpts {
    Adaptive,
    Flat,
}

impl fmt::Display for AccelProfileOpts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AccelProfileOpts::Adaptive => write!(f, "adaptive"),
            AccelProfileOpts::Flat => write!(f, "flat"),
        }
    }
}

#[derive(Clone)]
pub enum FollowMouseOpts {
    Disabled,
    Full,
    /// Will focus mouse on other windows on focus but not the keyboard.
    Loose,
    /// full loose, will not refocus on click, but allow mouse focus to be detached from the keyboard like in Loose
    FullLoose,
}

impl fmt::Display for FollowMouseOpts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FollowMouseOpts::Disabled => write!(f, "0"),
            FollowMouseOpts::Full => write!(f, "1"),
            FollowMouseOpts::Loose => write!(f, "2"),
            FollowMouseOpts::FullLoose => write!(f, "3"),
        }
    }
}
#[derive(Clone)]
pub enum SwitchOverrideFocusOpts {
    Disable,
    /// focus will follow mouse if changing from tiled to floating and vice versa.
    EnabledFloatTiled,
    /// will also follow mouse on float -> float switches
    EnabledFloatTiledAndFloatToFloat,
}

impl fmt::Display for SwitchOverrideFocusOpts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SwitchOverrideFocusOpts::Disable => write!(f, "0"),
            SwitchOverrideFocusOpts::EnabledFloatTiled => write!(f, "1"),
            SwitchOverrideFocusOpts::EnabledFloatTiledAndFloatToFloat => write!(f, "2"),
        }
    }
}

#[derive(Clone)]
pub enum ScrollMethodOpts {
    TwoFingers,
    Edge,
    OnButtonDown,
    NoScroll,
}

impl fmt::Display for ScrollMethodOpts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ScrollMethodOpts::Edge => write!(f, "edge"),
            ScrollMethodOpts::TwoFingers => write!(f, "2fg"),
            ScrollMethodOpts::OnButtonDown => write!(f, "on_button_down"),
            ScrollMethodOpts::NoScroll => write!(f, "no_scroll"),
        }
    }
}
