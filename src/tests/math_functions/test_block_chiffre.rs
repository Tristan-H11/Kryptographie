#[cfg(test)]
mod tests {
    use bigdecimal::num_bigint::BigInt;

    use crate::big_i;
    use crate::encryption::math_functions::block_chiffre::{
        create_blocks_from_string_decrypt, create_blocks_from_string_encript,
        create_string_from_blocks, create_string_from_blocks_decrypt, split_into_blocks,
        string_to_int_vec, sums_vec_to_string_vec, to_sum_vec, u32_to_c, ubig_to_u32,
    };
    use crate::encryption::math_functions::number_theory::fast_exponentiation;
    use crate::encryption::rsa::rsa_keygen_service::RsaKeygenService;

    ///
    /// Test um zu prüfen, ob ein String aufgeteilt, manipuliert, zusammengesetzt und wieder umgekehrt werden kann.
    /// Dafür wird der String zerlegt, die Zahl verdoppelt und ein Ciphertext darauf erstellt.
    /// Dieser Cipher wird dann auch wieder zerlegt, die Zahl halbiert und ein Plaintext erstellt.
    /// Dieser Plaintext wird dann wieder zusammengesetzt und sollte dem ursprünglichen String entsprechen.
    ///
    #[test]
    fn test_loop_create_mult_decode_create_div_decode_1() {
        let mut failure_count = 0;

        for _ in 0..12 {
            let keygen_service = RsaKeygenService::new(128);
            let (public_key, private_key) = keygen_service.generate_keypair(40);

            let message = "Das ist ein langer Test um etwas zu prüfen. Dieser Text wird jetzt einfach mal echt wirklich lang. 0123456789";
            let basis_length = 55296 as u32;
            println!("{}", message);

            //            let result = create_blocks_from_string(message, public_key.block_size - 1, true)
            let result = create_blocks_from_string_encript(message, 2, true, basis_length)
                .iter()
                .map(|x| {
                    let zwischenstand = fast_exponentiation(
                        x,
                        &public_key.get_e_for_test(),
                        &public_key.get_n_for_test(),
                    ); //verschlüsseln
                    println!(
                        "{}^{} mod {} = {}",
                        x,
                        public_key.get_e_for_test(),
                        public_key.get_n_for_test(),
                        zwischenstand
                    );
                    zwischenstand
                })
                .collect::<Vec<BigInt>>();
            println!("\nVerschlüsselte Nachricht als RSA Vector: {:?}\n", result);

            let encrypted_string = create_string_from_blocks(result);
            println!("Verschlüsselter String: {}\n", encrypted_string);

            // Ohne Blocklänge, da diese in der Methode aus dem String extrahiert wird
            let result = create_blocks_from_string_decrypt(&encrypted_string, true, basis_length)
                .iter()
                .map(|x| {
                    fast_exponentiation(x, &private_key.get_d(), &private_key.get_n())
                    //entschlüsseln
                })
                .collect();
            println!("\nEntschlüsselte Nachricht: {:?}\n", result);

            let string = create_string_from_blocks_decrypt(result);

            // Ersetze assert durch eine if-Anweisung
            if string.trim() != message {
                failure_count += 1;
            }
        }

        // Am Ende des Tests, prüfe, ob der Fehlerzähler 0 ist
        assert_eq!(failure_count, 0, "Fehlgeschlagene Tests: {}", failure_count);
        print!("{} : Tests sind fehlgeschlagen", failure_count);
    }

    #[test]
    fn test_loop_create_mult_decode_create_div_decode_2() {
        let mut failure_count = 0;

        for _ in 0..12 {
            let keygen_service = RsaKeygenService::new(128);
            let (public_key, private_key) = keygen_service.generate_keypair(40);
            let message = "Das ist ein langer Test um etwas zu prüfen. Dieser Text wird jetzt einfach mal echt wirklich lang. 0123456789";
            let basis_length = 55296 as u32;
            let result = create_blocks_from_string_encript(message, 8, true, basis_length)
                .iter()
                .map(|x| {
                    let zwischenstand = fast_exponentiation(
                        x,
                        &public_key.get_e_for_test(),
                        &public_key.get_n_for_test(),
                    ); //verschlüsseln
                    zwischenstand
                })
                .collect::<Vec<BigInt>>();
            let encrypted_string = create_string_from_blocks(result);
            let result = create_blocks_from_string_decrypt(&encrypted_string, true, basis_length)
                .iter()
                .map(|x| {
                    fast_exponentiation(x, &private_key.get_d(), &private_key.get_n())
                    //entschlüsseln
                })
                .collect();
            let string = create_string_from_blocks_decrypt(result);
            if string.trim() != message {
                failure_count += 1;
            }
        }
        assert_eq!(failure_count, 0, "Fehlgeschlagene Tests: {}", failure_count);
        print!("{} : Tests sind fehlgeschlagen", failure_count);
    }

    #[test]
    fn test_loop_create_mult_decode_create_div_decode_3() {
        let mut failure_count = 0;
        for _ in 0..12 {
            let keygen_service = RsaKeygenService::new(256);
            let (public_key, private_key) = keygen_service.generate_keypair(40);
            let message = "Das ist ein langer Test um etwas zu prüfen. Dieser Text wird jetzt einfach mal echt wirklich lang. 0123456789";
            let basis_length = 55296 as u32;
            let result = create_blocks_from_string_encript(message, 13, true, basis_length)
                .iter()
                .map(|x| {
                    let zwischenstand = fast_exponentiation(
                        x,
                        &public_key.get_e_for_test(),
                        &public_key.get_n_for_test(),
                    ); //verschlüsseln
                    zwischenstand
                })
                .collect::<Vec<BigInt>>();
            let encrypted_string = create_string_from_blocks(result);
            let result = create_blocks_from_string_decrypt(&encrypted_string, true, basis_length)
                .iter()
                .map(|x| {
                    fast_exponentiation(x, &private_key.get_d(), &private_key.get_n())
                    //entschlüsseln
                })
                .collect();
            let string = create_string_from_blocks_decrypt(result);
            if string.trim() != message {
                failure_count += 1;
            }
        }
        assert_eq!(failure_count, 0, "Fehlgeschlagene Tests: {}", failure_count);
        print!("{} : Tests sind fehlgeschlagen", failure_count);
    }

    ///
    /// Prüft, ob die Funktionen zum Zerteilen und Zusammensetzen eines String das Inverse voneinander sind.
    ///
    #[test]
    fn test_create_block_umkehrfunktion_create_string() {
        let m = "Da苉 ist eine Testnachricht";
        let block_size = 8;
        let basis_length = 55296 as u32;
        let encoded = create_string_from_blocks(create_blocks_from_string_encript(
            m,
            block_size,
            true,
            basis_length,
        ));
        let decoded = create_string_from_blocks_decrypt(create_blocks_from_string_decrypt(
            &encoded,
            true,
            basis_length,
        ));
        assert_eq!(decoded.trim(), m);

        let m = "Da苉 ist eine Testnachricht";
        let block_size = 6;
        let encoded = create_string_from_blocks(create_blocks_from_string_encript(
            m,
            block_size,
            true,
            basis_length,
        ));
        let decoded = create_string_from_blocks_decrypt(create_blocks_from_string_decrypt(
            &encoded,
            true,
            basis_length,
        ));
        assert_eq!(decoded.trim(), m);

        let m = "Da苉 ist eine Testnachricht";
        let block_size = 47;
        let encoded = create_string_from_blocks(create_blocks_from_string_encript(
            m,
            block_size,
            true,
            basis_length,
        ));
        let decoded = create_string_from_blocks_decrypt(create_blocks_from_string_decrypt(
            &encoded,
            true,
            basis_length,
        ));
        assert_eq!(decoded.trim(), m);

        let m = "Da苉 ist eine Testnachricht";
        let block_size = 3;
        let encoded = create_string_from_blocks(create_blocks_from_string_encript(
            m,
            block_size,
            true,
            basis_length,
        ));
        let decoded = create_string_from_blocks_decrypt(create_blocks_from_string_decrypt(
            &encoded,
            true,
            basis_length,
        ));
        assert_eq!(decoded.trim(), m);
    }

    #[test]
    fn test_create_chiffre() {
        let message = "Da苉 ist eine Testnachricht";
        let block_size = 7;
        let basis_length = 55296 as u32;
        let result = create_blocks_from_string_encript(message, block_size, true, basis_length);
        let expected_result = vec![
            big_i!(1943938337267550087026074257524),
            big_i!(914822981356602019800946507860),
            big_i!(2887304683313907978613082523752),
            big_i!(3258925137110102081877384560672),
        ];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_decrypt_chiffre() {
        let sums = vec![
            big_i!(1943938337267550087026074257524),
            big_i!(914822981356602019800946507860),
            big_i!(2887304683313907978613082523752),
            big_i!(3258925137110102081877384560672),
        ];
        let result = create_string_from_blocks_decrypt(sums);
        let expected_result = "Da苉 ist eine Testnachricht".to_string();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_split_into_blocks() {
        // Testfall 1: Ein einfacher String wird in Blöcke der Größe 4 aufgeteilt.
        let message = String::from("Da苉 ist eine Testnachricht");
        let block_size = 4;
        let result = split_into_blocks(&message, block_size, true);
        assert_eq!(
            result,
            vec!["Da苉 ", "ist ", "eine", " Tes", "tnac", "hric", "ht  "]
        );

        // Testfall 2: Ein String, der bereits eine Blockgröße hat, wird nicht verändert,
        // es kommt kein neuer leerer Block dazu.
        let message = String::from("123AB");
        let block_size = 5;
        let result = split_into_blocks(&message, block_size, true);
        assert_eq!(result, vec!["123AB"]);

        // Testfall 3: Ein leerer String wird in Blöcke der Größe 3 aufgeteilt.
        let message = String::from("   ");
        let block_size = 3;
        let result = split_into_blocks(&message, block_size, true);
        assert_eq!(result, vec!["   "]);

        // Testfall 4: Ein String wird in Blöcke der Größe 1 aufgeteilt.
        let message = String::from("abcdef");
        let block_size = 1;
        let result = split_into_blocks(&message, block_size, true);
        assert_eq!(result, vec!["a", "b", "c", "d", "e", "f"]);
    }

    #[test]
    fn test_string_to_int_vec() {
        let message = "Da苉 ist eine Testnachricht ";
        let blocks = split_into_blocks(&message, 4, true);
        let expected = vec![
            vec!['D' as u32, 'a' as u32, '苉' as u32, ' ' as u32],
            vec!['i' as u32, 's' as u32, 't' as u32, ' ' as u32],
            vec!['e' as u32, 'i' as u32, 'n' as u32, 'e' as u32],
            vec![' ' as u32, 'T' as u32, 'e' as u32, 's' as u32],
            vec!['t' as u32, 'n' as u32, 'a' as u32, 'c' as u32],
            vec!['h' as u32, 'r' as u32, 'i' as u32, 'c' as u32],
            vec!['h' as u32, 't' as u32, ' ' as u32, ' ' as u32],
        ];
        let result = string_to_int_vec(blocks);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_digits_from_vec_to_sum() {
        let digit_vectors = vec![
            vec!['D' as u32, 'a' as u32, '苉' as u32, ' ' as u32],
            vec!['i' as u32, 's' as u32, 't' as u32, ' ' as u32],
            vec!['e' as u32, 'i' as u32, 'n' as u32, 'e' as u32],
            vec![' ' as u32, 'T' as u32, 'e' as u32, 's' as u32],
            vec!['t' as u32, 'n' as u32, 'a' as u32, 'c' as u32],
            vec!['h' as u32, 'r' as u32, 'i' as u32, 'c' as u32],
            vec!['h' as u32, 't' as u32, ' ' as u32, ' ' as u32],
        ];

        let base = big_i!(55296);
        let result = to_sum_vec(digit_vectors, &base);

        let expected_result = vec![
            big_i!(11497444858239008),
            big_i!(17753298306195488),
            big_i!(17076964999090277),
            big_i!(5410678690363507),
            big_i!(19613115525224547),
            big_i!(17584219565365347),
            big_i!(17584225676623904),
        ];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_sum_to_strings() {
        let sums = vec![
            big_i!(11497444858239008),
            big_i!(17753298306195488),
            big_i!(17076964999090277),
            big_i!(5410678690363507),
            big_i!(19613115525224547),
            big_i!(17584219565365347),
            big_i!(17584225676623904),
        ];

        let base = big_i!(55296);
        let result = sums_vec_to_string_vec(sums, &base);

        let expected_result = vec![
            "Da苉 ".to_string(),
            "ist ".to_string(),
            "eine".to_string(),
            " Tes".to_string(),
            "tnac".to_string(),
            "hric".to_string(),
            "ht  ".to_string(),
        ];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_join_strings() {
        let input = vec![
            "Da苉 ".to_string(),
            "ist ".to_string(),
            "eine".to_string(),
            " Tes".to_string(),
            "tnac".to_string(),
            "hric".to_string(),
            "ht  ".to_string(),
        ];

        let result = input.join("");

        let expected_result = "Da苉 ist eine Testnachricht  ".to_string();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_char_to_u32() {
        assert_eq!('a' as u32, 97); // Unicode
        assert_eq!('b' as u32, 98);
        assert_eq!('z' as u32, 122);
        assert_eq!('A' as u32, 65);
        assert_eq!('B' as u32, 66);
        assert_eq!('Z' as u32, 90);
        assert_eq!('0' as u32, 48);
        assert_eq!('1' as u32, 49);
        assert_eq!('9' as u32, 57);
    }

    #[test]
    fn test_u32_to_char() {
        assert_eq!(u32_to_c(97), 'a');
        assert_eq!(u32_to_c(122), 'z');
        assert_eq!(u32_to_c(65), 'A');
        assert_eq!(u32_to_c(90), 'Z');
        assert_eq!(u32_to_c(48), '0');
        assert_eq!(u32_to_c(57), '9');
        assert_eq!(u32_to_c(46), '.');
        assert_eq!(u32_to_c(44), ',');
    }

    #[test]
    fn test_ubig_to_u32() {
        let value = big_i!(12345);
        let result = ubig_to_u32(&value);
        assert_eq!(result, 12345);
    }
}
