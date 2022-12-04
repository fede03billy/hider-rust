// The text_to_chars() function takes a string as an argument and returns an array of the Unicode character codes of each character in the string.
fn text_to_chars(text: &str) -> Vec<u32> {
    text.chars().map(|c| c as u32).collect()
}

// The apply_salt_to_char() function takes a character code as an argument and returns the result of applying the bitwise XOR operator to the character code and the salt.
fn apply_salt_to_char(code: u32, salt: &str) -> u32 {
    let salt_chars = text_to_chars(salt);
    salt_chars.iter().fold(code, |acc, &b| acc ^ b)
}

// The hide() function takes a salt and a text as arguments and returns the encoded text.
fn hide(salt: &str, text: &str) -> String {
    // The byte_hex() function takes a number as an argument and returns the hexadecimal representation of that number as a string.
    fn byte_hex(n: u32) -> String {
        format!("{:02x}", n)
    }

    let mut encoded = "";

    for c in text.chars() {
        let code = c as u32;
        let code = apply_salt_to_char(code, salt);
        encoded += &byte_hex(code);
    }

    encoded
}

// The unhide() function takes a salt and an encoded text as arguments and returns the decoded text.
fn unhide(salt: &str, encoded: &str) -> String {
    let mut decoded = "";

    for hex in encoded.as_bytes().chunks(2) {
        let code = u32::from_str_radix(std::str::from_utf8(hex).unwrap(), 16).unwrap();
        let code = apply_salt_to_char(code, salt);
        decoded += &std::char::from_u32(code).unwrap().to_string();
    }

    decoded
}

// encrypting
let encrypted_text = hide("salt", "Hello"); // -> 426f666665
println!("{}", encrypted_text);

// decrypting
let decrypted_string = unhide("salt", "426f666665"); // -> Hello
println!("{}", decrypted_string);

