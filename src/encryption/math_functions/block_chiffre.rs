use crate::encryption::math_functions::big_int_util::{char_to_u32, u32_to_char, ubig_to_u32};
use ibig::{ubig, UBig};

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
pub(crate) fn split_into_blocks(message: &str, block_size: usize) -> Vec<String> {
    message
        .chars()
        .collect::<Vec<char>>() //Erstelle einen Vektor für die Blöcke bestehend aus Zeichen
        .chunks(block_size) //Definiert die Blockgröße im Vector
        .map(|chunk| {
            // Durchlaufe alle chunks, im letzten muss du ggf. Leerzeichen auffüllen
            let mut block = chunk.iter().collect::<String>(); // .iter --> füge chars zu String zusammen
            while block.len() < block_size {
                block.push(' '); // Fügt Leerzeichen hinzu, um den letzten Block zu füllen
            }
            block
        })
        .collect() // Falls alle Blöcke im Vektor zusammen
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
pub(crate) fn string_to_int_vec(message: &str) -> Vec<u32> {
    message.chars().map(char_to_u32).collect()
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
/// ) // 6083869600275
pub(crate) fn digits_from_vec_to_sum(digits: &Vec<u64>, g_base: u16) -> UBig {
    let mut sum = ubig!(0);
    let mut base = ubig!(1);
    for &digit in digits.iter().rev() {
        // [12, 2, 0, 5] --> 12 * 47^3 + 2 * 47^2 + 0 * 47^1 + 5 * 47^0
        sum += &base * digit;
        base *= g_base;
    }
    sum
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
/// TODO: Hier muss später das `g` rausgenommen werden, wenn die Margitta uns gesagt hat,
/// welcher Buchstabe welchen Wert hat.
pub(crate) fn sum_to_string(sum: &UBig, g: u32) -> String {
    let mut temp_sum = sum.clone();
    let mut result = String::new();
    let base = ubig!(g);

    while temp_sum > ubig!(0) {
        let remainder = ubig_to_u32(&(&temp_sum % &base));
        result.push(u32_to_char(remainder));
        temp_sum = temp_sum / &base;
    }
    result.chars().rev().collect()
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
pub(crate) fn string_to_sum(message: &str, g: u32) -> UBig {
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
pub(crate) fn sum_to_digits(sum: &UBig, g: u32) -> Vec<u32> {
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
pub(crate) fn int_vec_to_string(int_vec: &Vec<u32>) -> String {
    todo!("Implementiere diese Funktion!")
}
