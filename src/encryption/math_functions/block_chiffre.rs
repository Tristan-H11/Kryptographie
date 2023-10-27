use std::str::FromStr;
use bigdecimal::num_bigint::BigUint;
use bigdecimal::{One, ToPrimitive, Zero};
use log::{debug, trace};


///
/// Methode, um einen String in eine Menge von gleich großen Blöcken in Dezimalform zu unterteilen.
///
/// # Argumente
/// * `m` - Der zu unterteilende String.
/// * `block_size` - Die Größe der Blöcke.
/// * `fill_blocks` - Gibt an, ob die Blöcke mit Leerzeichen aufgefüllt werden sollen.
///
/// # Rückgabe
/// * `Vec<BigUint>` - Die codierte Darstellung des Strings als vec der Summen.
///
pub(crate) fn create_blocks_from_string_enc(m: &str, block_size: usize, fill_blocks: bool) -> Vec<BigUint> {
    debug!("Erstelle Chiffre mit Blockgröße {} für {}", block_size, m);
    let b = split_into_blocks(m, block_size, fill_blocks);
    let i_vec = string_to_int_vec(b);
    let base = BigUint::from(55296u32); //todo -- basis auf mainview auslagern damit der
                                        // user diese bei bedarf ändern kann (nice to have)
    to_sum_vec(i_vec, &base)
}

// Neue funktion für Rückweg beim decodieren!!!
pub(crate) fn create_blocks_from_string_dec(m: &str, fill_blocks: bool) -> Vec<BigUint> {
    let parts: Vec<&str> = m.splitn(2, '\u{FE8D}').collect();
    let block_size = match usize::from_str(parts[0]) {
        Ok(size) => size,
        Err(_) => panic!("Ungültige Blockgröße im Eingabestring"),
    };
    let message = parts[1];
    debug!("Erstelle Chiffre mit Blockgröße {} für {}", block_size, message);

    let b = split_into_blocks(message, block_size, fill_blocks);
    let i_vec = string_to_int_vec(b);
    let base = BigUint::from(55296u32);
    to_sum_vec(i_vec, &base)
}

///
/// Methode, um eine Menge von gleich großen Blöcken in Dezimalform in einen String zu überführen.
///
/// # Argumente
/// * `sums` - Die zu überführenden Summen.
///
/// # Rückgabe
/// * `String` - Der decodierte String.
///
pub(crate) fn create_string_from_blocks(sums: Vec<BigUint>) -> String {
    println!("Erstelle String aus Vektor von Summen: Anzahl der Vectorblöcke --> {}", sums.len());
    let base = BigUint::from(55296u32);
    let mut strings = sums_vec_to_string_vec(sums, &base);
    println!("Chiffrierter Vector: {:?}", strings);

    // todo -- tristan --> in Zeile 55 wird 24, 25 oder 25,25 usw. als interne Zahlenwert übergeben.
    // folglich bekommen wir hier nicht die richtige länge aus den "strings", da diese anders
    //berechnet werden. Wir brauchen eine länge wie 8 oder 10 oder so, also von dem chinesischen
    //zeichentext, damit wir es danach vernünftig splitten können.
    //wenn hier der richtige wert übergeben werden sollte, müsste das gesamte RSA funktionieren.
    let max_length = &strings.iter()
        .map(|s| s.len()).max().unwrap();
    println!("Maximale Länge eines Strings: {}", max_length);

    // Füllen Sie jeden String mit dem Zeichen "\u{FE8D}", um die maximale Länge zu erreichen
    // -- ziel ist es, eine einheitliche blocksize zu erhalten
    strings = strings.iter()
        .map(|s| format!("{}{}", s, "\u{FE8D}".repeat(max_length - s.len())))
        .collect();
    format!("{}\u{FE8D}{}", max_length, &strings.join(""))
}

///
/// # Nur zu Testzwecken öffentlich!
/// Methode, um einen String in eine Menge von gleich großen Blöcken zu unterteilen.
/// Nicht-volle Blöcke werden mit Space (' ') aufgefüllt.
///
/// # Argumente
/// * `message` - Der zu unterteilende String.
/// * `block_size` - Die Größe der Blöcke.
/// * `fill_block` - Gibt an, ob die Blöcke mit Leerzeichen aufgefüllt werden sollen.
///
/// # Rückgabe
/// * `Vec<String>` - Die Menge der Blöcke als Vector.
///
/// # Beispiel
/// Beispiel von Seite 20 IT-Sec Skript:
/// ```
/// split_into_blocks("Das ist eine Testnachricht", 4)
/// ["Das ", "ist ", "eine", " Tes", "tnac", "hric", "ht  "]
/// ```
pub(crate) fn split_into_blocks(message: &str, block_size: usize, fill_block: bool) -> Vec<String> {
    println!("Erstelle Blöcke mit Blockgröße {} für '{}'", block_size, message);
    message
        .chars()
        .collect::<Vec<char>>()
        .chunks(block_size) //Definiert die Blockgröße im Vector
        .map(|c| {
            // Durchlaufe alle chunks, im letzten muss du ggf. Leerzeichen auffüllen
            let mut b = c.iter().collect::<String>(); // .iter --> füge chars zu String zusammen
            if fill_block {
                while b.len() < block_size {
                    b.push(' '); // Fügt Leerzeichen hinzu, um den letzten Block zu füllen
                }
            }
            println!("Erstellte Block '{}'", b);
            b
        })
        .collect() // Fasst alle Blöcke im Vektor zusammen
}

///
/// # Nur zu Testzwecken öffentlich!
/// Methode, um den Vector mit seinen Strings in einen Vector mit Integern zu überführen.
///
/// # Argumente
/// * `b_vec` - Der zu überführende Vec<String>.
///
/// # Rückgabe
/// * `Vec<Vec<u32>>` - Die codierte Darstellung des Strings als integer.
///
/// # Beispiel
/// Beispiel von Seite 21 IT-Sec Skript:
/// ```
/// string_to_int_vec("["Das ", "ist ", "eine", " Tes", "tnac", "hric", "ht  "]")
/// vec![
///             vec![char_to_u16('D'), char_to_u16('a'), char_to_u16('s'), char_to_u16(' ')],
///             vec![char_to_u16('i'), char_to_u16('s'), char_to_u16('t'), char_to_u16(' ')],
///             vec![char_to_u16('e'), char_to_u16('i'), char_to_u16('n'), char_to_u16('e')],
///             vec![char_to_u16(' '), char_to_u16('T'), char_to_u16('e'), char_to_u16('s')],
///             vec![char_to_u16('t'), char_to_u16('n'), char_to_u16('a'), char_to_u16('c')],
///             vec![char_to_u16('h'), char_to_u16('r'), char_to_u16('i'), char_to_u16('c')],
///             vec![char_to_u16('h'), char_to_u16('t'), char_to_u16(' '), char_to_u16(' ')],
///         ];
/// ```
///
pub(crate) fn string_to_int_vec(b_vec: Vec<String>) -> Vec<Vec<u32>> {
    println!("Erstelle Integer Vektor aus String Vektor");
    b_vec.into_iter().map(|b| {
        let vec = b.chars().map(|b| b as u32).collect();
        println!("Erstelle Integer Vektor aus String Vektor: {:?}", vec);
        vec
    }).collect()
}

///
/// # Nur zu Testzwecken öffentlich!
/// Methode, um einen Vektor von Integern als g-adische Zahl zu interpretieren
/// und in eine Dezimalzahl zu überführen.
///
/// # Argumente
/// * `d_vec` - Der zu überführende Vec<Vec<u32>>.
///
/// # Rückgabe
/// * `Vec<BigUint>` - Die codierte Darstellung des Strings als vec der Summen.
/// vec![
///             BigUint::from(19140715035688992u64),
///             BigUint::from(29555366483460128u64),
///             BigUint::from(28429423626551397u64),
///             BigUint::from(9007560038613107u64),
///             BigUint::from(32651569751195747u64),
///             BigUint::from(29273887211061347u64),
///             BigUint::from(29273895796211744u64),
///         ];
///
/// # Beispiel
/// Beispiel von Seite 21 IT-Sec Skript:
/// ```
pub(crate) fn to_sum_vec(d_vec: Vec<Vec<u32>>, base: &BigUint) -> Vec<BigUint> {
    println!("Erstelle Summen Vektor aus Integer Vektor");
    d_vec.into_iter().map(|d| helper_fun_sum_for_digits(&d, base)).collect()
}

fn helper_fun_sum_for_digits(i_vec: &Vec<u32>, g_base: &BigUint) -> BigUint {
    println!("Erstelle Summe aus Integer Vektor");
    let mut sum = BigUint::zero();
    let mut base = BigUint::one();
    for &digit in i_vec.iter().rev() {
        println!("Addiere {} * {} zu Summe", base, digit);
        sum += &base * BigUint::from(digit);
        base *= g_base;
    }
    println!("Summe: {}", sum);
    sum
}

///
/// # Nur zu Testzwecken öffentlich!
/// Methode, um eine Dezimalzahl in einen String (g-adisch) zu überführen.
///
/// # Argumente
/// * `sums` - Der zu überführende Vec<BigUint>.
/// * `base` - Die Basis des g-adischen Systems.
///
/// # Rückgabe
/// * `String` - Vector der Strings.
///         let expected_result = vec![
///             "Das ".to_string(),
///             "ist ".to_string(),
///             "eine".to_string(),
///             " Tes".to_string(),
///             "tnac".to_string(),
///             "hric".to_string(),
///             "ht  ".to_string(),
///         ];
///
///
pub(crate) fn sums_vec_to_string_vec(sums: Vec<BigUint>, base: &BigUint) -> Vec<String> {
    sums.into_iter()
        .map(|sum|helper_fun_sum_to_string(&sum, base))
        .collect()
}

fn helper_fun_sum_to_string(sum: &BigUint, base: &BigUint) -> String {
    let mut t_sum = sum.clone();
    let mut res = String::new();
    let zero = BigUint::zero();

    // Konvertiere die Summe in ein g-adisches System zu Basis base
    while t_sum > zero {
        let digit = &t_sum % base;
        println!("{} % {} = {} ", t_sum, base, digit);
        println!("--> {}\n", char::from_u32(digit.to_u32().unwrap()).unwrap());
        t_sum = &t_sum / base;
        res.push(u32_to_c(ubig_to_u32(&digit)));
    }
    res.chars().rev().collect()
}

///
/// Konvertiere ein u32 Code in ein Zeichen -- z.B. für Blockchiffre
///
pub(crate) fn u32_to_c(value: u32) -> char {
    match char::from_u32(value) {
        Some(x) => x,
        None => panic!("oben Ungültiger u32 Wert: {}", value),
    }
}

///
/// wandle eine ubig Zahl in einen u32 Wert um
///
pub(crate) fn ubig_to_u32(value: &BigUint) -> u32 {
    let value_str = format!("{}", value);
    match value_str.parse::<u32>(){
        Ok(x) => x,
        Err(_) => panic!("unten Ungültiger u32 Wert: {}", value),
    }
}

