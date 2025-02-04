mod gui;
mod driver_manager;
mod installer;

use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Foundation::*;

fn main() -> windows::core::Result<()> {
    gui::init_logger();
    let app = gui::App::new()?;
    app.run()
}