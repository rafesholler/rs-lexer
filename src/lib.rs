use std::error::Error;
use regex::Regex;

mod token;
use token::Token;

pub fn tokenize(data: &str, file: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut token_patterns = std::collections::HashMap::new();
    
    let mut split_data = data.split_whitespace();

    while let Some(regex) = split_data.next() {
        if let Some(token_name) = split_data.next() {
            let mut new_regex = String::from(regex);
            new_regex.insert_str(0, "^");
            token_patterns.insert(String::from(new_regex), token_name.to_uppercase());
        } 
    }

    let mut tokens = vec![];

    let mut buffer = &file[0..file.len()];

    while buffer.len() > 0 {
        let mut found = false;
        for t_pattern in &token_patterns {
            
            
            let re = Regex::new(&t_pattern.0)?;
            if let Some(word) = re.find(buffer) {
                tokens.push(Token::new(t_pattern.1.to_string(), word.as_str().to_string()));
                found = true;
                buffer = &buffer[(word.as_str().len())..buffer.len()];
                break;
            }
        }
        if !found {
            if !buffer[0..1].contains(char::is_whitespace) {
                tokens.push(Token::new("raw".to_string(), buffer[0..1].to_string()));
            }
            buffer = &buffer[1..buffer.len()];
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(tokenize("", "").unwrap(), vec![]);
    }

    #[test]
    fn test1() {
        let lex = std::fs::read_to_string("src/test/test_lex1.txt").unwrap();
        let file = std::fs::read_to_string("src/test/test_file1.txt").unwrap();

        let output = tokenize(&lex, &file).unwrap();

        let expected = vec![
            Token::new("LITERAL".to_string(), "This".to_string()),
            Token::new("LITERAL".to_string(), "tests".to_string()),
            Token::new("LITERAL".to_string(), "words".to_string()),
            Token::new("LITERAL".to_string(), "and".to_string()),
            Token::new("LITERAL".to_string(), "integers".to_string()),
            Token::new("LITERAL".to_string(), "like".to_string()),
            Token::new("NUM".to_string(), "9000".to_string()),
            Token::new("NUM".to_string(), "-7474".to_string()),
            Token::new("LITERAL".to_string(), "and".to_string()),
            Token::new("NUM".to_string(), "22222".to_string()),
        ];

        assert_eq!(output, expected);
    }

    #[test]
    fn test2() {
        let lex = std::fs::read_to_string("src/test/test_lex2.txt").unwrap();
        let file = std::fs::read_to_string("src/test/test_file2.txt").unwrap();

        let output = tokenize(&lex, &file).unwrap();

        let expected = vec![
            Token::new("LIT".to_string(), "Testing".to_string()),
            Token::new("LINE_BREAK".to_string(), "\n".to_string()),
            Token::new("LIT".to_string(), "more".to_string()),
            Token::new("LEFT_PAREN".to_string(), "(".to_string()),
            Token::new("LIT".to_string(), "like".to_string()),
            Token::new("LIT".to_string(), "parenthases".to_string()),
            Token::new("RIGHT_PAREN".to_string(), ")".to_string()),
            Token::new("LIT".to_string(), "and".to_string()),
            Token::new("LINE_BREAK".to_string(), "\n".to_string()),
            Token::new("LINE_BREAK".to_string(), "\n".to_string()),
            Token::new("LIT".to_string(), "whitespace".to_string()),
            Token::new("LIT".to_string(), "patterns".to_string()),
            Token::new("LIT".to_string(), "like".to_string()),
            Token::new("LIT".to_string(), "these".to_string()),
        ];

        assert_eq!(output, expected);
    }
}