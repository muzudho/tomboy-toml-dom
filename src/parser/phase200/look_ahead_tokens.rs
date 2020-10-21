//! Look-ahead tokens.  
//! 先読みトークン。  

use crate::parser::phase200::LookAheadTokens;
use crate::parser::phase200::Token;

impl Default for LookAheadTokens {
    fn default() -> Self {
        LookAheadTokens {
            current: None,
            one_ahead: None,
            two_ahead: None,
            three_ahead: None,
            four_ahead: None,
        }
    }
}
impl LookAheadTokens {
    pub fn push(&mut self, token: Option<Token>) {
        self.current = self.one_ahead.clone();
        self.one_ahead = self.two_ahead.clone();
        self.two_ahead = self.three_ahead.clone();
        self.three_ahead = self.four_ahead.clone();
        self.four_ahead = token;
    }
}
