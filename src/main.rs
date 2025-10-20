mod lexer;

fn main() {
    if std::env::args().count() != 3 {
        panic!("Incorrect number of arguments!");
    }

    let lex_path = std::env::args().nth(1).expect("Could not get lex file path!");
    let file_path = std::env::args().nth(2).expect("Could not get code file path!");

    let lex_data = std::fs::read_to_string(lex_path).expect("Could not read lex file!");
    let file_text = std::fs::read_to_string(file_path).expect("Could not read code file!");
}