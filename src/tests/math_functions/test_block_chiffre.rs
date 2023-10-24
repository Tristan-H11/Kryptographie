#[cfg(test)]
mod tests {
    use crate::encryption::math_functions::block_chiffre::{create_chiffre, decode_chiffre, digits_from_vec_to_sum, join_strings, split_into_blocks, string_to_int_vec, sums_to_strings};
    use crate::encryption::math_functions::big_int_util::{char_to_u16};
    use bigdecimal::num_bigint::BigUint;

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
            vec![char_to_u16('h'), char_to_u16('t'), char_to_u16(' '), char_to_u16(' ')],
        ];
        let result = string_to_int_vec(blocks);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_digits_from_vec_to_sum() {
        let digit_vectors = vec![
            vec![char_to_u16('D'), char_to_u16('a'), char_to_u16('s'), char_to_u16(' ')],
            vec![char_to_u16('i'), char_to_u16('s'), char_to_u16('t'), char_to_u16(' ')],
            vec![char_to_u16('e'), char_to_u16('i'), char_to_u16('n'), char_to_u16('e')],
            vec![char_to_u16(' '), char_to_u16('T'), char_to_u16('e'), char_to_u16('s')],
            vec![char_to_u16('t'), char_to_u16('n'), char_to_u16('a'), char_to_u16('c')],
            vec![char_to_u16('h'), char_to_u16('r'), char_to_u16('i'), char_to_u16('c')],
            vec![char_to_u16('h'), char_to_u16('t'), char_to_u16(' '), char_to_u16(' ')],
        ];

        let result = digits_from_vec_to_sum(digit_vectors);

        let expected_result = vec![
            BigUint::from(19140715035688992u64),
            BigUint::from(29555366483460128u64),
            BigUint::from(28429423626551397u64),
            BigUint::from(9007560038613107u64),
            BigUint::from(32651569751195747u64),
            BigUint::from(29273887211061347u64),
            BigUint::from(29273895796211744u64),
        ];
        assert_eq!(result, expected_result);
    }


    #[test]
    fn test_sum_to_strings() {
        let sums = vec![
            BigUint::from(19140715035688992u64),
            BigUint::from(29555366483460128u64),
            BigUint::from(28429423626551397u64),
            BigUint::from(9007560038613107u64),
            BigUint::from(32651569751195747u64),
            BigUint::from(29273887211061347u64),
            BigUint::from(29273895796211744u64),
        ];

        let result = sums_to_strings(sums);

        let expected_result = vec![
            "Das ".to_string(),
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
            "Das ".to_string(),
            "ist ".to_string(),
            "eine".to_string(),
            " Tes".to_string(),
            "tnac".to_string(),
            "hric".to_string(),
            "ht  ".to_string(),
        ];

        let result = join_strings(input);

        let expected_result = "Das ist eine Testnachricht  ".to_string();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_create_chiffre() {
        let message = "Das ist eine Testnachricht";
        let block_size = 4;
        let result = create_chiffre(message, block_size);
        let expected_result = vec![
            BigUint::from(19140715035688992u64),
            BigUint::from(29555366483460128u64),
            BigUint::from(28429423626551397u64),
            BigUint::from(9007560038613107u64),
            BigUint::from(32651569751195747u64),
            BigUint::from(29273887211061347u64),
            BigUint::from(29273895796211744u64),
        ];
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_decode_chiffre() {
        let sums = vec![
            BigUint::from(19140715035688992u64),
            BigUint::from(29555366483460128u64),
            BigUint::from(28429423626551397u64),
            BigUint::from(9007560038613107u64),
            BigUint::from(32651569751195747u64),
            BigUint::from(29273887211061347u64),
            BigUint::from(29273895796211744u64),
        ];
        let result = decode_chiffre(sums);
        let expected_result = "Das ist eine Testnachricht  ".to_string();
        assert_eq!(result, expected_result);
    }


}

