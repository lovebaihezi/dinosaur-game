pub mod app;
mod camera;
pub mod components;
mod dino;
mod game_control;
pub mod game_logic;
mod ground;
mod resources;
pub mod test_functions;
mod tree;
mod windows_handler;

pub use camera::normal_app_setup;
pub use dino::*;
pub use game_control::*;
pub use ground::*;
pub use resources::*;
pub use tree::*;
pub use windows_handler::*;
