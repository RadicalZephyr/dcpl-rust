use std::fmt;

use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;

use rustyline::error::ReadlineError;
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
            Rule::float => SExp::Float(pair.as_str().parse().unwrap()),
            Rule::integer => SExp::Integer(pair.as_str().parse().unwrap()),
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

#[derive(Clone, Debug, PartialEq)]
pub enum SExp {
    List(Vec<SExp>),
    Float(f64),
    Integer(i128),
    String(String),
    Symbol(String),
}

impl SExp {
    pub fn string(content: impl Into<String>) -> SExp {
        SExp::String(content.into())
    }

    pub fn symbol(content: impl Into<String>) -> SExp {
        SExp::Symbol(content.into())
    }

    pub fn into_list(self) -> Option<Vec<SExp>> {
        match self {
            SExp::List(exprs) => Some(exprs),
            _ => None,
        }
    }

    pub fn into_float(self) -> Option<f64> {
        match self {
            SExp::Float(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_integer(self) -> Option<i128> {
        match self {
            SExp::Integer(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
            SExp::Symbol(name) => Some(name),
            _ => None,
        }
    }

    pub fn into_symbol(self) -> Option<String> {
        match self {
            SExp::Symbol(name) => Some(name),
            _ => None,
        }
    }
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
            Float(val) => write!(f, "{}", val),
            Integer(val) => write!(f, "{}", val),
            Symbol(content) => write!(f, "{}", content),
            String(content) => write!(f, "\"{}\"", content),
        }
    }
}

pub struct Interpreter<F> {
    name: String,
    prompt: String,
    continuation_prompt: String,
    buffer: String,
    editor: Editor<()>,
    interpret: F,
}

impl<F> Interpreter<F>
where
    F: FnMut(SExp) -> Option<String>,
{
    pub fn new(name: &'static str, interpret: F) -> Interpreter<F> {
        Interpreter::new_with_prompts(name, format!("{}> ", name.to_lowercase()), "> ", interpret)
    }

    pub fn new_with_prompts(
        name: impl Into<String>,
        prompt: impl Into<String>,
        continuation_prompt: impl Into<String>,
        interpret: F,
    ) -> Interpreter<F> {
        Interpreter {
            name: name.into(),
            prompt: prompt.into(),
            continuation_prompt: continuation_prompt.into(),
            buffer: String::new(),
            editor: Editor::new(),
            interpret,
        }
    }

    pub fn run(&mut self) {
        println!("Welcome to the {} interpreter!", self.name);
        self.editor.load_history(&self.history_file_name()).ok();

        loop {
            let current_prompt = if self.buffer.is_empty() {
                &self.prompt
            } else {
                &self.continuation_prompt
            };

            let line = self
                .editor
                .readline_with_initial(current_prompt, (&self.buffer, &""));
            match line {
                Ok(line) => {
                    self.append_line(line);

                    match SExpParser::parse_line(&self.buffer) {
                        Ok(sexp) => {
                            if let Some(output) = (self.interpret)(sexp) {
                                println!("{}", output);
                            }
                            self.editor.add_history_entry(self.buffer.as_ref());
                            self.buffer.clear();
                        }
                        Err(_error) => {}
                    }
                }
                Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }

        self.editor
            .save_history(&self.history_file_name())
            .expect("saving history file failed...");
    }

    fn history_file_name(&self) -> String {
        format!("{}.txt", self.name)
    }

    fn append_line(&mut self, line: String) {
        if self.buffer.is_empty() {
            self.buffer = line;
        } else {
            self.buffer.push_str(&line);
        }
    }
}

#[cfg(test)]
mod test {
    use super::SExp::*;
    use super::*;

    fn parse(input: impl AsRef<str>) -> SExp {
        SExpParser::parse_line(input).expect("unexpected parse error")
    }

    #[test]
    fn test_parse_symbol() {
        assert_eq!(Symbol("world".into()), parse("world"));
    }

    #[test]
    fn test_parse_string() {
        assert_eq!(String("hello".into()), parse("\"hello\""));
    }

    #[test]
    fn test_parse_integer() {
        assert_eq!(Integer(10), parse("10"));
    }

    #[test]
    fn test_parse_negative_integer() {
        assert_eq!(Integer(-5), parse("-5"));
    }

    #[test]
    fn test_parse_float() {
        assert_eq!(Float(1.0), parse("1.0"));
    }

    #[test]
    fn test_parse_negative_float() {
        assert_eq!(Float(-2.0), parse("-2.0"));
    }

    #[test]
    fn test_parse_fractional_float() {
        assert_eq!(Float(0.1), parse("0.1"));
    }

    #[test]
    fn test_parse_empty_list() {
        assert_eq!(List(vec![]), parse("()"));
    }

    #[test]
    fn test_parse_singleton_list() {
        assert_eq!(List(vec![Integer(1)]), parse("(1)"));
    }

    #[test]
    fn test_parse_list() {
        assert_eq!(
            List(vec![Integer(1), Integer(2), Integer(3)]),
            parse("(1 2 3)")
        );
    }
}
