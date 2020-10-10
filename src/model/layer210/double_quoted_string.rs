//! Double quoted string model.  
//! 二重引用符文字列モデル。  
//!
//! # Examples
//!
//! ```
//! // "ハロー"
//! ```

use crate::model::{layer110::token::Token, layer210::DoubleQuotedString};
use std::fmt;

impl Default for DoubleQuotedString {
    fn default() -> Self {
        DoubleQuotedString {
            value: String::new(),
        }
    }
}
impl DoubleQuotedString {
    pub fn push_token(&mut self, token: &Token) {
        self.value.push_str(&token.value);
    }
}
impl fmt::Debug for DoubleQuotedString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.value)
    }
}