extern crate core;

use std::process::Command;

use std::sync::{Arc, Mutex};

use enigo::Enigo;
use fltk::button::Button;
use fltk::{app, app::AppScheme, prelude::*, window::Window};
use rdev::{grab, Event, EventType};

use last_right_click::LastRightClick;
use load_config::LoadConfig;

mod last_right_click;
mod load_config;

fn main() {
    let app = app::App::default().with_scheme(AppScheme::Plastic);

    app::set_background_color(170, 189, 206);

    let configs = LoadConfig::load_buttons();

    let mut window_height = configs.len() as i32 * 60;

    if window_height < 130 {
        window_height = 140;
    }

    let mut window = Window::default()
        .with_size(180, window_height)
        .with_label("Switcher")
        .center_screen();

    let last_right_click = LastRightClick::new();
    let last_right_click_clone = Arc::new(Mutex::new(last_right_click));

    let mut button_x = 30;

    for config in configs {
        let title_boxed = Box::new(config.name);
        let title_static: &'static str = Box::leak(title_boxed);

        let mut button = Button::new(50, button_x, 80, 40, Some(title_static));
        button_x += 50;

        let last_right_click_clone_button = last_right_click_clone.clone();

        button.set_callback(move |_button| {
            Command::new("open")
                .arg(config.app_path.as_str())
                .output()
                .expect("Failed to open application");

            last_right_click_clone_button.lock().unwrap().reset();
        });
    }

    window.show();

    let window_clone = Arc::new(Mutex::new(window.clone()));

    if let Err(_error) = grab(move |event: Event| {
        match event.event_type {
            EventType::ButtonPress(rdev::Button::Right) => {
                let cursor_location: (i32, i32) = Enigo::mouse_location();

                let mut last_cursor_location = last_right_click_clone.lock().unwrap();

                if last_cursor_location.x == cursor_location.0
                    && last_cursor_location.y == cursor_location.1
                {
                    let mut window = window_clone.lock().unwrap();

                    window.set_pos(cursor_location.0 - 80, cursor_location.1 - 110);

                    // Show in front of everything else
                    window.platform_show();

                    last_cursor_location.reset();

                    return None;
                }

                last_cursor_location.x = cursor_location.0;
                last_cursor_location.y = cursor_location.1;

                Some(event)
            }

            _ => Some(event),
        }
    }) {
        fltk::dialog::alert(
            200,
            100,
            "Failed to listen to key events. \
            Enable the app in Settings > Privacy and Security > Accessibility",
        );
    }

    while app.wait() {}
}
