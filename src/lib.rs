use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "sexp.pest"]
pub struct SExpParser;

pub enum SExp {
    List(Vec<SExp>),
    Number(i64),
    String(String),
    Symbol(String),
}

impl SExpParser {
    pub fn parse_sexp(input: impl AsRef<str>) -> Vec<SExp> {
        let input = input.as_ref();
        let file = SExpParser::parse(Rule::file, input)
            .expect("unsuccessful parse...")
            .next()
            .unwrap();

        SExpParser::parse_list(file.into_inner())
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
