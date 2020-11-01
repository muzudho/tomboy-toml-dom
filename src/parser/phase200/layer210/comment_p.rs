//! Comment syntax parser.  
//! コメント構文パーサー。  

use crate::model::layer210::NonEol;
use crate::model::{
    layer110::{CharacterType, TokenType},
    layer210::Comment,
};
use crate::parser::phase200::error;
use crate::parser::phase200::error_via;
use crate::parser::phase200::layer210::{non_eol_p::Judge as NonEolPJudge, NonEolP};
use crate::parser::phase200::layer210::{CommentP, PResult};
use crate::parser::phase200::Character;
use crate::parser::phase200::LookAheadCharacters;
use crate::parser::phase200::Token;
use casual_logger::Table;

/// Syntax machine state.  
/// 構文状態遷移。  
#[derive(Debug, Clone)]
pub enum State {
    End,
    First,
    NonEol,
}

pub enum Judge {
    CommentStartSymbol(Character),
    CommentCharacter(NonEol),
}

impl CommentP {
    pub fn new() -> Self {
        CommentP {
            product: Comment::default(),
            state: State::First,
        }
    }
    pub fn get_product(&mut self) -> Comment {
        self.product.clone()
    }
    /// # Arguments
    ///
    /// * `token` - Token.  
    ///             トークン。  
    /// # Returns
    ///
    /// * `bool` - このパーサーの対象とするトークンになる.  
    ///                             結果。
    pub fn judge1(&self, character: &Character) -> Option<Judge> {
        match self.state {
            State::End => None,
            State::First => match character.type_ {
                CharacterType::CommentStartSymbol => {
                    Some(Judge::CommentStartSymbol(character.clone()))
                }
                _ => panic!("comment_p.rs.57. type={:?}", character.type_),
            },
            State::NonEol => {
                if let Some(judge) = NonEolP::judge(character) {
                    match judge {
                        NonEolPJudge::Ascii(ch)
                        | NonEolPJudge::HorizontalTab(ch)
                        | NonEolPJudge::NonAscii(ch) => Some(Judge::CommentCharacter(ch)),
                    }
                } else {
                    None
                }
            }
        }
    }
    pub fn commit1(&mut self, judge: &Judge) {
        match self.state {
            State::End => {
                panic!("comment_p.rs.61.");
            }
            State::First => match judge {
                Judge::CommentStartSymbol(ch) => {
                    self.product
                        .push_token(&Token::from_character(&ch, TokenType::Comment));
                }
                Judge::CommentCharacter(ch) => {
                    panic!("comment_p.rs.82.");
                }
            },
            State::NonEol => match judge {
                Judge::CommentStartSymbol(_ch) => {
                    panic!("comment_p.rs.108.");
                }
                Judge::CommentCharacter(non_eol) => {
                    self.product.push_token(&Token::from_character(
                        &non_eol.get_character(),
                        TokenType::Comment,
                    ));
                }
            },
        }
    }
    /// # Arguments
    ///
    /// * `characters` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn forward1(&mut self, characters: &LookAheadCharacters) -> PResult {
        match self.state {
            State::End => {
                panic!("comment_p.rs.61.");
            }
            State::First | State::NonEol => {
                // 次の１文字。
                let character1 = characters.current.as_ref().unwrap();
                if let None = NonEolP::judge(character1) {
                    self.state = State::End;
                    return PResult::End;
                }
            }
        }

        PResult::Ongoing
    }
    /// Log.  
    /// ログ。  
    pub fn log(&self) -> Table {
        let mut t = Table::default().clone();
        t.str("product", &self.product.to_string());
        t
    }
}
