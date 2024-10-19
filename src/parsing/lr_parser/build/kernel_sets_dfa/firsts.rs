use std::collections::HashSet;

use crate::handles::{Handle, Handled};
use crate::handles::collections::{HandledVec, HandleMap};
use crate::handles::specials::OrderlyHandled;
use crate::parsing::lr_parser::rules::{GrammarSymbol, ProductionRule};

pub struct FirstsMap<'a, Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    firsts_for_nonterminals: HandleMap<Nonterminal, HashSet<Option<Handle<Terminal>>>>,
    rules: &'a HandledVec<ProductionRule<Terminal, Nonterminal, Tag>>,
    rules_for_nonterminals:
        &'a HandleMap<Nonterminal, Vec<Handle<ProductionRule<Terminal, Nonterminal, Tag>>>>,
}

impl<'a, Terminal, Nonterminal, Tag> FirstsMap<'a, Terminal, Nonterminal, Tag>
where
    Terminal: Handled,
    Nonterminal: Handled,
    Tag: OrderlyHandled,
{
    pub fn new(
        rules: &'a HandledVec<ProductionRule<Terminal, Nonterminal, Tag>>,
        rules_for_nonterminals: &'a HandleMap<
            Nonterminal,
            Vec<Handle<ProductionRule<Terminal, Nonterminal, Tag>>>,
        >,
    ) -> Self {
        let mut first_for_nonterminals = HandleMap::new();
        for nonterminal in rules_for_nonterminals.keys() {
            first_for_nonterminals.insert(nonterminal, HashSet::new());
        }
        Self {
            firsts_for_nonterminals: first_for_nonterminals,
            rules,
            rules_for_nonterminals,
        }
    }

    pub fn build(&mut self) {
        let nonterminals: Vec<Handle<Nonterminal>> = self.rules_for_nonterminals.keys().collect();
        let mut changed = true;
        while changed {
            changed = false;
            for &nonterminal in &nonterminals {
                let new_firsts = self.get_produced_firsts(nonterminal);
                let existing_firsts = self.firsts_for_nonterminals.get_mut(nonterminal).expect(
                    "Every nonterminal should have a (maybe empty) set of firsts \
                        associated with it",
                );
                for new_first in new_firsts {
                    changed |= existing_firsts.insert(new_first);
                }
            }
        }
    }

    fn get_produced_firsts(
        &self,
        nonterminal: Handle<Nonterminal>,
    ) -> HashSet<Option<Handle<Terminal>>> {
        self.rules_for_nonterminals
            .get(nonterminal)
            .expect(
                "Every nonterminal should have a (maybe empty) vector of rules associated \
                with it",
            )
            .iter()
            .flat_map(|&rule| self.firsts_for_string(&self.rules[rule].rhs))
            .collect()
    }

    fn firsts_for_grammar_symbol(
        &self,
        grammar_symbol: GrammarSymbol<Terminal, Nonterminal>,
    ) -> HashSet<Option<Handle<Terminal>>> {
        match grammar_symbol {
            GrammarSymbol::Terminal(terminal) => vec![Some(terminal)].into_iter().collect(),
            GrammarSymbol::Nonterminal(nonterminal) => self
                .firsts_for_nonterminals
                .get(nonterminal)
                .expect("Each nonterminal should have a `firsts` entry associated with it")
                .clone(),
        }
    }

    fn firsts_for_string(
        &self,
        string: &Vec<GrammarSymbol<Terminal, Nonterminal>>,
    ) -> HashSet<Option<Handle<Terminal>>> {
        let mut i = 0;
        let mut firsts = HashSet::new();
        loop {
            match string.get(i) {
                Some(&symbol) => {
                    let sub_firsts = self.firsts_for_grammar_symbol(symbol);
                    firsts.extend(sub_firsts.iter().filter(|&&x| !x.is_none()));
                    if !sub_firsts.contains(&None) {
                        return firsts;
                    } else {
                        i += 1;
                    }
                }
                None => {
                    firsts.insert(None);
                    return firsts;
                }
            }
        }
    }

    pub fn terminal_firsts_for_string(
        &self,
        string: &Vec<GrammarSymbol<Terminal, Nonterminal>>,
    ) -> impl Iterator<Item=Handle<Terminal>> {
        self.firsts_for_string(string).into_iter().filter_map(|x| x)
    }
}
