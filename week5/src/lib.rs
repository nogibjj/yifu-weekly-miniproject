// use std::collections::HashMap;

// fn alpha_ascii_dict() -> HashMap<&'static str, &'static str> {
//   map! {
//       "A" => ".-",      "B" => "-...",    "C" => "-.-.",
//       "D" => "-..",     "E" => ".",       "F" => "..-.",
//       "G" => "--.",     "H" => "....",    "I" => "..",
//       "J" => ".---",    "K" => "-.-",     "L" => ".-..",
//       "M" => "--",      "N" => "-.",      "O" => "---",
//       "P" => ".--.",    "Q" => "--.-",    "R" => ".-.",
//       "S" => "...",     "T" => "-",       "U" => "..-",
//       "V" => "...-",    "W" => ".--",     "X" => "-..-",
//       "Y" => "-.--",    "Z" => "--..",

//       "1" => ".----",   "2" => "..---",   "3" => "...--",
//       "4" => "....-",   "5" => ".....",   "6" => "-....",
//       "7" => "--...",   "8" => "---..",   "9" => "----.",
//       "0" => "-----",

//       "&" => ".-...",   "@" => ".--.-.",  ":" => "---...",
//       "," => "--..--",  "." => ".-.-.-",  "'" => ".----.",
//       "\"" => ".-..-.", "?" => "..--..",  "/" => "-..-.",
//       "=" => "-...-",   "+" => ".-.-.",   "-" => "-....-",
//       "(" => "-.--.",   ")" => "-.--.-",  " " => "/",
//       "!" => "-.-.--",
//   }
// }

// fn morse_ascii_dic() -> HashMap<&'static str, &'static str> {
//   map! {
//       ".-" => "A",      "-..." => "B",    "-.-." => "C",
//       "-.." => "D",     "." => "E",       "..-." => "F",
//       "--." => "G",     "...." => "H",    ".." => "I",
//       ".---" => "J",    "-.-" => "K",     ".-.." => "L",
//       "--" => "M",      "-." => "N",      "---" => "O",
//       ".--." => "P",    "--.-" => "Q",    ".-." => "R",
//       "..." => "S",     "-" => "T",       "..-" => "U",
//       "...-" => "V",    ".--" => "W",     "-..-" => "X",
//       "-.--" => "Y",    "--.." => "Z",

//       ".----" => "1",   "..---" => "2",   "...--" => "3",
//       "....-" => "4",   "....." => "5",   "-...." => "6",
//       "--..." => "7",   "---.." => "8",   "----." => "9",
//       "-----" => "0",

//       ".-..." => "&",   ".--.-." => "@",  "---..." => ":",
//       "--..--" => ",",  ".-.-.-" => ".",  ".----." => "'",
//       ".-..-." => "\"", "..--.." => "?",  "-..-." => "/",
//       "-...-" => "=",   ".-.-." => "+",   "-....-" => "-",
//       "-.--." => "(",   "-.--.-" => ")",   "/" => " ",
//       "-.-.--" => "!",
//   }
// }

// // create a function to convert Ascii to Morse code
// pub fn ascii_morse(input: &str) -> String{
//   let mut output = String::new();
//   let dict = alpha_ascii_dict();
//   for c in input.chars() {
//     let c = c.to_uppercase().to_string();
//     if dict.contains_key(&c) {
//       output.push_str(dict[&c]);
//       output.push(' ');
//     }
//   }
//   output
// }

// // Declaritive macro for creating readable map declarations
// macro_rules! map {
//   ($($key:expr => $value:expr),*) => {{
//       let mut map = ::std::collections::HashMap::new();
//       $(map.insert($key, $value);)*
//       map
//   }}
// }

// // create a function to convert Morse code to Ascii
// pub fn morse_ascii(input: &str) -> String{
//   let mut output = String::new();
//   let dict = morse_ascii_dic();
//   for c in input.split_whitespace() {
//     if dict.contains_key(c) {
//       output.push_str(dict[c]);
//     }
//   }
//   output
// }

// fn check(input: &str) {
//   let morse = ascii_morse(input);
//   let ascii = morse_ascii(&morse);
//   println!("{} -> {} -> {}", input, morse, ascii);
// }

// Morse code lookup table
static _MORSE_CODE: [&str; 36] = [
    ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
    "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--..",
    "-----", ".----", "..---", "...--", "....-", ".....", "-....", "--...", "---..", "----.",
];

// Convert ASCII to Morse code
pub fn _ascii_to_morse(input: &str) -> String {
    let mut output = String::new();

    for c in input.chars() {
        match c {
            'a'..='z' => output.push_str(_MORSE_CODE[(c as u8 - b'a') as usize]),
            'A'..='Z' => output.push_str(_MORSE_CODE[(c as u8 - b'A') as usize]),
            '0'..='9' => output.push_str(_MORSE_CODE[(c as u8 - b'0') as usize + 26]),
            ' ' => output.push(' '),
            _ => {} // Ignore characters that are not supported
        }
        output.push(' '); // Add a space between Morse code characters
    }

    output
}

// Convert Morse code to ASCII
pub fn _morse_to_ascii(input: &str) -> String {
    let morse_code = [
        (".-", 'A'),
        ("-...", 'B'),
        ("-.-.", 'C'),
        ("-..", 'D'),
        (".", 'E'),
        ("..-.", 'F'),
        ("--.", 'G'),
        ("....", 'H'),
        ("..", 'I'),
        (".---", 'J'),
        ("-.-", 'K'),
        (".-..", 'L'),
        ("--", 'M'),
        ("-.", 'N'),
        ("---", 'O'),
        (".--.", 'P'),
        ("--.-", 'Q'),
        (".-.", 'R'),
        ("...", 'S'),
        ("-", 'T'),
        ("..-", 'U'),
        ("...-", 'V'),
        (".--", 'W'),
        ("-..-", 'X'),
        ("-.--", 'Y'),
        ("--..", 'Z'),
        (".----", '1'),
        ("..---", '2'),
        ("...--", '3'),
        ("....-", '4'),
        (".....", '5'),
        ("-....", '6'),
        ("--...", '7'),
        ("---..", '8'),
        ("----.", '9'),
        ("-----", '0'),
    ];

    let mut output = String::new();
    let mut current_char = String::new();

    for c in input.chars() {
        if c == ' ' {
            if let Some((_morse, ascii)) =
                morse_code.iter().find(|(morse, _)| *morse == current_char)
            {
                output.push(*ascii);
            } else if current_char.is_empty() {
                output.push(' ');
            } else {
                output.push('?');
            }
            current_char.clear();
        } else {
            current_char.push(c);
        }
    }

    // Handle the last Morse code character
    if let Some((_morse, ascii)) = morse_code.iter().find(|(morse, _)| *morse == current_char) {
        output.push(*ascii);
    } else if current_char.is_empty() {
        // Do nothing
    } else {
        output.push('?');
    }

    output
}
