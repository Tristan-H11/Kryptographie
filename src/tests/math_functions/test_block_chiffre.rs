#[cfg(test)]
mod tests {
    use crate::encryption::math_functions::block_chiffre::{digits_from_vec_to_sum, int_vec_to_string, split_into_blocks, string_to_int_vec, string_to_sum, sum_to_digits, sum_to_string};
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
        let message = "Das ist eine Testnachricht ";
        let blocks = split_into_blocks(&message, 4);
        let expected = vec![
            vec![char_to_u16('D'), char_to_u16('a'), char_to_u16('s'), char_to_u16(' ')],
            vec![char_to_u16('i'), char_to_u16('s'), char_to_u16('t'), char_to_u16(' ')],
            vec![char_to_u16('e'), char_to_u16('i'), char_to_u16('n'), char_to_u16('e')],
            vec![char_to_u16(' '), char_to_u16('T'), char_to_u16('e'), char_to_u16('s')],
            vec![char_to_u16('t'), char_to_u16('n'), char_to_u16('a'), char_to_u16('c')],
            vec![char_to_u16('h'), char_to_u16('r'), char_to_u16('i'), char_to_u16('c')],
            vec![char_to_u16('h'), char_to_u16('t'), char_to_u16(' '), char_to_u16(' ')]
        ];
        let result = string_to_int_vec(blocks);
        assert_eq!(result, expected);
    }


    #[cfg(test)]
    mod tests {
        use super::*;
        use bigdecimal::num_bigint::BigUint;
        use bigdecimal::ToPrimitive;
        use crate::encryption::math_functions::block_chiffre::help_fun_sum_for_digits;

        #[test]
        fn test_digits_from_vec_to_sum() {
            let digit_vectors = vec![
                vec![char_to_u16('D'), char_to_u16('a'), char_to_u16('s'), char_to_u16(' ')],
                vec![char_to_u16('i'), char_to_u16('s'), char_to_u16('t'), char_to_u16(' ')],
                vec![char_to_u16('e'), char_to_u16('i'), char_to_u16('n'), char_to_u16('e')],
                vec![char_to_u16(' '), char_to_u16('T'), char_to_u16('e'), char_to_u16('s')],
                vec![char_to_u16('t'), char_to_u16('n'), char_to_u16('a'), char_to_u16('c')],
                vec![char_to_u16('h'), char_to_u16('r'), char_to_u16('i'), char_to_u16('c')],
                vec![char_to_u16('h'), char_to_u16('t'), char_to_u16(' '), char_to_u16(' ')]
            ];

            let result = digits_from_vec_to_sum(digit_vectors);

            // Erwartete Summen für jeden Block berechnen
            let expected_result = vec![
                help_fun_sum_for_digits(&vec![char_to_u16('D'), char_to_u16('a'), char_to_u16('s'), char_to_u16(' ')]),
                help_fun_sum_for_digits(&vec![char_to_u16('i'), char_to_u16('s'), char_to_u16('t'), char_to_u16(' ')]),
                help_fun_sum_for_digits(&vec![char_to_u16('e'), char_to_u16('i'), char_to_u16('n'), char_to_u16('e')]),
                help_fun_sum_for_digits(&vec![char_to_u16(' '), char_to_u16('T'), char_to_u16('e'), char_to_u16('s')]),
                help_fun_sum_for_digits(&vec![char_to_u16('t'), char_to_u16('n'), char_to_u16('a'), char_to_u16('c')]),
                help_fun_sum_for_digits(&vec![char_to_u16('h'), char_to_u16('r'), char_to_u16('i'), char_to_u16('c')]),
                help_fun_sum_for_digits(&vec![char_to_u16('h'), char_to_u16('t'), char_to_u16(' '), char_to_u16(' ')])
            ];

            assert_eq!(result, expected_result);
        }
    }


    #[test]
    fn test_sum_to_string() {
        let sum = BigUint::from(176096411691_u64);
        let expected = ")*+";
        let result = sum_to_string(&sum);
        assert_eq!(
            result, expected,
            "Die Funktion sum_to_string hat einen unerwarteten Wert zurückgegeben"
        );
    }

    #[test]
    fn test_string_to_sum() {
        let message = ")*+";
        let expected_sum = BigUint::from(176096411691_u64);
        let result_sum = string_to_sum(&message);
        assert_eq!(result_sum, expected_sum);
    }
}
