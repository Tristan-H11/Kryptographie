#[cfg(test)]
mod tests {
    use crate::encryption::math_functions::block_chiffre::{digits_from_vec_to_sum, int_vec_to_string, split_into_blocks,
        string_to_int_vec, string_to_sum, sum_to_digits, sum_to_string,
    };
    use crate::encryption::math_functions::big_int_util::{char_to_u16, u16_to_char, ubig_to_u16};
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
        let message = "abcXYZ012";
        let expected = vec![
            char_to_u16('a'),
            char_to_u16('b'),
            char_to_u16('c'),
            char_to_u16('X'),
            char_to_u16('Y'),
            char_to_u16('Z'),
            char_to_u16('0'),
            char_to_u16('1'),
            char_to_u16('2'),
        ];
        let result = string_to_int_vec(&message);
        assert_eq!(result, expected);
    }

    //todo -- tristan, check ob diese funktion auch so für den RSA benötigt wird, da jetzt basis 16
    // im code ist und nicht 2^16
    #[test]
    fn test_digits_from_vec_to_sum() {
        let digits = vec![1, 51, 22];
        let result = digits_from_vec_to_sum(&digits);
        let expected_result = BigUint::from(1094_u32);
        // Vektor umkehren
        // Summe= 1*16^2 + 51*16^1 + 22*16^0
        assert_eq!(result, expected_result);
    }


    #[test]
    fn test_sum_to_string() {
        let sum = BigUint::from_u32(1094).unwrap();
        let expected = "8a";
        let result = sum_to_string(&sum);
        assert_eq!(
            result, expected,
            "Die Funktion sum_to_string hat einen unerwarteten Wert zurückgegeben"
        );
    }

    #[test]
    fn test_string_to_sum() {
        let message = "8a";
        let expected_sum = BigUint::from_u64(138).unwrap();
        let result_sum = string_to_sum(&message);
        assert_eq!(result_sum, expected_sum);
    }
}
