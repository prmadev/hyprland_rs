//! Hyprland Window Manager Configuration library through rust!
//!
//! hyprland-rs provides rust bindings to generate configuration
//! _commands_ which can be sent through hyprland sockets.
//!

/// connection module provides connection and methods needed to
/// connect to the hyprland socket for sending commands.
pub mod connection;

/// hypr provides type structures and methods for creating a full
/// configuration command set.
pub mod hypr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_default() {
        for cmd in hypr::Settings::new().run_commands().iter() {
            println!("{}", cmd)
        }
    }
}
