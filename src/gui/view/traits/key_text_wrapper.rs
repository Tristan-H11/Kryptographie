pub trait TextWrapper {
    fn key_text_wrapper(&self, text: &str, chars_per_line: usize) -> String;
}

pub struct KeyTextWrapper;

impl TextWrapper for KeyTextWrapper {
    fn key_text_wrapper(&self, text: &str, chars_per_line: usize) -> String {
        text.chars()
            .enumerate()
            .fold(String::new(), |mut acc, (i, c)| {
                if i > 0 && i % chars_per_line == 0 {
                    acc.push('\n');
                }
                acc.push(c);
                acc
            })
    }
}
