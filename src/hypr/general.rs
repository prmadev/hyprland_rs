use std::fmt;

#[derive(Clone)]
pub struct General {
    /// self-explanatory
    border_size: u8,
    /// disable borders for floating windows
    no_border_on_floating: bool,
    /// gaps between windows
    gaps_in: u8,
    /// gaps between windows and monitor edges
    gaps_out: u8,
    /// self-explanatory
    col_inactive_border: String,
    /// self-explanatory
    col_active_border: String,
    /// in seconds, after how many seconds of cursor’s inactivity to hide it. Set to 0 for never.
    cursor_inactive_timeout: i8,
    /// Makes the compositor redraw only the needed bits of the display. Saves on resources by not redrawing when not needed. Available modes: none, monitor, full. You don’t need to know what different modes do, just always use full.
    damage_tracking: DamageTrackingOpts,
    /// which layout to use.
    layout: LayoutOpts,
    /// if true, will not warp the cursor in many cases (focusing, keybinds, etc)
    no_cursor_warps: bool,
    /// if on, will also apply the sensitivity to raw mouse output (e.g. sensitivity in games) NOTICE: really not recommended.
    apply_sens_to_raw: bool,
    ///
    main_mod: Mods,
}

impl General {
    pub fn new() -> General {
        General {
            border_size: 1,
            no_border_on_floating: false,
            gaps_in: 5,
            gaps_out: 20,
            col_inactive_border: String::from("0xffffffff"),
            col_active_border: String::from("0xffffffff"),
            cursor_inactive_timeout: 0,
            damage_tracking: DamageTrackingOpts::Full,
            layout: LayoutOpts::Dwindle,
            no_cursor_warps: false,
            apply_sens_to_raw: false,
            main_mod: Mods::Super,
        }
    }
    pub fn run_commands(&mut self) -> Vec<String> {
        const PREFIX: &str = "keyword";
        const SECTION: &str = "general";
        vec![
            format!("{PREFIX} {SECTION}:{} {}", "border_size", self.border_size),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "no_border_on_floating", self.no_border_on_floating
            ),
            format!("{PREFIX} {SECTION}:{} {}", "gaps_in", self.gaps_in),
            format!("{PREFIX} {SECTION}:{} {}", "gaps_out", self.gaps_out),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "col.active_border", self.col_active_border
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "col.inactive_border", self.col_inactive_border
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "cursor_inactive_timeout", self.cursor_inactive_timeout
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "damage_tracking", self.damage_tracking
            ),
            format!("{PREFIX} {SECTION}:{} {}", "layout", self.layout),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "no_cursor_warps", self.no_cursor_warps
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "apply_sens_to_raw", self.apply_sens_to_raw
            ),
            format!("{PREFIX} {SECTION}:{} {}", "main_mod", self.main_mod),
        ]
    }

    pub fn set_border_size(&mut self, new_value: u8) {
        self.border_size = new_value;
    }

    pub fn set_no_border_on_floating(&mut self, new_value: bool) {
        self.no_border_on_floating = new_value;
    }

    pub fn set_gaps_in(&mut self, new_value: u8) {
        self.gaps_in = new_value;
    }

    pub fn set_gaps_out(&mut self, new_value: u8) {
        self.gaps_out = new_value;
    }

    pub fn set_color_inactive_border(&mut self, new_value: &str) {
        self.col_inactive_border = String::from(new_value);
    }

    pub fn set_color_active_border(&mut self, new_value: &str) {
        self.col_active_border = String::from(new_value);
    }

    pub fn set_cursor_inactive_timeout(&mut self, new_value: i8) {
        self.cursor_inactive_timeout = new_value;
    }

    pub fn set_damage_tracking(&mut self, new_value: DamageTrackingOpts) {
        self.damage_tracking = new_value;
    }

    pub fn set_layout(&mut self, new_value: LayoutOpts) {
        self.layout = new_value;
    }

    pub fn set_no_cursor_warps(&mut self, new_value: bool) {
        self.no_cursor_warps = new_value;
    }

    pub fn set_apply_sens_to_raw(&mut self, new_value: bool) {
        self.apply_sens_to_raw = new_value;
    }

    pub fn set_main_mod(&mut self, new_value: Mods) {
        self.main_mod = new_value;
    }
}

#[derive(Clone)]
pub enum LayoutOpts {
    Dwindle,
    Master,
}

impl fmt::Display for LayoutOpts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LayoutOpts::Dwindle => write!(f, "dwindle"),
            LayoutOpts::Master => write!(f, "master"),
        }
    }
}

#[derive(Clone)]
pub enum DamageTrackingOpts {
    Nothing,
    Monitor,
    Full,
}

impl fmt::Display for DamageTrackingOpts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DamageTrackingOpts::Full => write!(f, "full"),
            DamageTrackingOpts::Nothing => write!(f, "none"),
            DamageTrackingOpts::Monitor => write!(f, "monitor"),
        }
    }
}

#[derive(Clone)]
pub enum Mods {
    Super,
}

impl fmt::Display for Mods {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Mods::Super => write!(f, "SUPER"),
        }
    }
}
