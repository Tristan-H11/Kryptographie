// German Description of GUI Functions
// Anforderungen für die GUI in Rust Rover:

// Drei Masken sollen erstellt werden:
// - Hauptmaske
// - Alice -- für Informationen von Alice
// - Bob -- für Informationen von Bob

// - Es kann immer nur eine Maske angezeigt werden, diese soll dynamisch beim öffnen mit ca. 80% der Bildschrimfläche geöffnet werden

// Hauptmaske für Schlüsselerzeugung:
// - Eingabemöglichkeit der Anzahl der Schritte bei dem Miller-Rabin-Verfahren
// - Eingabemöglichkeit für Länge der Primzahlen p_1 und p_2
// - Startbutton, welcher Mathefunktion aufruft, welche die beiden Feldinformationen übergeben bekommt und ein Ergebnis in ein Lable auf dieser Maske schreibt
// - Label um den öffentlichen Schlüssel für Alice und Bob anzuzeigen (n,e_a) und (n,e_b)
// - Button um Alice Maske anzeigen zu lassen
// - Button um BOB Maske anzeigen zu lassen

// Maske Alice:
// - Anzeige des geheimen Schlüssels für Alice (d_a)
// - Feld zum Anzeigen des Klartextes bzw. Chiffrentextes
// - Startbutton zum Verschlüsseln
// - Startbutton zum Signieren einer Nachricht (optional)
// - Startbutton zum Entschlüsseln eines von Bob empfangenen Chiffretextes und gleichzeitiges Verifizieren der Signatur von Bob
// - Button zum Versenden einer chiffrierten Nachricht an Bob mit optional beigefügter Signatur
// - Feld zum Anzeigen der Signaturen
// - Feld zur Übersicht ob Signatur gültig oder ungültig
// - Button zum Clearen aller Felder der Maske mit Ausnahme von d_a
// - Button um auf Hauptmaske zurückzukehren

// Das Feld für die Anzeige von Klar/Chiffrentext soll folgendes beinhalten:
// - Hier wird ein Klartext eingegeben
// - Nach dem Chiffrieren wird der Chiffrentext in dem Feld angezeigt und der ursprüngliche Klartext wird rausgelöscht
// - Nach dem Verschicken der Nachricht soll das Feld geleert werden
// - Wenn Nachricht von Bob empfangen wird, soll diese in diesem Feld ausgegeben werden
// - Nach dem Dechiffrieren soll der entschlüsselte Text angezeigt werden und der ursprünglich chiffrierte empfangene Text soll gelöscht werden

// - Das Feld zum Anzeigen von Signaturen soll folgende eigenschaften haben:
// - Die Signatur wird beim drücken des entsprechenden Buttons im lable angezeigt
// - Beim versenden der Nachricht wird das Signaturfeld geleert
// - die Signatur einer empfangenen chifrierten nachricht in dem Feld anzeigen lassen

// Maske Bob ist genauso aufgebaut wie die von Alice, nur dass hier eine Nachricht von Alice empfangen wird und eine an Alice verschickt werden kann

// -------------------------------------------------------------------------------------------------

// English Description of GUI functions
// Requirements for the GUI in Rust Rover:

// Three masks should be created:
// - Main mask
// - Alice -- for information from Alice
// - Bob -- for information from Bob

// - Only one mask can be displayed at a time, it should dynamically open to cover about 80% of the screen area when accessed

// Main mask for key generation:
// - Input option for the number of steps in the Miller-Rabin procedure
// - Input option for the length of prime numbers p_1 and p_2
// - Start button, which calls a math function, receiving both field information and writing a result into a label on this mask
// - Label to display the public keys for Alice and Bob (n,e_a) and (n,e_b)
// - Button to display Alice's mask
// - Button to display Bob's mask

// Mask Alice:
// - Display of the secret key for Alice (d_a)
// - Field for displaying plaintext or ciphertext
// - Start button for encryption
// - Start button for signing a message (optional)
// - Start button for decrypting a ciphertext received from Bob and simultaneously verifying Bob's signature
// - Button to send an encrypted message to Bob with an optional attached signature
// - Field for displaying signatures
// - Field for overview whether signature is valid or invalid
// - Button to clear all fields of the mask except for d_a
// - Button to return to the main mask

// The field for displaying plain/ciphertext should contain the following:
// - Here a plaintext is entered
// - After encrypting, the ciphertext is displayed in the field and the original plaintext is erased
// - After sending the message, the field should be cleared
// - When a message from Bob is received, it should be displayed in this field
// - After decrypting, the decrypted text should be displayed and the originally encrypted received text should be erased

// The field for displaying signatures should have the following properties:
// - The signature is displayed in the label when the corresponding button is pressed
// - When sending a message, the signature field is cleared
// - Display the signature of a received encrypted message in the field

// Mask Bob is set up exactly like that of Alice, except that here a message from Alice is received and one can be sent to Alice

use crate::gui::controller::controller::AppController;
use crate::gui::model::model::{AppState, View};
use crate::gui::view::view::{build_alice_view, build_bob_view, build_haupt_menu};
use druid::{widget::ViewSwitcher, Widget, WidgetExt};

// UI Bau Funktion
pub fn build_ui() -> impl Widget<AppState> {
    ViewSwitcher::new(
        |data: &AppState, _env| data.current_view.clone(),
        |selector, _data, _env| {
            let selected_widget: Box<dyn Widget<_>> = match *selector {
                View::HauptMenu => Box::new(build_haupt_menu().lens(AppState::haupt_menu)),
                View::Alice => Box::new(build_alice_view().lens(AppState::alice)),
                View::Bob => Box::new(build_bob_view().lens(AppState::bob)),
            };
            selected_widget.boxed()
        },
    )
    .controller(AppController)
    .boxed()
}
