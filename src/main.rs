// #![windows_subsystem = "windows"]

mod login;
// mod signup;
pub mod window;
use iced::{Sandbox, Settings, futures::io::Window};

pub fn main() -> iced::Result {


    let window_size = (320, 200);
    
    let setting = Settings {
        window: iced::window::Settings {
            size: window_size,
            resizable: false,
            ..Default::default()
        },
        ..Default::default()
    };
    login::Login::run(setting)
}
