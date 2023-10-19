use ibig::UBig;

// TODO: Öffentliche Funktionen implementieren, weil der Rest hier unten nur für interne Zwecke ist.

///
/// Methode, um einen String in eine Menge von gleich großen Blöcken zu unterteilen.
/// Nicht-volle Blöcke werden mit Space (\s) aufgefüllt.
///
/// # Argumente
/// * `message` - Der zu unterteilende String.
/// * `block_size` - Die Größe der Blöcke.
///
/// # Rückgabe
/// * `Vec<String>` - Die Menge der Blöcke.
///
/// # Beispiel
/// Beispiel von Seite 20 IT-Sec Skript:
/// ```
/// split_into_blocks("MATHEMATIK*IST*SPANNEND!", 8)
/// // ["MATHEMAT", "IK*IST*S", "PANNEND!"]
/// ```
fn split_into_blocks(message: &String, block_size: usize) -> Vec<String> {
    todo!("Implementiere diese Funktion!")
}

///
/// Methode, um einen String in einen Vektor von Integern zu überführen.
///
/// # Argumente
/// * `message` - Der zu überführende String.
///
/// # Rückgabe
/// * `Vec<u32>` - Die codierte Darstellung des Strings.
///
/// # Beispiel
/// Beispiel von Seite 21 IT-Sec Skript:
/// ```
/// string_to_int_vec("MATHEMAT") // [12,0,19,7,4,12,0,19]
/// ```
///
fn string_to_int_vec(message: &String) -> Vec<u32> {
    todo!("Implementiere diese Funktion!")
}

///
/// Methode, um einen Vektor von Integern als g-adische Zahl zu interpretieren
/// und in eine Dezimalzahl zu überführen.
///
/// # Argumente
/// * `digits` - Der zu überführende Vektor.
/// * `g` - Die Basis des g-adischen Systems.
///
/// # Rückgabe
/// * `UBig` - Die Summe des g-adischen Systems.
///
/// # Beispiel
/// Beispiel von Seite 21 IT-Sec Skript:
/// ```
/// digits_to_sum(
///     vec![12,0,19,7,4,12,0,19],
///     47
/// ) // 6.083.869.600.275
fn digits_to_sum(digits: &Vec<u32>, g: u32) -> UBig {
    todo!("Implementiere diese Funktion!")
}

///
/// Methode, um eine Dezimalzahl in einen String (g-adisch) zu überführen.
///
/// # Argumente
/// * `sum` - Die zu überführende Summe.
/// * `g` - Die Basis des g-adischen Systems.
///
/// # Rückgabe
/// * `String` - Der String.
///
/// # Beispiel
/// Beispiel von Seite 21 IT-Sec Skript:
/// ```
/// sum_to_string(ubig!(422.078.969.854.681), 47) // "R8F9BX-YO"
/// ```
///
fn sum_to_string(sum: &UBig, g: u32) -> String {
    todo!("Implementiere diese Funktion!")
}

///
/// Methode, um einen String (g-adisch) in seine Dezimaldarstellung zu überführen.
///
/// # Argumente
/// * `message` - Der zu überführende String.
/// * `g` - Die Basis des g-adischen Systems.
///
/// # Rückgabe
/// * `UBig` - Die Dezimaldarstellung des Strings.
///
/// # Beispiel
/// Beispiel von Seite 21 IT-Sec Skript:
/// ```
/// string_to_sum("R8F9BX-YO", 47) // 422.078.969.854.681
/// ```
///
fn string_to_sum(message: &String, g: u32) -> UBig {
    todo!("Implementiere diese Funktion!")
}

///
/// Methode, um eine Dezimalzahl in eine Menge von Ziffern zu überführen.
/// Die Ziffern sind die Koeffizienten der g-adischen Darstellung der Zahl.
///
/// # Argumente
/// * `sum` - Die zu überführende Summe.
/// * `g` - Die Basis des g-adischen Systems.
///
/// # Rückgabe
/// * `Vec<u32>` - Die Menge der Koeffizienten.
///
/// # Beispiel
/// Beispiel von Seite 21 IT-Sec Skript:
/// ```
/// sum_to_digits(ubig!(422.078.969.854.681), 47) // [17,34,5,35,1,23,40,24,14]
/// ```
///
fn sum_to_digits(sum: &UBig, g: u32) -> Vec<u32> {
    todo!("Implementiere diese Funktion!")
}

///
/// Methode, um einen Vektor von Integern in einen String zu überführen.
///
/// # Argumente
/// * `int_vec` - Der zu überführende Vektor.
///
/// # Rückgabe
/// * `String` - Der String.
///
/// # Beispiel
/// Beispiel von Seite 21 IT-Sec Skript:
/// ```
/// int_vec_to_string(&vec![12,0,19,7,4,12,0,19]) // "MATHEMAT"
///
fn int_vec_to_string(int_vec: &Vec<u32>) -> String {
    todo!("Implementiere diese Funktion!")
}
