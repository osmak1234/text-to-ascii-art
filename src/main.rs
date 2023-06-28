pub mod ascii;
use ascii::CHARS;

// You need to specialy concat ascii art because of how line brakes work, otherwise you would have just
// everything vertically
pub fn join_art(s1: &str, s2: &str) -> String {
    // handles edge cases, with simple match
    match (s1.is_empty(), s2.is_empty()) {
        (true, true) => "".to_string(),
        (true, false) => s2.to_string(),
        (false, true) => s1.to_string(),
        (false, false) => {
            // case when both string are something
            // each line has to have the same amount of characters, or other art will be shifted
            // TODO: handle each line with different ammount of characters

            // you get all lines of the &str
            let lines1: Vec<&str> = s1.split('\n').collect();
            let lines2: Vec<&str> = s2.split('\n').collect();

            // concats each line of the 2 ascii art
            let s3: Vec<String> = lines1
                .into_iter()
                .zip(lines2.into_iter())
                .map(|(str1, str2)| str1.to_owned() + " " + str2)
                .collect();

            s3.join("\n")
        }
    }
}

pub fn convert(input: String) -> Result<String, String> {
    // substitutes everything with the equivalent in ascii art, or an empty string instead
    let art_vector = input
        .chars()
        .map(|ch| CHARS.get(ch as usize).unwrap_or(&"").to_owned())
        .collect::<Vec<&str>>();

    // function to go through all the entered characters
    let mut final_string = "".to_string();
    let mut bad_char = false;
    for art in &art_vector {
        if art.is_empty() && !bad_char {
            bad_char = true
        }
    }
    // happens if you pass unhadled characters
    if bad_char {
        Err("Error: Some unallowed characters, you can use: a..=Z, 0..=9 ,`; : . , < > ( ) ! * # @ ^`".to_string())
    } else {
        for art in art_vector {
            final_string = join_art(&final_string, art);
        }
        Ok(final_string)
    }
}
fn main() {
    match convert("ab  cdaaa".to_string()) {
        Ok(string) => println!("{}", string),
        Err(err) => println!("Error: {}", err),
    }
}
