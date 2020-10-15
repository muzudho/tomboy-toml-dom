//! Single quoted string syntax parser.  
//! 単一引用符文字列構文パーサー。  

use crate::model::{
    layer110::token::{Token, TokenType},
    layer210::SingleQuotedString,
};
use crate::parser::phase200::layer210::{PResult, SingleQuotedStringP};
use casual_logger::Table;

impl SingleQuotedStringP {
    pub fn flush(&mut self) -> Option<SingleQuotedString> {
        let m = self.buffer.clone();
        self.buffer = None;
        m
    }
    pub fn new() -> Self {
        SingleQuotedStringP {
            buffer: Some(SingleQuotedString::default()),
        }
    }
    /// # Arguments
    ///
    /// * `tokens` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, tokens: (Option<&Token>, Option<&Token>, Option<&Token>)) -> PResult {
        let token0 = tokens.0.unwrap();
        match token0.type_ {
            // `'`
            TokenType::SingleQuotation => {
                // End of syntax.
                // 構文の終わり。
                return PResult::End;
            }
            _ => {
                let m = self.buffer.as_mut().unwrap();
                m.push_token(&token0);
            }
        }
        PResult::Ongoing
    }
    /// Log.  
    /// ログ。  
    pub fn log(&self) -> Table {
        let mut t = Table::default().clone();
        if let Some(m) = &self.buffer {
            t.str("value", &format!("{:?}", m));
        }
        t
    }
}
