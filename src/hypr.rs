mod animation;
mod binds;
mod debug;
mod decoration;
mod general;
mod gestures;
mod input;
mod misc;
pub use animation::*;
pub use binds::*;
pub use debug::*;
pub use decoration::*;
pub use general::*;
pub use gestures::*;
pub use input::*;
pub use misc::*;

pub struct Settings {
    general: General,
    decoration: Decoration,
    animation: Animation,
    binds: Binds,
    debug: Debug,
    gestures: Gestures,
    input: Input,
    misc: Misc,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            general: General::new(),
            decoration: Decoration::new(),
            animation: Animation::new(),
            binds: Binds::new(),
            debug: Debug::new(),
            gestures: Gestures::new(),
            input: Input::new(),
            misc: Misc::new(),
        }
    }
    pub fn get_mut_general(&mut self) -> &mut General {
        &mut self.general
    }

    pub fn get_mut_decoration(&mut self) -> &mut Decoration {
        &mut self.decoration
    }

    pub fn get_mut_animation(&mut self) -> &mut Animation {
        &mut self.animation
    }

    pub fn get_mut_binds(&mut self) -> &mut Binds {
        &mut self.binds
    }
    pub fn get_mut_debug(&mut self) -> &mut Debug {
        &mut self.debug
    }
    pub fn get_mut_gestures(&mut self) -> &mut Gestures {
        &mut self.gestures
    }
    pub fn get_mut_input(&mut self) -> &mut Input {
        &mut self.input
    }
    pub fn get_mut_misc(&mut self) -> &mut Misc {
        &mut self.misc
    }

    pub fn run_commands(&mut self) -> Vec<String> {
        let mut commands: Vec<String> = vec![];
        commands.append(&mut self.general.run_commands());
        commands.append(&mut self.animation.run_commands());
        commands.append(&mut self.binds.run_commands());
        commands.append(&mut self.debug.run_commands());
        commands.append(&mut self.gestures.run_commands());
        commands.append(&mut self.input.run_commands());
        commands.append(&mut self.misc.run_commands());
        commands.to_owned()
    }
}
