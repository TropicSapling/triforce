#[derive(Debug)]
pub enum Token {
    Default(String),
    UserDef(String),
    BegOpenList,
    BegList,
    EndList,
    Newline
}

pub fn tokenised(code: String) -> Vec<Token> {
    let tokens = vec![];

    for c in code.chars() {
        // TODO ...
    }

    tokens
}
