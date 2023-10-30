mod encryption;
mod gui;
mod tests;

use druid::{AppLauncher, Color, FontDescriptor, theme, WindowDesc};
use gui::gui::build_ui;
use gui::model::model::AppState;
use log::LevelFilter;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .with_colors(true)
        .init()
        .unwrap();

    let main_window = WindowDesc::new(build_ui())
        .title("RSA-Simulator")
        .resizable(true);
    let initial_state = AppState::default();
    AppLauncher::with_window(main_window)
        .configure_env(|env, _| {
            let background_color = Color::from_hex_str("#2a2a2a").unwrap();
            let button_color = Color::from_hex_str("#607d8b").unwrap();
            let button_color_pressed = Color::from_hex_str("#2b3f49").unwrap();
            let text_color = Color::from_hex_str("#FFFFFF").unwrap();

            env.set(theme::TEXTBOX_BORDER_RADIUS, 0.0);
            env.set(theme::TEXTBOX_BORDER_WIDTH, 0.0);

            env.set(theme::WINDOW_BACKGROUND_COLOR, background_color);
            env.set(theme::TEXT_COLOR, text_color);
            env.set(theme::BUTTON_LIGHT, button_color);
            env.set(theme::BUTTON_DARK, button_color_pressed);
            env.set(theme::BUTTON_BORDER_RADIUS, 0.0);
            env.set(theme::BUTTON_BORDER_WIDTH, 0.0);

            env.set(theme::UI_FONT, FontDescriptor::default().with_size(14.0));
        })
        .launch(initial_state)
        .expect("Failed to launch application");
}
