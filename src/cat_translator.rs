const BIN_TO_CHAR: [&str; 128] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "=",
    "[", "]", ";", "'", "#", "|", ",", ".", "/", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ",
    " ", " ", " ", " ", " ", " ", " ", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L",
    "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "!", "€", "£", "$", "%",
    "^", "&", "*", "(", ")", "_", "+", "{", "}", ":", "@", "~", "|", "<", ">", "?", ")", "\"", " ",
    " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", " ", "", "",
];

const BIN_TO_CAT: [&str; 16] = [
    "meow", "meoww", "meowww", "meowwww", "mrow", "mroww", "mrowww", "mrowwww", "mrp", "mrrp",
    "mrrrp", "mrrrrp", "purr", "purrr", "purrrr", "purrrrr",
];

fn cat_noise_to_bin(cat_noise: &str) -> usize {
    if cat_noise.get(0..1).unwrap() == "p" {
        // length min bound to 4
        let len = usize::max(cat_noise.len(), 4);
        // length max bound to 7
        let len = usize::min(len, 7);
        return 12 | (len - 4);
        // if meow
    } else if cat_noise.get(0..1).unwrap() == "m" && cat_noise.contains("e") {
        // length min bound to 4
        let len = usize::max(cat_noise.len(), 4);
        // length max bound to 7
        let len = usize::min(len, 7);
        return len - 4;
        // if mrrp
    } else if cat_noise.get(0..1).unwrap() == "m" && !cat_noise.contains("o") {
        // length min bound to 4
        let len = usize::max(cat_noise.len(), 3);
        // length max bound to 7
        let len = usize::min(len, 6);
        return 8 | (len - 3);
        // if mrow
    } else {
        // length min bound to 4
        let len = usize::max(cat_noise.len(), 4);
        // length max bound to 7
        let len = usize::min(len, 7);
        return 4 | (len - 4);
    }
}

// translate a {length} bit number to its String binary representation with a length of {length} bits
fn number_to_bin(number: u8, length: usize) -> String {
    let result = format!("{number:b}");
    "0".repeat(length - result.len()) + &result
}

// translate a text to its bit representation, each character is 7 bits and their bit
// representation is their index in the BIN_TO_CHAR array
fn text_to_bin(text: &str) -> Result<String, String> {
    let unvalid_characters = text
        .chars()
        .map(|x| (x, BIN_TO_CHAR.iter().find(|y| **y == &x.to_string())))
        .filter(|x| x.1.is_none())
        .map(|x| x.0)
        .collect::<Vec<char>>();
    if unvalid_characters.len() != 0 {
        return Err(format!(
            "{} is not a valid character\nhere is a list of all valid characters: \nabcdefghijklmnopqrstuvwxyz1234567890-=[];'#|,./ ABCDEFGHIJKLMNOPQRSTUVWXYZ!€£$%^&*()_+{}:@~|<>?)\"",
            unvalid_characters[0], "{}"
        ));
    }
    Ok(text
        .chars()
        .map(|x| {
            BIN_TO_CHAR
                .iter()
                .enumerate()
                .find(|y| y.1 == &x.to_string())
                .map(|x| x.0)
                .unwrap()
        })
        .map(|x| number_to_bin(x as u8, 7))
        .collect::<Vec<String>>()
        .join(""))
}

// translate cat noises to their bit representations, each noise is 4 bits and their bit
// representation is their index in the BIN_TO_CAT array
fn cat_to_bin(text: &str) -> String {
    text.split(" ")
        .filter(|x| x != &":3" && x != &":3c")
        .map(|x| cat_noise_to_bin(x))
        .map(|x| number_to_bin(x as u8, 4))
        .collect::<Vec<String>>()
        .join("")
        + &text
            .split(" ")
            .filter(|x| x == &":3" || x == &":3c")
            .map(|x| if x == ":3" { 0 } else { 1 })
            .map(|x| number_to_bin(x, 1))
            .collect::<Vec<String>>()
            .join("")
}

fn bin_to_cat(bin: &str) -> String {
    let mut result = vec![];
    for i in (0..bin.len()).filter(|x| x % 4 == 0 || bin.len() - x <= bin.len() % 4) {
        if bin.len() - i <= bin.len() % 4 {
            result.push(if bin.get(i..=i).unwrap() == "0" {
                ":3"
            } else {
                ":3c"
            })
        } else {
            let cat_bin = bin.get(i..i + 4).unwrap();
            let cat_noise = BIN_TO_CAT[usize::from_str_radix(cat_bin, 2).expect("")];
            result.push(cat_noise);
        }
    }
    result.join(" ")
}

fn bin_to_text(bin: &str) -> String {
    let mut result = vec![];
    for i in (0..bin.len())
        .filter(|x| x % 7 == 0)
        .filter(|x| bin.len() >= x + 7)
    {
        let letter_bin = bin.get(i..i + 7).unwrap();
        let letter = BIN_TO_CHAR[usize::from_str_radix(letter_bin, 2).expect("")];
        result.push(letter);
    }
    result.join("")
}

pub fn text_to_cat(text: &str) -> Result<String, String> {
    match text_to_bin(text) {
        Ok(bin) => Ok(bin_to_cat(&bin)),
        Err(e) => Err(e),
    }
}

pub fn cat_noises_to_text(cat_noises: &str) -> String {
    bin_to_text(&cat_to_bin(cat_noises))
}
