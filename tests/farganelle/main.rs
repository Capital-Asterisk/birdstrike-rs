use birdstrikers::links::*;
use birdstrikers::links::stupidlgrn::*;

use logos::Logos;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_iterator() {

        let mut aaa = bitslice_it(&[0, 1 | (1 << 20) | (1 << 62), 0, 0, 1]);
        assert_eq!(aaa.next(), Some(64));
        assert_eq!(aaa.next(), Some(84));
        assert_eq!(aaa.next(), Some(126));
        assert_eq!(aaa.next(), Some(256));
        assert_eq!(aaa.next(), None);
        
        let mut aaa = bitslice_it(&[0, 0, 0, 1, 0, 0, 1]);
        assert_eq!(aaa.next(), Some(192));
        assert_eq!(aaa.next(), Some(384));
        assert_eq!(aaa.next(), None);
        
    }

    #[test]
    fn test_dsl() {
        let dslin = "
*foo:  NUM      30
*bar:  PNT      (2, 2)
*qux:  STR      \"hi there *screams*\"
*nand: LFUNC0   nand

# comment
*gate0: LogicGate  + LFUNC0:NAND
    + n0:qux
    + port:name   # another comment
    + PNT0:foo +PNT1:h

*gate0 +n0:23";
        
        let mut lex = dsl::Token::lexer(dslin);
        
        while let Some(tkn) = lex.next() {
            println!("{}    >>> {:?}", lex.slice(), tkn);
        }
        
        
        
    
    }

    

}
