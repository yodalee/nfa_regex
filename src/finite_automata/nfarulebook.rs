use std::collections::HashSet;
use std::hash::Hash;

use super::farule::FARule;

#[derive(Clone)]
pub struct NFARulebook<T> {
    rules: Vec<FARule<T>>,
}

impl<T: Eq + Clone + Hash> NFARulebook<T> {
    pub fn new(rules: Vec<FARule<T>>) -> Self {
        NFARulebook{rules: rules}
    }

    pub fn next_states(&self, states: &HashSet<T>, character: Option<char>) -> HashSet<T> {
        let mut next_states: HashSet<T> = HashSet::new();
        for state in states.iter() {
            for next_state in self.follow_rules_for(state, character) {
                next_states.insert(next_state);
            }
        }
        next_states
    }

    pub fn follow_rules_for(&self, state: &T, character: Option<char>) -> Vec<T> {
        self.rules.iter()
                  .filter(|rule| rule.applies_to(state, character))
                  .map(|rule| rule.follow())
                  .collect()
    }

    pub fn follow_free_moves(&self, states: &HashSet<T>) -> HashSet<T>{
        let more_states = self.next_states(states, None);
        if more_states.is_subset(states) {
            states.clone()
        } else {
            self.follow_free_moves(&states.union(&more_states)
                .cloned()
                .collect())
        }
    }

    pub fn rules(&self) -> Vec<FARule<T>> { self.rules.clone() }
}
