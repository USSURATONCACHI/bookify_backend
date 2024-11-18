pub fn get_leading_digits(text: &str) -> &str {
    let mut digits_end = 0;
    for (i, c) in text.char_indices() {
        if !c.is_digit(10) {
            digits_end = i;
            break;
        }
    }

    &text[..digits_end]
}
