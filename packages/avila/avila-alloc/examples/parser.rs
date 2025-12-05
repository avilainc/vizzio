//! Parser example using stack-only allocations
//!
//! Demonstrates parsing without any heap allocation

use avila_alloc::{StackVec, StackString};

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    value: StackString<32>,
}

#[derive(Debug, PartialEq)]
enum TokenKind {
    Number,
    Operator,
    Identifier,
}

fn tokenize(input: &str) -> StackVec<Token, 32> {
    let mut tokens = StackVec::new();
    let mut current = StackString::<32>::new();

    for c in input.chars() {
        if c.is_whitespace() {
            if !current.is_empty() {
                let kind = if current.as_str().chars().all(|c| c.is_numeric()) {
                    TokenKind::Number
                } else if current.len() == 1 && "+-*/".contains(current.as_str()) {
                    TokenKind::Operator
                } else {
                    TokenKind::Identifier
                };

                let _ = tokens.push(Token { kind, value: current });
                current = StackString::new();
            }
        } else {
            let _ = current.push(c);
        }
    }

    if !current.is_empty() {
        let kind = if current.as_str().chars().all(|c| c.is_numeric()) {
            TokenKind::Number
        } else if current.len() == 1 && "+-*/".contains(current.as_str()) {
            TokenKind::Operator
        } else {
            TokenKind::Identifier
        };
        let _ = tokens.push(Token { kind, value: current });
    }

    tokens
}

fn main() {
    println!("=== Stack-Only Parser Example ===\n");

    let input = "x + 42 * y - 10";
    println!("Input: {}", input);

    let tokens = tokenize(input);

    println!("\nTokens (no heap allocation!):");
    for (i, token) in tokens.iter().enumerate() {
        println!("  {}: {:?} = '{}'", i, token.kind, token.value.as_str());
    }

    println!("\nTotal tokens: {}", tokens.len());
    println!("Stack usage: {} / {} tokens", tokens.len(), tokens.capacity());
}
