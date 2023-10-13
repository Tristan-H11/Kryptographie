pub struct GuiMath;

impl GuiMath {
    pub fn calculate_window_size() -> (f64, f64) {
        // Definieren Sie die Fenstergröße als feste Werte -- Dynamische Berechnung mit Druid GUI leider nicht möglich ...
        let window_width = 800.0;  // Beispielwert
        let window_height = 600.0;  // Beispielwert

        (window_width, window_height)
    }
}


