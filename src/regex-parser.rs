extern crate nfa_regex;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use nfa_regex::finite_automata::faruledata::{FARuleData};
use nfa_regex::regular_expressions::regex::{Regex};
use nfa_regex::regular_expressions::tonfa::{ToNFA};
use pest::Parser;
use pest::iterators::{Pair};
use std::env;

#[cfg(debug_assertions)]
const _GRAMMER: &'static str = include_str!("regex.pest");

#[derive(Parser)]
#[grammar = "regex.pest"]
struct RegexParser;

pub fn main() {
    let args : Vec<_> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: regex-parser \"pattern\" string");
        eprintln!("Add double quote to prevent shell expand | and *");
    }

    let pair = RegexParser::parse(Rule::choose, &args[1])
                .unwrap_or_else(|e| panic!("{}", e))
                .next().unwrap();
    let pattern = build_regex(pair);
    if pattern.matches(&args[2]) {
        println!("Pattern {} can matches {}", args[1], args[2]);
    } else {
        println!("Pattern {} cannot matches {}", args[1], args[2]);
    }
}

fn build_set(pair: Pair<Rule>, reverse: bool) -> Box<Regex> {
    let mut set = Vec::new();
    let inner = pair.into_inner();
    for pair in inner {
        println!("rule: {:?}", pair.as_rule());
        set.push(match pair.as_rule() {
            Rule::range => {
                let mut inner = pair.into_inner();
                let start = inner.next().unwrap().into_span().as_str().chars().next().unwrap();
                let end = inner.next().unwrap().into_span().as_str().chars().next().unwrap();
                FARuleData::range(start, end)
            }
            Rule::character => {
                let c = pair.into_span().as_str().chars().next().unwrap();
                FARuleData::char(c)
            }
            _ => unreachable!("Unexpected rule: {}", pair),
        })
    }
    Regex::set(&set, reverse)
}

fn build_regex(pair: Pair<Rule>) -> Box<Regex> {
    println!("rule: {:?}", pair.as_rule());
    match pair.as_rule() {
        Rule::empty => Regex::empty(),
        Rule::character => Regex::literal(pair
            .into_span().as_str().chars().next().unwrap()),
        Rule::reverse_set => {
            let mut inner = pair.into_inner();
            let may_op = inner.next().unwrap();
            if may_op.as_rule() == Rule::op_not {
                build_set(inner.next().unwrap(), true)
            } else {
                build_set(may_op, false)
            }
        }
        Rule::repeat => {
            let mut inner = pair.into_inner();
            let regex = build_regex(inner.next().unwrap());
            match inner.next() {
                Some(pair) => match pair.as_rule() {
                    Rule::op_repeat => Regex::repeat(regex),
                    Rule::op_plus => Regex::plus(regex),
                    Rule::op_optional => Regex::optional(regex),
                    _ => unreachable!("Unexpected rule: {:?}", pair.as_rule()),
                }
                None => regex,
            }
        },
        Rule::choose => {
            let mut inner = pair.into_inner();
            let fst = build_regex(inner.next().unwrap());
            match inner.next() {
                Some(rest) => {
                    Regex::choose(fst, build_regex(rest))
                },
                None => fst,
            }
        },
        Rule::concat => {
            let mut inner = pair.into_inner();
            let fst = build_regex(inner.next().unwrap());
            match inner.next() {
                Some(rest) => Regex::concatenate(fst, build_regex(rest)),
                None => fst,
            }
        },
        _ => unreachable!("Unexpected rule: {}", pair),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regexparser_empty() {
        let pair = RegexParser::parse(Rule::choose, "")
                    .unwrap_or_else(|e| panic!("{}", e))
                    .next().unwrap();
        let pattern = build_regex(pair);
        assert!(pattern.matches(""));
        assert!(!pattern.matches("a"));
    }

    #[test]
    fn test_regexparser_literal() {
        let pair = RegexParser::parse(Rule::choose, "a")
                    .unwrap_or_else(|e| panic!("{}", e))
                    .next().unwrap();
        let pattern = build_regex(pair);
        assert!(!pattern.matches(""));
        assert!(pattern.matches("a"));
    }

    #[test]
    fn test_regexparser_set_char() {
        let pair = RegexParser::parse(Rule::choose, "[aeiouAEIOU]")
                    .unwrap_or_else(|e| panic!("{}", e))
                    .next().unwrap();
        let pattern = build_regex(pair);
        assert!(!pattern.matches(""));
        assert!(pattern.matches("e"));
        assert!(!pattern.matches("r"));
        assert!(!pattern.matches("AB"));
    }

    #[test]
    fn test_regexparser_set_range() {
        let pair = RegexParser::parse(Rule::choose, "[a-z]")
                    .unwrap_or_else(|e| panic!("{}", e))
                    .next().unwrap();
        let pattern = build_regex(pair);
        assert!(!pattern.matches(""));
        assert!(pattern.matches("e"));
        assert!(pattern.matches("r"));
        assert!(!pattern.matches("A"));
        assert!(!pattern.matches("ww"));
    }

    #[test]
    fn test_regexparser_set_reverse() {
        let pair = RegexParser::parse(Rule::choose, "[^a-z]")
                    .unwrap_or_else(|e| panic!("{}", e))
                    .next().unwrap();
        let pattern = build_regex(pair);
        assert!(!pattern.matches(""));
        assert!(!pattern.matches("e"));
        assert!(!pattern.matches("r"));
        assert!(pattern.matches("A"));
        assert!(pattern.matches("1"));
        assert!(!pattern.matches("ww"));
    }

    #[test]
    fn test_regexparser_repeat() {
        let pair = RegexParser::parse(Rule::choose, "a*")
                    .unwrap_or_else(|e| panic!("{}", e))
                    .next().unwrap();
        let pattern = build_regex(pair);
        assert!(pattern.matches(""));
        assert!(pattern.matches("a"));
        assert!(!pattern.matches("b"));
        assert!(pattern.matches("aaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
    }

    #[test]
    fn test_regexparser_plus() {
        let pair = RegexParser::parse(Rule::choose, "a*")
                    .unwrap_or_else(|e| panic!("{}", e))
                    .next().unwrap();
        let pattern = build_regex(pair);
        assert!(pattern.matches(""));
        assert!(pattern.matches("a"));
        assert!(!pattern.matches("b"));
        assert!(pattern.matches("aaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
    }
    #[test]
    fn test_regexparser_optional() {
        let pair = RegexParser::parse(Rule::choose, "a?")
                    .unwrap_or_else(|e| panic!("{}", e))
                    .next().unwrap();
        let pattern = build_regex(pair);
        assert!(pattern.matches(""));
        assert!(pattern.matches("a"));
        assert!(!pattern.matches("b"));
        assert!(!pattern.matches("aaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
    }

    #[test]
    fn test_regexparser_choose() {
        let pair = RegexParser::parse(Rule::choose, "a|b")
                    .unwrap_or_else(|e| panic!("{}", e))
                    .next().unwrap();
        let pattern = build_regex(pair);
        assert!(!pattern.matches(""));
        assert!(pattern.matches("a"));
        assert!(pattern.matches("b"));
        assert!(!pattern.matches("ab"));
    }

    #[test]
    fn test_regexparser_concat() {
        let pair = RegexParser::parse(Rule::choose, "abcd")
                    .unwrap_or_else(|e| panic!("{}", e))
                    .next().unwrap();
        let pattern = build_regex(pair);
        assert!(!pattern.matches(""));
        assert!(!pattern.matches("a"));
        assert!(!pattern.matches("b"));
        assert!(pattern.matches("abcd"));
        assert!(!pattern.matches("abcdefg"));
    }
}
