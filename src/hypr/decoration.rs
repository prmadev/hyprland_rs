#[derive(Clone)]
pub struct Decoration {
    /// rounded corners’ radius (in layout px)
    rounding: u8,
    /// enable antialiasing (no-jaggies) for rounded corners
    multisample_edges: bool,
    /// self-explanatory, only for windows. (0.0 - 1.0)
    active_opacity: f32,
    /// self-explanatory, only for windows. (0.0 - 1.0)
    inactive_opacity: f32,
    /// self-explanatory, only for windows. (0.0 - 1.0)
    fullscreen_opacity: f32,
    /// enable kawase window background blur
    blur: bool,
    /// blur size (distance)
    blur_size: u8,
    /// the amount of passes to perform
    blur_passes: u8,
    /// make the blur layer ignore the opacity of the window
    blur_ignore_opacity: bool,
    /// whether to enable further optimizations to the blur. Recommended to turn on, as it will massively improve performance, but some people have experienced graphical issues
    blur_new_optimizations: bool,
    /// enable drop shadows on windows
    drop_shadow: bool,
    /// Shadow range (“size”) in layout px
    shadow_range: u32,
    /// (1 - 4), in what power to render the falloff (more power, the faster the falloff)
    shadow_render_power: u8,
    /// if true, the shadow will not be rendered behind the window itself, only around it.
    shadow_ignore_window: bool,
    /// shadow’s color. Alpha dictates shadow’s opacity.
    col_shadow: String,
    /// inactive shadow color. (if not set, will fall back to col.shadow)
    col_shadow_inactive: String,
    /// shadow’s rendering offset.
    shadow_offset: Vec<u8>,
    /// enables dimming of inactive windows
    dim_inactive: bool,
    /// how much inactive windows should be dimmed, 0.0 - 1.0
    dim_strength: f32,
}
impl Decoration {
    pub fn new() -> Decoration {
        Decoration {
            active_opacity: 1.0,
            blur: true,
            blur_ignore_opacity: false,
            blur_new_optimizations: false,
            blur_passes: 1,
            blur_size: 8,
            col_shadow: String::from("0xee1a1a1a"),
            col_shadow_inactive: String::from("0x00000000"),
            dim_inactive: false,
            dim_strength: 0.5,
            drop_shadow: true,
            fullscreen_opacity: 1.0,
            inactive_opacity: 1.0,
            multisample_edges: true,
            rounding: 0,
            shadow_ignore_window: true,
            shadow_offset: vec![0, 0],
            shadow_range: 4,
            shadow_render_power: 3,
        }
    }

    pub fn run_commands(&mut self) -> Vec<String> {
        const PREFIX: &str = "keyword";
        const SECTION: &str = "decoration";
        vec![
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "active_opacity", self.active_opacity
            ),
            format!("{PREFIX} {SECTION}:{} {}", "blur", self.blur),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "blur_ignore_opacity", self.blur_ignore_opacity
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "blur_new_optimizations", self.blur_new_optimizations
            ),
            format!("{PREFIX} {SECTION}:{} {}", "blur_passes", self.blur_passes),
            format!("{PREFIX} {SECTION}:{} {}", "blur_size", self.blur_size),
            format!("{PREFIX} {SECTION}:{} {}", "col.shadow", self.col_shadow),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "col.shadow_inactive", self.col_shadow_inactive
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "dim_inactive", self.dim_inactive
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "dim_strength", self.dim_strength
            ),
            format!("{PREFIX} {SECTION}:{} {}", "drop_shadow", self.drop_shadow),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "fullscreen_opacity", self.fullscreen_opacity
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "inactive_opacity", self.inactive_opacity
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "multisample_edges", self.multisample_edges
            ),
            format!("{PREFIX} {SECTION}:{} {}", "rounding", self.rounding),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "shadow_ignore_window", self.shadow_ignore_window
            ),
            format!(
                "{PREFIX} {SECTION}:{} {:?}",
                "shadow_offset", self.shadow_offset
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "shadow_range", self.shadow_range
            ),
            format!(
                "{PREFIX} {SECTION}:{} {}",
                "shadow_render_power", self.shadow_render_power
            ),
        ]
    }

    pub fn set_active_opacity(&mut self, new_value: f32) {
        self.active_opacity = new_value;
    }

    pub fn set_blur(&mut self, new_value: bool) {
        self.blur = new_value;
    }

    pub fn set_rounding(&mut self, new_value: u8) {
        self.rounding = new_value;
    }
    pub fn set_multisample_edges(&mut self, new_value: bool) {
        self.multisample_edges = new_value;
    }
    pub fn set_inactive_opacity(&mut self, new_value: f32) {
        self.inactive_opacity = new_value;
    }
    pub fn set_fullscreen_opacity(&mut self, new_value: f32) {
        self.fullscreen_opacity = new_value;
    }
    pub fn set_blur_size(&mut self, new_value: u8) {
        self.blur_size = new_value;
    }
    pub fn set_blur_passes(&mut self, new_value: u8) {
        self.blur_passes = new_value;
    }
    pub fn set_blur_ignore_opacity(&mut self, new_value: bool) {
        self.blur_ignore_opacity = new_value;
    }
    pub fn set_blur_new_optimizations(&mut self, new_value: bool) {
        self.blur_new_optimizations = new_value;
    }
    pub fn set_drop_shadow(&mut self, new_value: bool) {
        self.drop_shadow = new_value;
    }

    pub fn set_shadow_range(&mut self, new_value: u32) {
        self.shadow_range = new_value;
    }

    pub fn set_shadow_render_power(&mut self, new_value: u8) {
        self.shadow_render_power = new_value;
    }
    pub fn set_shadow_ignore_window(&mut self, new_value: u8) {
        self.shadow_render_power = new_value;
    }
    pub fn set_col_shadow(&mut self, new_value: String) {
        self.col_shadow = new_value;
    }
    pub fn set_col_shadow_inactive(&mut self, new_value: String) {
        self.col_shadow_inactive = new_value;
    }
    pub fn set_shadow_offset(&mut self, new_value: Vec<u8>) {
        self.shadow_offset = new_value;
    }
    pub fn set_dim_inactive(&mut self, new_value: bool) {
        self.dim_inactive = new_value;
    }

    pub fn set_dim_strength(&mut self, new_value: f32) {
        self.dim_strength = new_value;
    }
}
