#[cfg(test)]
mod tests {
    use crate::encryption::math_functions::block_chiffre::{create_blocks_from_string, create_string_from_blocks, to_sum_vec, join_string_vec, split_into_blocks, string_to_int_vec, sums_vec_to_string_vec};
    use crate::encryption::math_functions::big_int_util::{c_to_u32};
    use bigdecimal::num_bigint::BigUint;

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
            vec![c_to_u32('D'), c_to_u32('a'), c_to_u32('苉'), c_to_u32(' ')],
            vec![c_to_u32('i'), c_to_u32('s'), c_to_u32('t'), c_to_u32(' ')],
            vec![c_to_u32('e'), c_to_u32('i'), c_to_u32('n'), c_to_u32('e')],
            vec![c_to_u32(' '), c_to_u32('T'), c_to_u32('e'), c_to_u32('s')],
            vec![c_to_u32('t'), c_to_u32('n'), c_to_u32('a'), c_to_u32('c')],
            vec![c_to_u32('h'), c_to_u32('r'), c_to_u32('i'), c_to_u32('c')],
            vec![c_to_u32('h'), c_to_u32('t'), c_to_u32(' '), c_to_u32(' ')],
        ];
        let result = string_to_int_vec(blocks);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_digits_from_vec_to_sum() {
        let digit_vectors = vec![
            vec![c_to_u32('D'), c_to_u32('a'), c_to_u32('苉'), c_to_u32(' ')],
            vec![c_to_u32('i'), c_to_u32('s'), c_to_u32('t'), c_to_u32(' ')],
            vec![c_to_u32('e'), c_to_u32('i'), c_to_u32('n'), c_to_u32('e')],
            vec![c_to_u32(' '), c_to_u32('T'), c_to_u32('e'), c_to_u32('s')],
            vec![c_to_u32('t'), c_to_u32('n'), c_to_u32('a'), c_to_u32('c')],
            vec![c_to_u32('h'), c_to_u32('r'), c_to_u32('i'), c_to_u32('c')],
            vec![c_to_u32('h'), c_to_u32('t'), c_to_u32(' '), c_to_u32(' ')],
        ];

        let base = BigUint::from(55296u32);
        let result = to_sum_vec(digit_vectors, &base);

        let expected_result = vec![
            BigUint::from(11497444858239008u64),
            BigUint::from(17753298306195488u64),
            BigUint::from(17076964999090277u64),
            BigUint::from(5410678690363507u64),
            BigUint::from(19613115525224547u64),
            BigUint::from(17584219565365347u64),
            BigUint::from(17584225676623904u64),
        ];
        assert_eq!(result, expected_result);
    }


    #[test]
    fn test_sum_to_strings() {
        let sums = vec![
            BigUint::from(11497444858239008u64),
            BigUint::from(17753298306195488u64),
            BigUint::from(17076964999090277u64),
            BigUint::from(5410678690363507u64),
            BigUint::from(19613115525224547u64),
            BigUint::from(17584219565365347u64),
            BigUint::from(17584225676623904u64),
        ];

        let base = BigUint::from(55296u32);
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

        let result = join_string_vec(input);

        let expected_result = "Da苉 ist eine Testnachricht  ".to_string();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_create_chiffre() {
        let message = "Da苉 ist eine Testnachricht";
        let block_size = 7;
        let result = create_blocks_from_string(message, block_size, true);
        let expected_result = vec![
                BigUint::from(1943938337267550087026074257524u128),
                BigUint::from(914822981356602019800946507860u128),
                BigUint::from(2887304683313907978613082523752u128),
                BigUint::from(3258925137110102081877384560672u128),
        ];
        assert_eq!(result, expected_result);

        // let message = "Þuď▼Ħìêć6¸¤ÜE+}♠ÎĨ+ËĖZd-ðóZħ☻";
        // let block_size = 4;
        // let result = create_blocks_from_string(message, block_size, true);
        // let expected_result = vec![
        // BigUint::from(11497444858239008u64),
        // BigUint::from(17753298306195488u64),
        // BigUint::from(17076964999090277u64),
        // BigUint::from(5410678690363507u64),
        // BigUint::from(19613115525224547u64),
        // BigUint::from(17584219565365347u64),
        // BigUint::from(17584225676623904u64),
        // ];
        // assert_eq!(result, expected_result);
    }

    #[test]
    fn test_decode_chiffre() {
        let sums = vec![
            BigUint::from(1943938337267550087026074257524u128),
            BigUint::from(914822981356602019800946507860u128),
            BigUint::from(2887304683313907978613082523752u128),
            BigUint::from(3258925137110102081877384560672u128),
        ];
        let result = create_string_from_blocks(sums);
        let expected_result = "Da苉 ist eine Testnachricht  ".to_string();
        assert_eq!(result, expected_result);
    }


}

