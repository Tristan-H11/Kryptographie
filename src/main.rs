mod rsa;
mod tests;
mod gui;

fn main() {
    let gui = gui::gui::Gui::new();
    gui.run();
}
