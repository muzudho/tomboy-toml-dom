pub mod token;
pub mod token_line;

/// Token type.  
/// トークンの種類。  
#[derive(Clone, Copy, Debug)]
pub enum TokenType {
    /// \  
    Backslash,
    /// }  
    CloseCurlyBracket,
    /// ,  
    Comma,
    /// .  
    Dot,
    /// "  
    DoubleQuotation,
    EndOfLine,
    /// =  
    Equals,
    /// -  
    Hyphen,
    /// キーに使える文字で構成した単語。ドットは含まない。
    /// Numeral, Alphabet, Underscore, Hyphen に分けるか？
    KeyWithoutDotNumeralHyphenUnderscore,
    /// Multi-byte character or more.  
    /// 全角文字などいろいろ。  
    OtherwiseExceptNumeralHyphenUnderscore,
    /// {  
    LeftCurlyBracket,
    /// [  
    LeftSquareBracket,
    /// 0 ～ 9.
    Numeral,
    /// }  
    RightCurlyBracket,
    /// ]  
    RightSquareBracket,
    /// #  
    Sharp,
    /// '  
    SingleQuotation,
    /// _  
    Underscore,
    /// Whitespace means tab ('\t' 0x09) or space (' ' 0x20).  
    /// ホワイトスペースは タブ ('\t', 0x09) と 半角スペース (' ' 0x20) です。  
    WhiteSpace,
}

/// A row of tokens.  
/// 一列のトークン。  
pub struct TokenLine {
    pub row_number: usize,
    pub tokens: Vec<Token>,
}

/// Token.  
/// 字句。  
#[derive(Clone)]
pub struct Token {
    pub column_number: usize,
    pub value: String,
    pub type_: TokenType,
}
