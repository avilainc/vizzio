//! STEP-File tokenizer (Rust puro)

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// #123
    EntityRef(u32),
    /// IFCWALL
    Keyword(String),
    /// "text"
    String(String),
    /// 123
    Integer(i64),
    /// 123.456
    Float(f64),
    /// (
    LeftParen,
    /// )
    RightParen,
    /// ,
    Comma,
    /// ;
    Semicolon,
    /// =
    Equal,
    /// $
    Null,
    /// .T. ou .F.
    Boolean(bool),
}

/// Tokenizador STEP
pub struct StepTokenizer {
    input: Vec<char>,
    position: usize,
}

impl StepTokenizer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while self.position < self.input.len() {
            self.skip_whitespace();

            if self.position >= self.input.len() {
                break;
            }

            let token = self.next_token()?;
            tokens.push(token);
        }

        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Token, String> {
        let ch = self.current_char();

        match ch {
            '(' => {
                self.advance();
                Ok(Token::LeftParen)
            }
            ')' => {
                self.advance();
                Ok(Token::RightParen)
            }
            ',' => {
                self.advance();
                Ok(Token::Comma)
            }
            ';' => {
                self.advance();
                Ok(Token::Semicolon)
            }
            '=' => {
                self.advance();
                Ok(Token::Equal)
            }
            '$' => {
                self.advance();
                Ok(Token::Null)
            }
            '#' => self.parse_entity_ref(),
            '\'' => self.parse_string(),
            '.' => self.parse_boolean_or_enum(),
            '-' | '0'..='9' => self.parse_number(),
            'A'..='Z' | 'a'..='z' | '_' => self.parse_keyword(),
            _ => Err(format!("Unexpected character: {}", ch)),
        }
    }

    fn parse_entity_ref(&mut self) -> Result<Token, String> {
        self.advance(); // skip #

        let start = self.position;
        while self.position < self.input.len() && self.current_char().is_ascii_digit() {
            self.advance();
        }

        let num_str: String = self.input[start..self.position].iter().collect();
        let num = num_str.parse::<u32>()
            .map_err(|_| format!("Invalid entity ref: #{}", num_str))?;

        Ok(Token::EntityRef(num))
    }

    fn parse_string(&mut self) -> Result<Token, String> {
        self.advance(); // skip opening '

        let mut result = String::new();

        while self.position < self.input.len() {
            let ch = self.current_char();

            if ch == '\'' {
                self.advance();
                // Check for escaped quote ''
                if self.position < self.input.len() && self.current_char() == '\'' {
                    result.push('\'');
                    self.advance();
                } else {
                    break;
                }
            } else {
                result.push(ch);
                self.advance();
            }
        }

        Ok(Token::String(result))
    }

    fn parse_boolean_or_enum(&mut self) -> Result<Token, String> {
        self.advance(); // skip .

        let start = self.position;
        while self.position < self.input.len() && self.current_char() != '.' {
            self.advance();
        }

        if self.position >= self.input.len() {
            return Err("Unexpected end of input in enum/boolean".to_string());
        }

        let content: String = self.input[start..self.position].iter().collect();
        self.advance(); // skip closing .

        match content.as_str() {
            "T" => Ok(Token::Boolean(true)),
            "F" => Ok(Token::Boolean(false)),
            _ => Ok(Token::Keyword(format!(".{}.", content))),
        }
    }

    fn parse_number(&mut self) -> Result<Token, String> {
        let start = self.position;
        let mut has_dot = false;

        if self.current_char() == '-' {
            self.advance();
        }

        while self.position < self.input.len() {
            let ch = self.current_char();

            if ch.is_ascii_digit() {
                self.advance();
            } else if ch == '.' && !has_dot {
                has_dot = true;
                self.advance();
            } else if ch == 'E' || ch == 'e' {
                self.advance();
                if self.position < self.input.len() && (self.current_char() == '+' || self.current_char() == '-') {
                    self.advance();
                }
            } else {
                break;
            }
        }

        let num_str: String = self.input[start..self.position].iter().collect();

        if has_dot {
            let num = num_str.parse::<f64>()
                .map_err(|_| format!("Invalid float: {}", num_str))?;
            Ok(Token::Float(num))
        } else {
            let num = num_str.parse::<i64>()
                .map_err(|_| format!("Invalid integer: {}", num_str))?;
            Ok(Token::Integer(num))
        }
    }

    fn parse_keyword(&mut self) -> Result<Token, String> {
        let start = self.position;

        while self.position < self.input.len() {
            let ch = self.current_char();
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let keyword: String = self.input[start..self.position].iter().collect();
        Ok(Token::Keyword(keyword))
    }

    fn current_char(&self) -> char {
        self.input[self.position]
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() {
            let ch = self.current_char();
            if ch.is_whitespace() {
                self.advance();
            } else if ch == '/' && self.position + 1 < self.input.len() && self.input[self.position + 1] == '*' {
                // Skip /* ... */ comments
                self.position += 2;
                while self.position + 1 < self.input.len() {
                    if self.input[self.position] == '*' && self.input[self.position + 1] == '/' {
                        self.position += 2;
                        break;
                    }
                    self.advance();
                }
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_entity_ref() {
        let mut tokenizer = StepTokenizer::new("#123");
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::EntityRef(123)]);
    }

    #[test]
    fn test_tokenize_string() {
        let mut tokenizer = StepTokenizer::new("'hello world'");
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::String("hello world".to_string())]);
    }

    #[test]
    fn test_tokenize_numbers() {
        let mut tokenizer = StepTokenizer::new("123 45.67 -89.0");
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens[0], Token::Integer(123));
        assert_eq!(tokens[1], Token::Float(45.67));
        assert_eq!(tokens[2], Token::Float(-89.0));
    }

    #[test]
    fn test_tokenize_entity() {
        let mut tokenizer = StepTokenizer::new("#1=IFCWALL('guid',$,'Wall',10.5);");
        let tokens = tokenizer.tokenize().unwrap();

        assert_eq!(tokens[0], Token::EntityRef(1));
        assert_eq!(tokens[1], Token::Equal);
        assert!(matches!(tokens[2], Token::Keyword(_)));
    }
}
