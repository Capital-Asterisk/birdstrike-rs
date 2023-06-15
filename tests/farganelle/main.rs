use birdstrikers::links::*;
use birdstrikers::links::stupidlgrn::*;


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
            *foo:   NUM      30
            *bar:   NUM
            *rock:  POINT      (2, 2)
            #*qux:   STRINT      \"hi there *screams*\"
            #*nand: LOGICFUNC   nand
            
            # comment
            *gate0: Center +NUM0:bar +NUM1:foo
            
            *gate1: Highlight +NUM2:bar +NUM32:bar
            
            #    + NUM0:name   # another comment
            #    + POINT0:foo +POINT0:h
            
            #*gate0 +n0:23";
            
        let dslin = "
            
            *panel_tl: POINT (10, 10)
            *panel_br: POINT (40, 40)
            
            *shape_tl: POINT (0, 0)
            *shape_br: POINT (10, 10)
            
            *btn_tl: POINT (0, 0)
            *btn_br: POINT (10, 10)

            *gate0: Center +POINT0:panel_tl +POINT1:panel_br +POINT2:shape_tl +POINT3:shape_br  +POINT4:btn_tl  +POINT5:btn_br
            
            ";

        assert_eq!(dsl::parse_numbered_port("NUM23"), Some((SpaghettiLinkType::F64, 23 as PortId)));
        assert_eq!(dsl::parse_numbered_port("POINT1"), Some((SpaghettiLinkType::Point, 1 as PortId)));
        
        match dsl::make_spaghetti(dslin) {
            Err(uwu) => println!("{}", dsl::make_err_msg(dslin, uwu.0, &uwu.1)),
            Ok(linkapp) => println!("cool: ")
        }
        
        
        
    
    }

    

}
