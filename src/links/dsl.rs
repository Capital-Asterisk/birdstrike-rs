use super::*;
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
pub enum Token {
    #[regex("\\#[^\\n]*")]
    Comment,
    
    #[regex("\\*[ \t]*[a-zA-Z0-9]+[ \t]*:[ \t]*[a-zA-Z0-9]+")]
    Item,
    
    #[regex("\\*[ \t]*[a-zA-Z0-9]+[ \t]*")]
    ItemExtra,
    
    #[regex("\\+[ \t]*[a-zA-Z0-9]+[ \t]*:[ \t]*[a-zA-Z0-9]+")]
    Connection,
    
    #[token("(")]
    BraceOpen,
    
    #[token(",")]
    Comma,
    
    #[token(")")]
    BraceClose,
    
    #[regex("\"(\\.|[^\"])*\"")]
    ValueQuotedString,
    
    #[regex("[+-]?[0-9]*[.]?[0-9]+(?:[eE][+-]?[0-9]+)?", priority = 2)]
    ValueNumber,
    
    #[regex("[a-zA-Z0-9]+", priority = 1)]
    ValueEnum
}


fn geah() {
    let a: ElemAnyId = 3;
}

