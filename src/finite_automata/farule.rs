use super::faruledata::FARuleData;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug,Clone)]
pub struct FARule<T> {
    pub state: T,
    pub next_state: T,
    kind: FARuleType
}

#[derive(Debug,Clone,PartialEq)]
enum FARuleType {
    RuleChar { character: char },
    RuleFree,
    RuleAny,
    RuleSet { set: Vec<FARuleData>, reverse: bool },
}

impl<T: Eq + PartialEq + Clone> FARule<T> {
    pub fn new_rulechar(state: &T, character: char, next_state: &T) -> Self {
        FARule {
            state: state.clone(),
            next_state: next_state.clone(),
            kind: FARuleType::RuleChar {
                character: character
            }
        }
    }

    pub fn new_rulefree(state: &T, next_state: &T) -> Self {
        FARule {
            state: state.clone(),
            next_state: next_state.clone(),
            kind: FARuleType::RuleFree
        }
    }

    pub fn new_ruleany(state: &T, next_state: &T) -> Self {
        FARule {
            state: state.clone(),
            next_state: next_state.clone(),
            kind: FARuleType::RuleAny
        }
    }

    pub fn new_ruleset(state: &T, next_state: &T, set: &Vec<FARuleData>, reverse: bool) -> Self {
        FARule {
            state: state.clone(),
            next_state: next_state.clone(),
            kind: FARuleType::RuleSet {
                set: set.clone(),
                reverse: reverse
            }
        }
    }

    pub fn applies_to(&self, state: &T, c: Option<char>) -> bool {
        self.state == *state && match c {
            Some(c) => match self.kind {
                FARuleType::RuleChar { character } => character == c,
                FARuleType::RuleFree => false,
                FARuleType::RuleAny => true,
                FARuleType::RuleSet { ref set, reverse } => {
                    reverse ^ set.iter().any(|data| data.applies_to(&c))
                }
            }
            None => self.kind == FARuleType::RuleFree
        }
    }

    pub fn follow(&self) -> T {
        self.next_state.clone()
    }
}

impl<T: Display> Display for FARule<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let describe = match self.kind {
            FARuleType::RuleChar { character } => character.to_string(),
            FARuleType::RuleFree => "free".to_string(),
            FARuleType::RuleAny => "any".to_string(),
            FARuleType::RuleSet { ref set, reverse } => {
                format!("[{}{}]", if reverse {"^"} else {""},
                    set.iter().map(|data| format!("{}", data)).collect::<Vec<String>>().join(""))
            }
        };
        write!(f, "FARule {} --{}--> {}", self.state, describe, self.next_state)
    }
}
