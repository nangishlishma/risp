use std::env;
use std::fmt;
use std::fs;

#[derive(Debug, PartialEq)]
enum Token {
    LParen,
    RParen,
    Number(i64),
    Word(String),
}

#[derive(Debug, Clone, PartialEq)]
enum Node {
    Null,
    List(Vec<Node>),
    Number(i64),
    Word(String),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Null => write!(f, "Null"),
            Node::Number(n) => write!(f, "{}", n),
            Node::Word(s) => write!(f, "{}", s),
            Node::List(list) => {
                write!(f, "(")?;
                for (i, obj) in list.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", obj)?;
                }
                write!(f, ")")
            }
        }
    }
}

fn lex(chars: &mut Vec<char>) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    chars.reverse();

    let mut ch = chars.pop().unwrap();
    while !chars.is_empty() {
        match ch {
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            ';' => {
                while !chars.is_empty() && ch != '\n' {
                    ch = chars.pop().unwrap();
                }
                continue;
            }
            _ => {
                let mut word = String::new();
                if ch.is_numeric() {
                    while ch.is_numeric() && !chars.is_empty() {
                        word.push(ch);
                        ch = chars.pop().unwrap();
                    }
                    tokens.push(Token::Number(word.parse::<i64>().unwrap()));
                    continue;
                } else if !ch.is_whitespace() {
                    while !ch.is_whitespace() && !chars.is_empty() {
                        word.push(ch);
                        ch = chars.pop().unwrap();
                    }
                    tokens.push(Token::Word(word));
                    continue;
                }
            }
        }
        ch = chars.pop().unwrap()
    }

    tokens.reverse();
    tokens
}

fn parse(tokens: &mut Vec<Token>) -> Node {
    let mut list: Vec<Node> = Vec::new();

    while !tokens.is_empty() {
        let token = tokens.pop().unwrap();
        match token {
            Token::LParen => list.push(parse(tokens)),
            Token::RParen => break,
            Token::Number(n) => list.push(Node::Number(n)),
            Token::Word(w) => list.push(Node::Word(w)),
        }
    }

    Node::List(list)
}

fn interpret(program: &Node) {
    let result = interp_node(program);
    println!("{:?}", result);
}

fn interp_node(node: &Node) -> Node {
    match node {
        Node::List(l) => interp_list(l),
        Node::Word(w) => interp_word(w),
        _ => node.clone(),
    }
}

fn interp_list(list: &Vec<Node>) -> Node {
    match &list[0] {
        Node::Word(w) => match w.as_str() {
            "+" => interp_binop(&list),
            _ => Node::Null,
        },
        _ => {
            let mut new_list: Vec<Node> = Vec::new();
            for node in list {
                let result = interp_node(node);
                if result != Node::Null {
                    new_list.push(result);
                }
            }
            Node::List(new_list)
        }
    }
}

fn interp_binop(list: &Vec<Node>) -> Node {
    let left = &interp_node(&list[1]);
    let right = &interp_node(&list[2]);
    if let Node::Word(w) = &list[0] {
        match w.as_str() {
            "+" => match (left, right) {
                (Node::Number(l), Node::Number(r)) => return Node::Number(l + r),
                _ => Node::Null,
            },
            _ => Node::Null,
        };
    }
    Node::Null
}

fn interp_word(word: &String) -> Node {
    Node::Null
}

fn main() {
    let mut args = env::args();
    if args.len() < 2 {
        panic!("No file provided.");
    } else {
        let file_loc = args.nth(1).unwrap();

        let mut chars: Vec<char> = fs::read_to_string(file_loc)
            .expect("Failed to read the file.")
            .replace("(", " ( ")
            .replace(")", " ) ")
            .chars()
            .collect();

        let mut tokens = lex(&mut chars);
        let program = parse(&mut tokens);
        interpret(&program);
    }
}
