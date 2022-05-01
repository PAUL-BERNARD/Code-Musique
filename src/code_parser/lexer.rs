use std::{iter::Peekable, str::Chars};


#[derive(PartialEq)]
pub(crate) enum Token {
    //BPM(u32),
    //Signature(u8,u8),
    //FilterName(String),
    //FilterValue(String),
    //RelativeNote(u8),
    String(String),
    Value(isize),
    BpmKw,
    Solidus,
    LeftABracket,
    RightABracket,
    NewLine,
    Comma,
    LeftParenthesis,
    RightParenthesis,
    Colon,
}

impl core::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(arg0) => f.debug_tuple("Str").field(arg0).finish(),
            Self::Value(arg0) => f.debug_tuple("Num").field(arg0).finish(),
            Self::BpmKw => write!(f, "BPM"),
            Self::Solidus => write!(f, "`/`"),
            Self::LeftABracket => write!(f, "`<`"),
            Self::RightABracket => write!(f, "`>`"),
            Self::NewLine => write!(f, "NewLine"),
            Self::Comma => write!(f, "`,`"),
            Self::LeftParenthesis => write!(f, "`(`"),
            Self::RightParenthesis => write!(f, "`)`"),
            Self::Colon => write!(f, "`:`"),
        }
    }
}

pub(crate) fn tokenizer(code : String) -> Result<Vec<Token>,String> {
    let mut code_iter = code.chars().peekable();
    let mut tokens = Vec::new();

    loop {
        match code_iter.peek() {
            Some('<') => tokens.push(Token::LeftABracket),
            Some('>') => tokens.push(Token::RightABracket),
            Some('(') => tokens.push(Token::LeftParenthesis),
            Some(')') => tokens.push(Token::RightParenthesis),
            Some(',') => tokens.push(Token::Comma),
            Some(':') => tokens.push(Token::Colon),
            Some('/') => tokens.push(Token::Solidus),
            Some('\n') => {
                if tokens.last() != Some(&Token::NewLine) {
                    tokens.push(Token::NewLine)
                }
            },
            Some('1'..='9') => {
                tokens.push(parse_number(&mut code_iter)?);
                continue;
            },
            Some('a'..='z' | 'A'..='Z') => {
                tokens.push(parse_string(&mut code_iter)?);
                continue;
            },
            Some('\r') => {},
            Some(' ') => {},
            Some('\t') => {},
            None => break,
            Some(c) => panic!("Unexpected character : {}",c),
        }
        code_iter.next();
    }

    Ok(tokens)
}

fn parse_number(code_iter : &mut Peekable<Chars>) -> Result<Token, String> {
    let mut number : isize = 0;
    let mut c : Option<&char>;
    loop {
        c = code_iter.peek();
        match c {
            Some('0'..='9') => {
                let digit = c.unwrap().to_digit(10).ok_or("Failed to parse digit")?;
                number = 10*number + digit as isize;
                
            }
            _ => return Ok(Token::Value(number))
        }
        code_iter.next();
    }
}

fn parse_string(code_iter : &mut Peekable<Chars>) -> Result<Token, String> {
    let mut collector : String = String::new();
    let mut c : Option<&char>;
    'a : loop {
        c = code_iter.peek();
        match c {
            Some('a'..='z' | 'A'..='Z' | '_') => collector.push(*c.unwrap()),
            _ => break 'a,
        }
        code_iter.next();
    }

    if collector.to_uppercase() == "BPM" {
        return Ok(Token::BpmKw);
    }

    Ok(Token::String(collector))
}





/* *************TESTS*************** */


#[test]
fn tokenize_plain() {
    let plain_code = std::fs::read_to_string("./tests/codebase/plain.xfzd")
        .expect("Impossible de lire le fichier");

    let tokens = tokenizer(plain_code).unwrap();
    dbg!(tokens);
}