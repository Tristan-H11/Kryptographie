mod rsa;
mod tests;
fn main() {
    let gui = rsa::gui::gui::Gui::new();
    gui.run();
}
