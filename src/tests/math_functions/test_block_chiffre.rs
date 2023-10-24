#[cfg(test)]
mod tests {
    use crate::encryption;
    use crate::encryption::math_functions::block_chiffre::{
        digits_from_vec_to_sum, split_into_blocks, string_to_int_vec, sum_to_string,
    };
    use bigdecimal::num_bigint::BigUint;
    use bigdecimal::FromPrimitive;

    #[test]
    fn test_split_into_blocks() {
        // Testfall 1: Ein einfacher String wird in Blöcke der Größe 4 aufgeteilt.
        let message = String::from("Das ist eine Testnachricht");
        let block_size = 4;
        let result = split_into_blocks(&message, block_size);
        assert_eq!(
            result,
            vec!["Das ", "ist ", "eine", " Tes", "tnac", "hric", "ht  "]
        );

        // Testfall 2: Ein String, der bereits eine Blockgröße hat, wird nicht verändert,
        // es kommt kein neuer leerer Block dazu.
        let message = String::from("123AB");
        let block_size = 5;
        let result = split_into_blocks(&message, block_size);
        assert_eq!(result, vec!["123AB"]);

        // Testfall 3: Ein leerer String wird in Blöcke der Größe 3 aufgeteilt.
        let message = String::from("   ");
        let block_size = 3;
        let result = split_into_blocks(&message, block_size);
        assert_eq!(result, vec!["   "]);

        // Testfall 4: Ein String wird in Blöcke der Größe 1 aufgeteilt.
        let message = String::from("abcdef");
        let block_size = 1;
        let result = split_into_blocks(&message, block_size);
        assert_eq!(result, vec!["a", "b", "c", "d", "e", "f"]);
    }

    #[test]
    fn test_string_to_int_vec() {
        let message = "abc, XYZ012".to_string();
        let block_size = 3;
        let blocks = split_into_blocks(&message, block_size); // erstellen des Vektors
                                                              // aus Blöcken
        let expectet_chiffre_results_as_blocks_in_vec: Vec<Vec<u32>> = vec![
            vec![0, 1, 2],
            vec![63, 76, 49],
            vec![50, 51, 52],
            vec![53, 54, 76],
        ];

        for (block, expected_vec) in blocks
            .iter()
            .zip(expectet_chiffre_results_as_blocks_in_vec.iter())
        {
            assert_eq!(
                string_to_int_vec(block),
                *expected_vec,
                "Fehler bei Block: {}",
                block
            );
        }
    }

    #[test]
    fn test_digits_from_vec_to_sum() {
        // Testfall 1: Zahlen in umgekehrter Reihenfolge und Basis 47.
        let digits = vec![12, 0, 19, 7, 4, 12, 0, 19];
        let base = 47;
        let result = digits_from_vec_to_sum(&digits, base);
        let expected_result = BigUint::from(6083869600275u64);
        assert_eq!(result, expected_result);

        // Testfall 2: Basis 2 und Binärzahlen.
        let digits = vec![1, 0, 1, 0, 1, 0];
        let base = 2;
        let result = digits_from_vec_to_sum(&digits, base);
        let expected_result = BigUint::from(42u32);
        assert_eq!(result, expected_result);

        // Testfall 3: Basis 16 und Hexadezimalzahlen.
        let digits = vec![13, 10, 15]; // [D, A, F]
        let base = 16;
        let result = digits_from_vec_to_sum(&digits, base);
        let expected_result = BigUint::from(3503u32);
        assert_eq!(expected_result, expected_result);
    }

    #[test]
    fn test_sum_to_string() {
        let sum = BigUint::from_u64(65537).unwrap();
        let expected = "AA"; // Der erwartete String für den Wert 65537 in Basis 65536
        let result = sum_to_string(&sum);
        assert_eq!(
            result, expected,
            "Die Funktion sum_to_string hat einen unerwarteten Wert zurückgegeben"
        );
    }
}
