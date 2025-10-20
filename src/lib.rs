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

    let mut i = 0;

    while i < file.len() {
        let mut found = false;
        for t_pattern in &token_patterns {
            if (i + t_pattern.0.len() - 2) >= file.len() {
                continue;
            }

            let word = &file[i..(i + t_pattern.0.len() - 2)];            
            let re = Regex::new(&t_pattern.0)?;
            if re.is_match(word) {
                tokens.push(Token::new(t_pattern.1.to_string(), word.to_owned()));
                found = true;
                i += word.len();
                break;
            }
        }
        if !found {
            i += 1;
            tokens.push(Token::new("RAW".to_string(), file[i..i+1].to_string()))
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(tokenize(String::new(), String::new()).unwrap(), vec![]);
    }
}