use crate::handle::{Handle, Handled};
use crate::handle::handle_map::HandleMap;
use crate::handle::handled_vec::HandledVec;
use crate::parsing::lr_parser::rules::GrammarSymbol;

pub struct GrammarSymbolsCollection<Terminal, Nonterminal>
where
    Terminal: Handled,
    Nonterminal: Handled,
{
    all_symbols: HandledVec<GrammarSymbol<Terminal, Nonterminal>>,
    terminals_map: HandleMap<Terminal, Handle<GrammarSymbol<Terminal, Nonterminal>>>,
    nonterminals_map: HandleMap<Nonterminal, Handle<GrammarSymbol<Terminal, Nonterminal>>>,
}

impl<Terminal, Nonterminal> GrammarSymbolsCollection<Terminal, Nonterminal>
where
    Terminal: Handled,
    Nonterminal: Handled,
{
    pub fn new(terminals: &Vec<Handle<Terminal>>, nonterminals: &Vec<Handle<Nonterminal>>) -> Self {
        let mut all_symbols = HandledVec::new();
        let mut terminals_map = HandleMap::new();
        let mut nonterminals_map = HandleMap::new();

        for &terminal in terminals {
            let symbol = all_symbols.insert(GrammarSymbol::Terminal(terminal));
            terminals_map.insert(terminal, symbol);
        }
        for &nonterminal in nonterminals {
            let symbol
                = all_symbols.insert(GrammarSymbol::Nonterminal(nonterminal));
            nonterminals_map.insert(nonterminal, symbol);
        }

        Self { all_symbols, terminals_map, nonterminals_map }
    }

    pub fn list_nonterminals<'a>(&'a self) -> impl Iterator<Item=Handle<Nonterminal>> + 'a {
        self
            .nonterminals_map
            .iter()
            .map(|(nonterminal, _)| nonterminal)
    }

    pub fn list_terminals<'a>(&'a self) -> impl Iterator<Item=Handle<Terminal>> + 'a {
        self
            .terminals_map
            .iter()
            .map(|(nonterminal, _)| nonterminal)
    }

    pub fn get_handle(&self, grammar_symbol: &GrammarSymbol<Terminal, Nonterminal>)
                      -> Handle<GrammarSymbol<Terminal, Nonterminal>>
    {
        match grammar_symbol {
            GrammarSymbol::Terminal(terminal) => {
                self.symbol_from_terminal(*terminal)
            }
            GrammarSymbol::Nonterminal(nonterminal) => {
                self.symbol_from_nonterminal(*nonterminal)
            }
        }
    }

    pub fn symbol_from_terminal(&self, terminal: Handle<Terminal>)
                                -> Handle<GrammarSymbol<Terminal, Nonterminal>>
    {
        *self.terminals_map.get(terminal).expect(
            "Every known terminal should have a handled grammar-symbol associated with it"
        )
    }

    pub fn symbol_from_nonterminal(&self, nonterminal: Handle<Nonterminal>)
                                   -> Handle<GrammarSymbol<Terminal, Nonterminal>>
    {
        *self.nonterminals_map.get(nonterminal).expect(
            "Every known nonterminal should have a handled grammar-symbol associated with it"
        )
    }
}
