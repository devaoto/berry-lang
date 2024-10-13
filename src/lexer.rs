#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Equal,
    Comma,
    Bang,
    EqualEqual,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    EqualGreater,
    And,
    Or,
    Semicolon,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    ConstVar, // Immutable variable
    Mutate, // Mutable variable
    Whether, // If
    Otherwise, // Else
    Compare, // Switch-like statement
    Fn, // Function
    Foreach, // For loop
    Forever, // Infinite loop
    Return, // Return statement
    Break, // Break loop
    Continue, // Continue loop
    Change, // Modify function/cluster
    Import, // Module import
    Export, // Module export
    Nullify, // Assign null
    Print,
    Ident(String),
    String(String),
    Boolean(bool),
    LeftParen,
    RightParen,
    Map, // Map function
    Filter, // Filter function
    Reduce, // Reduce function
    Fold, // Fold function
    Zip, // Zip two collections
    Lazy, // Lazy evaluation
    Memoize, // Memoize function
    Lambda, // Lambda expression
    Compose, // Function composition
    Pipe, // Pipe function
    Partial, // Partial application
    LetRec, // Recursive let
    Match, // Pattern matching
    Defer, // Defer execution
    Await, // Await result
    Yield, // Yield control (generators)
    Catch, // Catch errors
    Backtick, // ` Backtick for function composition
    PipeForward, // |>
    ComposeLeft, // <<
    ComposeRight, // >>
    LambdaArrow, // ->
    Bind, // <-
    NullCoalesce, // ??
    Concat, // ++
    Exponent, // **
    TypeDeclaration, // ::
    Assign, // :=
    MaybeAssign, // ?=
    ModAssign, // %=
    DivAssign, // /=
    Xor, // ^
    Dot,
    Hash,

    EOF,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '-' => {
                if let Some(c) = chars.peek() {
                    if *c == '>' {
                        tokens.push(Token::LambdaArrow);
                        chars.next();
                    } else {
                        tokens.push(Token::Minus);
                    }
                } else {
                    tokens.push(Token::Minus);
                }
            }
            '*' => {
                if let Some(c) = chars.peek() {
                    if *c == '*' {
                        tokens.push(Token::Exponent);
                        chars.next();
                    } else {
                        tokens.push(Token::Asterisk);
                    }
                } else {
                    tokens.push(Token::Asterisk);
                }
            }
            '/' => {
                if let Some(c) = chars.peek() {
                    if *c == '=' {
                        tokens.push(Token::DivAssign);
                        chars.next();
                    } else {
                        tokens.push(Token::Slash);
                    }
                } else {
                    tokens.push(Token::Slash);
                }
            }
            '%' => {
                if let Some(c) = chars.peek() {
                    if *c == '=' {
                        tokens.push(Token::ModAssign);
                        chars.next();
                    } else {
                        tokens.push(Token::Percent);
                    }
                } else {
                    tokens.push(Token::Percent);
                }
            }
            '`' => tokens.push(Token::Backtick),

            ',' => tokens.push(Token::Comma),
            ';' => tokens.push(Token::Semicolon),
            '[' => tokens.push(Token::LBracket),
            ']' => tokens.push(Token::RBracket),
            '{' => tokens.push(Token::LBrace),
            '}' => tokens.push(Token::RBrace),
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            '^' => tokens.push(Token::Xor),
            '.' => tokens.push(Token::Dot),
            '#' => tokens.push(Token::Hash),
            '?' => {
                if let Some(c) = chars.peek() {
                    if *c == '?' {
                        tokens.push(Token::NullCoalesce);
                        chars.next();
                    } else if *c == '=' {
                        tokens.push(Token::MaybeAssign);
                        chars.next();
                    }
                } else {
                    tokens.push(Token::MaybeAssign);
                }
            }
            '=' => {
                if let Some(c) = chars.peek() {
                    match *c {
                        '=' => {
                            tokens.push(Token::EqualEqual);
                            chars.next();
                        }
                        '>' => {
                            tokens.push(Token::EqualGreater);
                            chars.next();
                        }
                        _ => tokens.push(Token::Equal),
                    }
                } else {
                    tokens.push(Token::Equal);
                }
            }
            ':' => {
                if let Some(c) = chars.peek() {
                    if *c == '=' {
                        tokens.push(Token::Assign);
                        chars.next();
                    } else {
                        tokens.push(Token::TypeDeclaration);
                        chars.next();
                    }
                }
            }
            '+' => {
                if let Some(c) = chars.peek() {
                    if *c == '+' {
                        tokens.push(Token::Concat);
                        chars.next();
                    } else {
                        tokens.push(Token::Plus);
                    }
                } else {
                    tokens.push(Token::Plus);
                }
            }
            '&' => {
                if let Some(c) = chars.peek() {
                    if *c == '&' {
                        tokens.push(Token::And);
                        chars.next();
                    } else {
                        panic!("Unexpected character {}", c);
                    }
                }
            }
            '!' => {
                if let Some(c) = chars.peek() {
                    if *c == '=' {
                        tokens.push(Token::BangEqual);
                        chars.next();
                    } else {
                        tokens.push(Token::Bang);
                    }
                }
            }
            '>' => {
                if let Some(c) = chars.peek() {
                    if *c == '=' {
                        tokens.push(Token::GreaterEqual);
                        chars.next();
                    } else if *c == '>' {
                        tokens.push(Token::ComposeRight);
                        chars.next();
                    } else {
                        tokens.push(Token::Greater);
                    }
                }
            }
            '<' => {
                if let Some(c) = chars.peek() {
                    if *c == '=' {
                        tokens.push(Token::LessEqual);
                        chars.next();
                    } else if *c == '-' {
                        tokens.push(Token::Bind);
                        chars.next();
                    } else if *c == '<' {
                        tokens.push(Token::ComposeLeft);
                        chars.next();
                    } else {
                        tokens.push(Token::Less);
                    }
                }
            }
            '|' => {
                if let Some(c) = chars.peek() {
                    if *c == '>' {
                        tokens.push(Token::PipeForward);
                        chars.next();
                    } else if *c == '|' {
                        tokens.push(Token::Or);
                        chars.next();
                    } else {
                        panic!("Unexpected character {}", c);
                    }
                }
            }
            '0'..='9' => {
                let mut number = c.to_string();
                while let Some(c) = chars.peek() {
                    if c.is_numeric() {
                        number.push(*c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                match number.parse::<f64>() {
                    Ok(n) => tokens.push(Token::Number(n)),
                    Err(_) => panic!("Invalid number found: {}", number),
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = c.to_string();
                while let Some(c) = chars.peek() {
                    if c.is_alphanumeric() || *c == '_' {
                        ident.push(*c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                match ident.as_str() {
                    "const" => tokens.push(Token::ConstVar),
                    "mutate" => tokens.push(Token::Mutate),
                    "whether" => tokens.push(Token::Whether),
                    "otherwise" => tokens.push(Token::Otherwise),
                    "compare" => tokens.push(Token::Compare),
                    "fn" => tokens.push(Token::Fn),
                    "foreach" => tokens.push(Token::Foreach),
                    "forever" => tokens.push(Token::Forever),
                    "return" => tokens.push(Token::Return),
                    "break" => tokens.push(Token::Break),
                    "continue" => tokens.push(Token::Continue),
                    "change" => tokens.push(Token::Change),
                    "import" => tokens.push(Token::Import),
                    "export" => tokens.push(Token::Export),
                    "nullify" => tokens.push(Token::Nullify),
                    "print" => tokens.push(Token::Print),
                    "true" => tokens.push(Token::Boolean(true)),
                    "false" => tokens.push(Token::Boolean(false)),
                    "map" => tokens.push(Token::Map),
                    "filter" => tokens.push(Token::Filter),
                    "reduce" => tokens.push(Token::Reduce),
                    "fold" => tokens.push(Token::Fold),
                    "zip" => tokens.push(Token::Zip),
                    "lazy" => tokens.push(Token::Lazy),
                    "memoize" => tokens.push(Token::Memoize),
                    "lambda" => tokens.push(Token::Lambda),
                    "compose" => tokens.push(Token::Compose),
                    "pipe" => tokens.push(Token::Pipe),
                    "partial" => tokens.push(Token::Partial),
                    "letRec" => tokens.push(Token::LetRec),
                    "match" => tokens.push(Token::Match),
                    "await" => tokens.push(Token::Await),
                    "yield" => tokens.push(Token::Yield),
                    "defer" => tokens.push(Token::Defer),
                    "catch" => tokens.push(Token::Catch),
                    _ => tokens.push(Token::Ident(ident)),
                }
            }
            '"' => {
                let mut string = String::new();
                while let Some(c) = chars.peek() {
                    if *c == '"' {
                        chars.next();
                        break;
                    } else {
                        string.push(*c);
                        chars.next();
                    }
                }
                tokens.push(Token::String(string));
            }
            ' ' | '\t' | '\n' | '\r' => {
                continue;
            }
            _ => panic!("Unexpected character: {}", c),
        }
    }

    tokens.push(Token::EOF);
    tokens
}
