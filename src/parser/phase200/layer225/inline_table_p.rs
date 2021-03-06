//! Inline table syntax parser.  
//! インライン・テーブル構文パーサー。  

use crate::model::layer225::InlineTable;
use crate::parser::phase200::error;
use crate::parser::phase200::error_via;
use crate::parser::phase200::{
    layer210::PResult,
    layer225::{InlineTableP, KeyvalP},
};
use casual_logger::Table as LogTable;
use look_ahead_items::LookAheadItems;

/// Inline table syntax machine state.  
/// インライン・テーブル構文状態遷移。  
///
/// Example: `{ key = value, key = value }`.  
#[derive(Debug)]
pub enum State {
    // First. After `{`.
    First,
    Keyval,
    AfterKeyval,
}

impl Default for InlineTableP {
    fn default() -> Self {
        InlineTableP {
            state: State::First,
            buffer: Some(InlineTable::default()),
            keyval_p: None,
        }
    }
}
impl InlineTableP {
    pub fn flush(&mut self) -> Option<InlineTable> {
        let m = self.buffer.clone();
        self.buffer = None;
        m
    }
    /// # Arguments
    ///
    /// * `look_ahead_items` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///               結果。
    pub fn parse(&mut self, look_ahead_items: &LookAheadItems<char>) -> PResult {
        let chr0 = look_ahead_items.get(0).unwrap();
        match self.state {
            // After `{`.
            State::First => {
                match chr0 {
                    '\t' | ' ' => {} // Ignore it.
                    // `apple.banana`
                    'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' => {
                        self.keyval_p = Some(Box::new(KeyvalP::new()));
                        self.state = State::Keyval;
                        match self.keyval_p.as_mut().unwrap().parse(look_ahead_items) {
                            PResult::End => {
                                // 1トークンでは終わらないから。
                                return error(
                                    &mut self.log(),
                                    &look_ahead_items,
                                    "inline_table.rs.64.",
                                );
                            }
                            PResult::Err(mut table) => {
                                return error_via(
                                    &mut table,
                                    &mut self.log(),
                                    &look_ahead_items,
                                    "inline_table.rs.71.",
                                )
                            }
                            PResult::Ongoing => {}
                        }
                    }
                    '}' => {
                        // Empty inline-table.
                        return PResult::End;
                    }
                    _ => return error(&mut self.log(), &look_ahead_items, "inline_table.rs.63."),
                }
            }
            // `apple.banana`.
            State::Keyval => {
                let p = self.keyval_p.as_mut().unwrap();
                match p.parse(look_ahead_items) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer.as_mut().unwrap().push_keyval(&child_m);
                            self.keyval_p = None;
                            self.state = State::AfterKeyval;
                        } else {
                            return error(
                                &mut self.log(),
                                &look_ahead_items,
                                "inline_table.rs.76.",
                            );
                        }
                    }
                    PResult::Err(mut table) => {
                        return error_via(
                            &mut table,
                            &mut self.log(),
                            &look_ahead_items,
                            "inline_table.rs.80.",
                        )
                    }
                    PResult::Ongoing => {}
                }
            }
            // After `banana = 3`.
            State::AfterKeyval => match chr0 {
                '\t' | ' ' => {} // Ignore it.
                // `,`
                ',' => {
                    self.state = State::First;
                }
                // `}`
                '}' => {
                    return PResult::End;
                }
                _ => return error(&mut self.log(), &look_ahead_items, "inline_table.rs.96."),
            },
        }
        PResult::Ongoing
    }

    /// Log.  
    /// ログ。  
    pub fn log(&self) -> LogTable {
        let mut t = LogTable::default()
            .str("parser", "InlineTableP#parse")
            .str("state", &format!("{:?}", self.state))
            .clone();
        if let Some(p) = &self.keyval_p {
            t.sub_t("keyval", &p.log());
        }
        t
    }
}
