enum KeywordType {
    Let,
    Print,
}

enum LiteralType {
    Int,
    String,
}

enum TokenType {
    Declaration,
    Ident,
    Keyword(KeywordType),
    Literal(LiteralType),
}

struct Token {
    type_type: TokenType,
    value: String,
}

impl Token {
    fn new(type_type: TokenType, value: String) -> Token {
        Token { type_type, value }
    }
}

struct Tokenizer {
    text: String,
    tokens: Vec<Token>,
}

impl Tokenizer {
    fn new(text: String) -> Tokenizer {
        Tokenizer {
            text,
            tokens: Vec::new(),
        }
    }
}
