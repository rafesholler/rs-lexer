use std::error::Error;
use regex::Regex;

mod token;
use token::Token;

pub fn tokenize(data: String, file: String) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut token_patterns = std::collections::HashMap::new();
    
    let mut split_data = data.split_whitespace();

    while let Some(regex) = split_data.next() {
        if let Some(token_name) = split_data.next() {
            let mut new_regex = String::from(regex);
            new_regex.push_str("$");
            new_regex.insert_str(0, "^");
            token_patterns.insert(String::from(new_regex), token_name.to_uppercase());
        } 
    }

    let mut tokens = vec![];
    let split_file = file.split_whitespace();

    for word in split_file {
        let mut found = false;
        for t_pattern in &token_patterns {
            let re = Regex::new(&t_pattern.0)?;
            if re.is_match(word) {
                tokens.push(Token::new(t_pattern.1.to_string(), word.to_owned()));
                found = true;
                break;
            }
        }
        if !found {
            tokens.push(Token::new("RAW".to_owned(), word.to_owned()));
        }
    }

    Ok(tokens)
}