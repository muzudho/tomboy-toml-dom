//! Syntax parser.
//! 構文パーサー。

use crate::lexical_parser::Token;
use crate::lexical_parser::TokenType;
use crate::object_model::line::{LineItemModel, LineM};
use crate::syntax::comment::CommentP;
use crate::syntax::key_value::KeyValueP;
use crate::syntax::SyntaxParserResult;
use casual_logger::{Log, Table};

pub struct LineP {
    state: MachineState,
    comment_syntax: Option<CommentP>,
    key_value_syntax: Option<KeyValueP>,
}
impl Default for LineP {
    fn default() -> Self {
        LineP {
            state: MachineState::First,
            comment_syntax: None,
            key_value_syntax: None,
        }
    }
}
impl LineP {
    pub fn product(&self) -> LineM {
        let mut product = LineM::default();
        if let Some(p) = &self.comment_syntax {
            product.items.push(LineItemModel::Comment(p.product()));
        }
        product
    }

    /// # Returns
    ///
    /// * `SyntaxParserResult` - Result.  
    ///                             結果。    
    pub fn parse(&mut self, token: &Token) -> SyntaxParserResult {
        match self.state {
            MachineState::CommentSyntax => {
                self.comment_syntax.as_mut().unwrap().parse(token);
            }
            MachineState::First => match token.type_ {
                TokenType::Key => {
                    /*
                    Log::info_t(
                        "LineP#parse",
                        Table::default()
                            .str("parser", "LineP#parse")
                            .str("state", &format!("{:?}", self.state))
                            .str("token", &format!("{:?}", token)),
                    );
                    */
                    self.key_value_syntax = Some(KeyValueP::new(&token.value));
                    self.state = MachineState::KeyPairSyntax;
                }
                TokenType::Sharp => {
                    self.comment_syntax = Some(CommentP::new());
                    self.state = MachineState::CommentSyntax;
                }
                _ => {
                    self.state = MachineState::Unimplemented;
                }
            },
            MachineState::KeyPairSyntax => {
                if let Some(key_value_syntax) = &mut self.key_value_syntax {
                    match key_value_syntax.parse(token) {
                        SyntaxParserResult::Ok(_) => {} // Ignored it.
                        SyntaxParserResult::Err(table) => {
                            return SyntaxParserResult::Err(
                                Table::default()
                                    .str("parser", "LineP#parse")
                                    .str("state", &format!("{:?}", self.state))
                                    .str("token", &format!("{:?}", token))
                                    .sub_t("error", &table)
                                    .clone(),
                            );
                        }
                    }
                } else {
                    panic!(Log::fatal_t(
                        "LineP#parse",
                        Table::default()
                            .str("parser", "LineP#parse")
                            .str("state", &format!("{:?}", self.state))
                            .str("token", &format!("{:?}", token))
                    ));
                }
            }
            MachineState::Unimplemented => {
                return SyntaxParserResult::Err(
                    Table::default()
                        .str("parser", "LineP#parse")
                        .str("state", &format!("{:?}", self.state))
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
        }

        SyntaxParserResult::Ok(false)
    }
    pub fn log(&self) -> Table {
        let mut t = Table::default()
            .str("state", &format!("{:?}", self.state))
            .clone();
        if let Some(comment_syntax) = &self.comment_syntax {
            t.sub_t("comment", &comment_syntax.log());
        }
        if let Some(key_value_syntax) = &self.key_value_syntax {
            t.sub_t("key_value", &key_value_syntax.log());
        }
        t
    }
}

#[derive(Debug)]
enum MachineState {
    /// `# comment`.
    CommentSyntax,
    First,
    /// `key = right_value`.
    KeyPairSyntax,
    Unimplemented,
}