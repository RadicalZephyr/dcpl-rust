use std::fmt;

use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;

use rustyline::Editor;

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
    name: String,
    prompt: String,
    continuation_prompt: String,
    buffer: String,
    editor: Editor<()>,
}

impl Interpreter {
    pub fn new(name: &'static str) -> Interpreter {
        Interpreter::new_with_prompts(name, format!("{}> ", name.to_lowercase()), "> ")
    }

    pub fn new_with_prompts(
        name: impl Into<String>,
        prompt: impl Into<String>,
        continuation_prompt: impl Into<String>,
    ) -> Interpreter {
        Interpreter {
            name: name.into(),
            prompt: prompt.into(),
            continuation_prompt: continuation_prompt.into(),
            buffer: String::new(),
            editor: Editor::new(),
        }
    }

    pub fn run(&mut self) {
        println!("Welcome to the {} interpreter!", self.name);
        self.editor.load_history(&format!("{}.txt", self.name)).ok();

        loop {
            let current_prompt = if self.buffer.is_empty() {
                &self.prompt
            } else {
                &self.continuation_prompt
            };

            let line = self.editor.readline(current_prompt);
            let line = line.expect("reading input failed...");
            self.append_line(line);

            match SExpParser::parse_line(&self.buffer) {
                Ok(sexp) => {
                    println!("read: {}", sexp);
                    self.buffer.clear();
                }
                Err(_error) => {}
            }
        }
    }

    fn append_line(&mut self, line: String) {
        if self.buffer.is_empty() {
            self.buffer = line;
        } else {
            self.buffer.push_str(&line);
        }
    }
}
