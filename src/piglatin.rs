pub fn make_piglatin(s: &str) -> Result<String, String> {
    let first_char = match s.chars().next() {
        Some(c) => c.clone(),
        None => return Err(String::from("String cannot be empty"))
    };   
    let new_string = match first_char {
        c if "aeiou".contains(c) => {
            format!("{s}hay")
        },
        _ => {
            let remaining_chars = String::from(&s[1..]);
            format!("{remaining_chars}{first_char}ay")
        }
    };
    Ok(new_string)
}