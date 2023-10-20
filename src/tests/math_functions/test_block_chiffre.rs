
#[cfg(test)]
mod tests {
    use crate::encryption::math_functions::block_ciffre::{
        split_into_blocks, string_to_int_vec, digits_to_sum, sum_to_string, string_to_sum,
        sum_to_digits, int_vec_to_string
    };
    use ibig::ubig;

    #[test]
    fn test_split_into_blocks() {
        // Testfall 1: Ein einfacher String wird in Blöcke der Größe 2 aufgeteilt.
        let message = String::from("Das ist eine Testnachricht");
        let block_size = 4;
        let result = split_into_blocks(&message, block_size);
        assert_eq!(result, vec!["Das ", "ist ", "eine", " Tes", "tnac", "hric", "ht  "]);

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
            vec![53, 54, 76]
        ];

        for (block, expected_vec) in blocks.iter()
            .zip(expectet_chiffre_results_as_blocks_in_vec.iter()) {
            assert_eq!(string_to_int_vec(block), *expected_vec, "Fehler bei Block: {}", block);
        }
    }
}
