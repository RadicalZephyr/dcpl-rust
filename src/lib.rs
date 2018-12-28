use std::{
    fmt,
    io::{self, BufRead, Write},
};

use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "sexp.pest"]
pub struct SExpParser;

type ParseError = pest::error::Error<Rule>;

impl SExpParser {
    pub fn parse_file(input: impl AsRef<str>) -> Vec<SExp> {
        let input = input.as_ref();
        let file = SExpParser::parse(Rule::file, input)
            .expect("unsuccessful parse...")
            .next()
            .unwrap();

        SExpParser::parse_list(file.into_inner())
    }

    pub fn parse_line(input: impl AsRef<str>) -> Result<SExp, ParseError> {
        let input = input.as_ref();
        let sexp = SExpParser::parse(Rule::sexp, input)?.next().unwrap();

        Ok(SExpParser::parse_rule(sexp))
    }

    fn parse_rule(pair: Pair<Rule>) -> SExp {
        match pair.as_rule() {
            Rule::list => SExp::List(SExpParser::parse_list(pair.into_inner())),
            Rule::number => SExp::Number(pair.as_str().parse().unwrap()),
            Rule::string => {
                let content = pair.as_str();
                let len = content.len();
                let content = &content[1..len - 1]; // drop the quotes
                SExp::String(content.into())
            }
            Rule::symbol => SExp::Symbol(pair.as_str().into()),
            _ => unreachable!(),
        }
    }

    fn parse_list(pairs: Pairs<Rule>) -> Vec<SExp> {
        pairs.map(SExpParser::parse_rule).collect()
    }
}

pub enum SExp {
    List(Vec<SExp>),
    Number(i64),
    String(String),
    Symbol(String),
}

impl fmt::Display for SExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::SExp::*;
        match self {
            List(exprs) => {
                write!(f, "(")?;
                let mut sep = "";
                for exp in exprs {
                    write!(f, "{}{}", sep, exp)?;
                    sep = " ";
                }
                write!(f, ")")
            }
            Number(val) => write!(f, "{}", val),
            Symbol(content) => write!(f, "{}", content),
            String(content) => write!(f, "\"{}\"", content),
        }
    }
}

pub struct Interpreter {
    prompt: String,
    continuation_prompt: String,
    buffer: String,
}

impl Interpreter {
    pub fn new(prompt: impl Into<String>, continuation_prompt: impl Into<String>) -> Interpreter {
        let prompt = prompt.into();
        let continuation_prompt = continuation_prompt.into();
        let buffer = String::new();
        Interpreter {
            prompt,
            continuation_prompt,
            buffer,
        }
    }

    pub fn run(&mut self) {
        let stdin_handle = io::stdin();
        let stdin = stdin_handle.lock();

        self.prompt();
        for line in stdin.lines() {
            let line = line.expect("reading input failed...");
            self.append_line(line);

            match SExpParser::parse_line(&self.buffer) {
                Ok(sexp) => {
                    println!("read: {}", sexp);
                    self.buffer.clear();
                }
                Err(_error) => {}
            }
            self.prompt();
        }
    }

    fn append_line(&mut self, line: String) {
        if self.buffer.is_empty() {
            self.buffer = line;
        } else {
            self.buffer.push_str(&line);
        }
    }

    fn prompt(&self) {
        let prompt = if self.buffer.is_empty() {
            &self.prompt
        } else {
            &self.continuation_prompt
        };
        print!("{} ", prompt);
        io::stdout().lock().flush().ok();
    }
}
