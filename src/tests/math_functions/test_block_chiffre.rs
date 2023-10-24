#[cfg(test)]
mod tests {
    use crate::encryption::math_functions::block_chiffre::{create_blocks_from_string, create_string_from_blocks, to_sum_vec, decode_s_vec, create_b_vec, s_to_i_vec, sums_vec_to_s_vec};
    use crate::encryption::math_functions::big_int_util::{c_to_u16};
    use bigdecimal::num_bigint::BigUint;

    #[test]
    fn test_split_into_blocks() {
        // Testfall 1: Ein einfacher String wird in Blöcke der Größe 4 aufgeteilt.
        let message = String::from("Das ist eine Testnachricht");
        let block_size = 4;
        let result = create_b_vec(&message, block_size);
        assert_eq!(
            result,
            vec!["Das ", "ist ", "eine", " Tes", "tnac", "hric", "ht  "]
        );

        // Testfall 2: Ein String, der bereits eine Blockgröße hat, wird nicht verändert,
        // es kommt kein neuer leerer Block dazu.
        let message = String::from("123AB");
        let block_size = 5;
        let result = create_b_vec(&message, block_size);
        assert_eq!(result, vec!["123AB"]);

        // Testfall 3: Ein leerer String wird in Blöcke der Größe 3 aufgeteilt.
        let message = String::from("   ");
        let block_size = 3;
        let result = create_b_vec(&message, block_size);
        assert_eq!(result, vec!["   "]);

        // Testfall 4: Ein String wird in Blöcke der Größe 1 aufgeteilt.
        let message = String::from("abcdef");
        let block_size = 1;
        let result = create_b_vec(&message, block_size);
        assert_eq!(result, vec!["a", "b", "c", "d", "e", "f"]);
    }

    #[test]
    fn test_string_to_int_vec() {
        let message = "Das ist eine Testnachricht ";
        let blocks = create_b_vec(&message, 4);
        let expected = vec![
            vec![c_to_u16('D'), c_to_u16('a'), c_to_u16('s'), c_to_u16(' ')],
            vec![c_to_u16('i'), c_to_u16('s'), c_to_u16('t'), c_to_u16(' ')],
            vec![c_to_u16('e'), c_to_u16('i'), c_to_u16('n'), c_to_u16('e')],
            vec![c_to_u16(' '), c_to_u16('T'), c_to_u16('e'), c_to_u16('s')],
            vec![c_to_u16('t'), c_to_u16('n'), c_to_u16('a'), c_to_u16('c')],
            vec![c_to_u16('h'), c_to_u16('r'), c_to_u16('i'), c_to_u16('c')],
            vec![c_to_u16('h'), c_to_u16('t'), c_to_u16(' '), c_to_u16(' ')],
        ];
        let result = s_to_i_vec(blocks);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_digits_from_vec_to_sum() {
        let digit_vectors = vec![
            vec![c_to_u16('D'), c_to_u16('a'), c_to_u16('s'), c_to_u16(' ')],
            vec![c_to_u16('i'), c_to_u16('s'), c_to_u16('t'), c_to_u16(' ')],
            vec![c_to_u16('e'), c_to_u16('i'), c_to_u16('n'), c_to_u16('e')],
            vec![c_to_u16(' '), c_to_u16('T'), c_to_u16('e'), c_to_u16('s')],
            vec![c_to_u16('t'), c_to_u16('n'), c_to_u16('a'), c_to_u16('c')],
            vec![c_to_u16('h'), c_to_u16('r'), c_to_u16('i'), c_to_u16('c')],
            vec![c_to_u16('h'), c_to_u16('t'), c_to_u16(' '), c_to_u16(' ')],
        ];

        let result = to_sum_vec(digit_vectors);

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

        let result = sums_vec_to_s_vec(sums);

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

        let result = decode_s_vec(input);

        let expected_result = "Das ist eine Testnachricht  ".to_string();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_create_chiffre() {
        let message = "Das ist eine Testnachricht";
        let block_size = 4;
        let result = create_blocks_from_string(message, block_size);
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
        let result = create_string_from_blocks(sums);
        let expected_result = "Das ist eine Testnachricht  ".to_string();
        assert_eq!(result, expected_result);
    }


}

