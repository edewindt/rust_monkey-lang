use crate::token::{lookup_identifier, Token, TokenType};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}
impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: Default::default(),
        };
        lexer.read_char();
        lexer
    }
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token {
                        ttype: TokenType::Eq,
                        literal: String::from("=="),
                    }
                } else {
                    Lexer::new_token(TokenType::Assign, self.ch)
                }
            }
            ';' => Lexer::new_token(TokenType::Semicolon, self.ch),
            '+' => Lexer::new_token(TokenType::Plus, self.ch),
            '(' => Lexer::new_token(TokenType::LParen, self.ch),
            ')' => Lexer::new_token(TokenType::RParen, self.ch),
            ',' => Lexer::new_token(TokenType::Comma, self.ch),
            '{' => Lexer::new_token(TokenType::LBrace, self.ch),
            '}' => Lexer::new_token(TokenType::RBrace, self.ch),
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token {
                        ttype: TokenType::NotEq,
                        literal: String::from("!="),
                    }
                } else {
                    Lexer::new_token(TokenType::Bang, self.ch)
                }
            }
            '-' => Lexer::new_token(TokenType::Minus, self.ch),
            '/' => Lexer::new_token(TokenType::Slash, self.ch),
            '<' => Lexer::new_token(TokenType::Lt, self.ch),
            '>' => Lexer::new_token(TokenType::Gt, self.ch),
            '*' => Lexer::new_token(TokenType::Asterisk, self.ch),
            '\0' => Token {
                ttype: TokenType::Eof,
                literal: "".to_string(),
            },
            _ => {
                return if Lexer::is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let ttype = lookup_identifier(&literal);
                    Token { ttype, literal }
                } else if Lexer::is_digit(self.ch) {
                    let literal = self.read_number();
                    let ttype = TokenType::Intiger;
                    Token { ttype, literal }
                } else {
                    Lexer::new_token(TokenType::Illegal, self.ch)
                }
            }
        };
        self.read_char();
        return token;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
    fn new_token(ttype: TokenType, ch: char) -> Token {
        Token {
            ttype,
            literal: ch.to_string(),
        }
    }

    fn is_letter(ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }
    fn is_digit(ch: char) -> bool {
        ch.is_numeric()
    }
    fn read_number(&mut self) -> String {
        let mut num = String::from("");

        while Lexer::is_digit(self.ch) {
            num.push(self.ch);
            self.read_char();
        }
        num
    }
    fn peek_char(&self) -> char {
        return if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        };
    }
    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while Lexer::is_letter(self.ch) {
            identifier.push(self.ch);
            self.read_char();
        }
        identifier
    }
}

#[cfg(test)]
mod test {
    use crate::token::{Token, TokenType};

    use super::Lexer;

    #[test]
    fn test_next_token() {
        let input = r#"
        let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;
        "#;

        let expected: Vec<Token> = vec![
            Token {
                ttype: TokenType::Let,
                literal: "let".to_string(),
            },
            Token {
                ttype: TokenType::Identifier,
                literal: "five".to_string(),
            },
            Token {
                ttype: TokenType::Assign,
                literal: "=".to_string(),
            },
            Token {
                ttype: TokenType::Intiger,
                literal: "5".to_string(),
            },
            Token {
                ttype: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                ttype: TokenType::Let,
                literal: "let".to_string(),
            },
            Token {
                ttype: TokenType::Identifier,
                literal: "ten".to_string(),
            },
            Token {
                ttype: TokenType::Assign,
                literal: "=".to_string(),
            },
            Token {
                ttype: TokenType::Intiger,
                literal: "10".to_string(),
            },
            Token {
                ttype: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                ttype: TokenType::Let,
                literal: "let".to_string(),
            },
            Token {
                ttype: TokenType::Identifier,
                literal: "add".to_string(),
            },
            Token {
                ttype: TokenType::Assign,
                literal: "=".to_string(),
            },
            Token {
                ttype: TokenType::Function,
                literal: "fn".to_string(),
            },
            Token {
                ttype: TokenType::LParen,
                literal: "(".to_string(),
            },
            Token {
                ttype: TokenType::Identifier,
                literal: "x".to_string(),
            },
            Token {
                ttype: TokenType::Comma,
                literal: ",".to_string(),
            },
            Token {
                ttype: TokenType::Identifier,
                literal: "y".to_string(),
            },
            Token {
                ttype: TokenType::RParen,
                literal: ")".to_string(),
            },
            Token {
                ttype: TokenType::LBrace,
                literal: "{".to_string(),
            },
            Token {
                ttype: TokenType::Identifier,
                literal: "x".to_string(),
            },
            Token {
                ttype: TokenType::Plus,
                literal: "+".to_string(),
            },
            Token {
                ttype: TokenType::Identifier,
                literal: "y".to_string(),
            },
            Token {
                ttype: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                ttype: TokenType::RBrace,
                literal: "}".to_string(),
            },
            Token {
                ttype: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                ttype: TokenType::Let,
                literal: "let".to_string(),
            },
            Token {
                ttype: TokenType::Identifier,
                literal: "result".to_string(),
            },
            Token {
                ttype: TokenType::Assign,
                literal: "=".to_string(),
            },
            Token {
                ttype: TokenType::Identifier,
                literal: "add".to_string(),
            },
            Token {
                ttype: TokenType::LParen,
                literal: "(".to_string(),
            },
            Token {
                ttype: TokenType::Identifier,
                literal: "five".to_string(),
            },
            Token {
                ttype: TokenType::Comma,
                literal: ",".to_string(),
            },
            Token {
                ttype: TokenType::Identifier,
                literal: "ten".to_string(),
            },
            Token {
                ttype: TokenType::RParen,
                literal: ")".to_string(),
            },
            Token {
                ttype: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                ttype: TokenType::Bang,
                literal: "!".to_string(),
            },
            Token {
                ttype: TokenType::Minus,
                literal: "-".to_string(),
            },
            Token {
                ttype: TokenType::Slash,
                literal: "/".to_string(),
            },
            Token {
                ttype: TokenType::Asterisk,
                literal: "*".to_string(),
            },
            Token {
                ttype: TokenType::Intiger,
                literal: "5".to_string(),
            },
            Token {
                ttype: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                ttype: TokenType::Intiger,
                literal: "5".to_string(),
            },
            Token {
                ttype: TokenType::Lt,
                literal: "<".to_string(),
            },
            Token {
                ttype: TokenType::Intiger,
                literal: "10".to_string(),
            },
            Token {
                ttype: TokenType::Gt,
                literal: ">".to_string(),
            },
            Token {
                ttype: TokenType::Intiger,
                literal: "5".to_string(),
            },
            Token {
                ttype: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                ttype: TokenType::If,
                literal: "if".to_string(),
            },
            Token {
                ttype: TokenType::LParen,
                literal: "(".to_string(),
            },
            Token {
                ttype: TokenType::Intiger,
                literal: "5".to_string(),
            },
            Token {
                ttype: TokenType::Lt,
                literal: "<".to_string(),
            },
            Token {
                ttype: TokenType::Intiger,
                literal: "10".to_string(),
            },
            Token {
                ttype: TokenType::RParen,
                literal: ")".to_string(),
            },
            Token {
                ttype: TokenType::LBrace,
                literal: "{".to_string(),
            },
            Token {
                ttype: TokenType::Return,
                literal: "return".to_string(),
            },
            Token {
                ttype: TokenType::True,
                literal: "true".to_string(),
            },
            Token {
                ttype: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                ttype: TokenType::RBrace,
                literal: "}".to_string(),
            },
            Token {
                ttype: TokenType::Else,
                literal: "else".to_string(),
            },
            Token {
                ttype: TokenType::LBrace,
                literal: "{".to_string(),
            },
            Token {
                ttype: TokenType::Return,
                literal: "return".to_string(),
            },
            Token {
                ttype: TokenType::False,
                literal: "false".to_string(),
            },
            Token {
                ttype: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                ttype: TokenType::RBrace,
                literal: "}".to_string(),
            },
            Token {
                ttype: TokenType::Intiger,
                literal: "10".to_string(),
            },
            Token {
                ttype: TokenType::Eq,
                literal: "==".to_string(),
            },
            Token {
                ttype: TokenType::Intiger,
                literal: "10".to_string(),
            },
            Token {
                ttype: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                ttype: TokenType::Intiger,
                literal: "10".to_string(),
            },
            Token {
                ttype: TokenType::NotEq,
                literal: "!=".to_string(),
            },
            Token {
                ttype: TokenType::Intiger,
                literal: "9".to_string(),
            },
            Token {
                ttype: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                ttype: TokenType::Eof,
                literal: "".to_string(),
            },
        ];

        let mut lexer = Lexer::new(input);

        for (idx, exp_token) in expected.into_iter().enumerate() {
            let recv_token = lexer.next_token();
            assert_eq!(
                exp_token.ttype, recv_token.ttype,
                "tests [(idx)] - token type wrong. expected={}, got={}",
                exp_token.ttype, recv_token.ttype
            );
            assert_eq!(
                exp_token.literal, recv_token.literal,
                "tests [(idx)] - literal wrong. expected={}, got={}",
                exp_token.literal, recv_token.literal
            );
        }
    }
}
