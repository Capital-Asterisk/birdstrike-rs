use super::*;
use logos::Logos;
use std::collections::HashMap;


#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
pub enum Token {
    #[regex("\\#[^\\n]*")]
    Comment,
    
    #[regex("\\*[ \t]*[a-zA-Z0-9_]+[ \t]*:[ \t]*[a-zA-Z0-9_]+")]
    Item,
    
    #[regex("\\*[ \t]*[a-zA-Z0-9_]+[ \t]*")]
    ItemExtra,
    
    #[regex("\\+[ \t]*[a-zA-Z0-9_]+[ \t]*:[ \t]*[a-zA-Z0-9_]+")]
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



type ParserState<'a> =  &'a[(Token, usize, &'a str)];


type ParseResult<'a> = Result<ParserState<'a>, (usize, String)>;

#[derive(Default, Debug)]
struct SpaghettiBuilder {
    out: SpaghettiLinkGraph,
    
    elem_types: HashMap<&'static str, (ElemTypeId, u8)>,
    
    elem_name_to_id: HashMap<String, (ElemAnyId, ElemTypeId, ElemLocalId)>,
    link_name_to_id: HashMap<String, (SpaghettiLinkType, LinkId)>,
    
    ports: Vec< HashMap<&'static str, (SpaghettiLinkType, PortId)> >
}

fn parse_top<'a>(mut state: ParserState<'a>, builder: &mut SpaghettiBuilder) -> ParseResult<'a> {
    while let Some(((token, pos, slice), remaining_next)) = &state.split_first() {
        match token {
            Token::Item         => {state = parse_item(state, builder, false)?;},
            Token::ItemExtra    => {state = parse_item(state, builder, true)?;},
            _ => return Err((*pos, format!("Unexpected token type ({:?}) at top level: {}", token, slice)))
        }
    }

    Ok(state)
}

fn parse_item<'a>(mut state: ParserState<'a>, builder: &mut SpaghettiBuilder, extra: bool) -> ParseResult<'a> {

    let ((token, pos, slice), remaining_next) = state.split_first().unwrap();

    if extra {
        return Err((*pos, "ItemExtra is not yet implemented".to_string()));
    } else {
    
        assert_eq!(*token, Token::Item);
    
        let mut parts = slice.splitn(2, ':');
        let (name, itemtype) = (&parts.next().unwrap().trim()[1..], parts.next().unwrap().trim()); // what
        
        println!("item: ({},{})", name, itemtype);

        match itemtype {
            //"LogicGate" => { state = parse_element(remaining_next, builder, name, 0, *pos)?; },
            "NUM"       => { state = parse_link_num(remaining_next, builder, name, *pos)?; },
            "POINT"     => { state = parse_link_point(remaining_next, builder, name, *pos)?; },
            _ => if let Some((elem_type, _)) = builder.elem_types.get(itemtype) { 
                state = parse_element(remaining_next, builder, name, *elem_type, *pos)?;
            } else {
                return Err((*pos, format!("Unknown item type: {}", itemtype)))
            }
        }
    }

    Ok(state)
}

pub fn parse_numbered_port(port_spec: &str) -> Option<(SpaghettiLinkType, PortId)> {
    
    let Some(pos) = port_spec.chars().position(|c| c.is_ascii_digit()) else { return None; };

    let link_type: SpaghettiLinkType = match &port_spec[0..pos] {
        "BOOL"      => SpaghettiLinkType::Bool,
        "NUM"       => SpaghettiLinkType::F64,
        "POINT"     => SpaghettiLinkType::Point,
        "LOGICFUNC" => SpaghettiLinkType::Logic,
        "STRING"    => SpaghettiLinkType::Str,
        _ => return None
    };

    let Ok(port) = &port_spec[pos..].parse::<u16>() else { return None; };

    Some((link_type, *port as PortId))
}

fn parse_element<'a>(mut state: ParserState<'a>, builder: &mut SpaghettiBuilder, name: &str, elem_type: ElemTypeId, posPrev: usize) -> ParseResult<'a> {

    let type_data = &mut builder.out.elements.elem_type_data[elem_type];
    let elem_any:   ElemAnyId = builder.out.elements.elem_any_types.len();
    let elem_local: ElemLocalId = type_data.elem_local_to_any.len();
    
    builder.out.elements.elem_any_types.push((elem_type, elem_local));
    type_data.elem_local_to_any.push(elem_any);
    
    let elem_cap = builder.out.elements.elem_any_types.capacity();


    // parse connections
    while let Some(((token, pos, slice), remaining_next)) = state.split_first() {
        if *token != Token::Connection {
            return Ok(state);
        }
        
        let mut parts = slice[1..].splitn(2, ':');
        let (port_spec, link_name) = (&parts.next().unwrap().trim(), parts.next().unwrap().trim()); // what

        
        println!("connection: {}:{}", port_spec, link_name);
        
        let Some((link_type_var, link_id)) = builder.link_name_to_id.get(link_name) else {
            return Err((*pos, format!("Cannot find link: {}", link_name)));
        };
        
        let (link_type_port, port_id) = if let Some(out) = parse_numbered_port(port_spec) {
            out
        } else {// TODO: else if find named port
            return Err((*pos, format!("Cannot find port: {}", port_spec)));
        };
        
        if link_type_port != *link_type_var {
            return Err((*pos, format!("Incompatible link types! Port {} (type:{:?}) <--> Link {} (type:{:?})", port_spec, link_type_port, link_name, link_type_var)));
        }
        
        let connections: &mut PerLinkType = builder.out.connections(link_type_port);
        
        // Element->Link connection
        connections.elem_to_port_to_link.resize(elem_cap, Default::default());
        let ports: &mut Vec<LinkId> = &mut connections.elem_to_port_to_link[elem_any];
        if ports.len() <= port_id {
            ports.resize(port_id + 1, usize::MAX);
        }
        ports[port_id] = *link_id;
        
        // Link->Element connection
        let elems = &mut connections.link_to_elems[*link_id];
        if ! elems.contains(&(elem_type, elem_local)) {
            elems.push((elem_type, elem_local));
        }
        
        state = remaining_next;
    }
    
    Ok(state)
}


fn parse_link_num<'a>(mut state: ParserState<'a>, builder: &mut SpaghettiBuilder, name: &'a str, posPrev: usize) -> ParseResult<'a> {
    let ((token, pos, slice), remaining_next) = state.split_first().unwrap();
    
    let values = &mut builder.out.values_f64;
    let id: LinkId = values.connections.link_to_elems.len();
    values.connections.link_to_elems.push(vec![]);
    
    let None = builder.link_name_to_id.insert(name.to_string(), (SpaghettiLinkType::F64, id)) else {
        return Err((posPrev, format!("Duplicate name: {}", name)));
    };
    
    let value:f64 = if *token == Token::ValueNumber {
        let Ok(parsed) = slice.parse::<f64>() else {
            return Err((*pos, "can't parse double".to_string()));
        };
        state = remaining_next;
        parsed
    } else {
        0.0
    };
    
    values.values.link_values.push(value);
    
    println!("Number parsed: {}", value);
    
    Ok(state)
}

fn parse_link_point<'a>(state: ParserState<'a>, builder: &mut SpaghettiBuilder, name: &'a str, posPrev: usize) -> ParseResult<'a> {
    //let ((token, pos, slice), remaining_next) = state.split_first().unwrap();
    
    let values = &mut builder.out.values_point;
    let id: LinkId = values.connections.link_to_elems.len();
    values.connections.link_to_elems.push(vec![]);
    
    let None = builder.link_name_to_id.insert(name.to_string(), (SpaghettiLinkType::Point, id)) else {
        return Err((posPrev, format!("Duplicate name: {}", name)));
    };
    
    if state.len() < 5
    {
        return Err((state[0].1, "Incomplete point".to_string()));
    }
    
    if !(   state[0].0 == Token::BraceOpen
         && state[1].0 == Token::ValueNumber
         && state[2].0 == Token::Comma
         && state[3].0 == Token::ValueNumber
         && state[4].0 == Token::BraceClose) {
        return Err((state[0].1, "Malformed point".to_string()));
    }
    
    
    let Ok(point_x) = state[1].2.parse::<f64>() else {
        return Err((state[1].1, "can't parse double".to_string()));
    };
    
    let Ok(point_y) = state[3].2.parse::<f64>() else {
        return Err((state[3].1, "can't parse double".to_string()));
    };
    

    values.values.link_values.push(Point{x: point_x, y: point_y});
    
    println!("Point parsed: {}, {}", point_x, point_y);
    
    Ok(&state[5..])
}


pub fn make_err_msg(source: &str, err_offset: usize, msg: &str) -> String {
    let total_read: &str = &source[0..err_offset];
    let mut line_num: u32 = 0;
    let mut line_start: usize = 0;
    let mut line_remaining = total_read;
    
    while let Some(size) = line_remaining.find('\n') {
        line_num += 1;
        line_start += size+1;
        line_remaining = &line_remaining[(size+1)..];
    }
    
    let line_end = line_start +  &source[line_start..].find('\n').unwrap_or( source.len() - line_start);
    
    return format!("Error: \"{}\" see line {}:\n> {}\n{}^ HERE!\n", msg, line_num+1, &source[line_start..line_end], ".".repeat(1 + err_offset - line_start));
}


pub fn make_spaghetti(source: &str) -> Result<SpaghettiLinkGraph, (usize, String)> {

    let mut lexer = dsl::Token::lexer(source);
    let mut lexed: Vec<(Token, usize, &str)> = vec![];
    
    while let Some(result) = lexer.next() {
        let offset: usize = source.len()-lexer.remainder().len()-lexer.slice().len()+1;
        let Ok(tkn) = result else {
            return Err((offset, "Syntax error UvU".to_string()));
        };
        
        if tkn != Token::Comment {
            lexed.push((tkn, offset, lexer.slice().trim()));
        }
    }
    
    //println!("lexed: {:#?}", lexed);
    
    let mut builder = SpaghettiBuilder {
        out:                Default::default(),
        elem_types:         HashMap::from([
            ("Highlight",       (0, 0)),
            ("Center",          (1, 0)),
            ("Text",            (2, 0)),
            ("Button",          (3, 0)),
            ("RadioSelector",   (4, 0)),
        ]),
        elem_name_to_id:    Default::default(),
        link_name_to_id:    Default::default(),
        ports:              Default::default(),
    };
    
    builder.out.elements.elem_type_data.resize(20, Default::default());
    
    builder.ports.push(HashMap::from([
        ("a", (SpaghettiLinkType::F64, 31 as PortId))
    ]));
    
    parse_top(&lexed[0..], &mut builder)?;

    //println!("{:#?}", builder.out);
    
    //builder.out.resize_updaters();
    
    //builder.out.mark_all_dirty();
    //println!("BEFORE UPDATE {:#?}", builder.out);
    //builder.out.update_until_stable(10);
    //println!("AFTER UPDATE {:#?}", builder.out);
    Ok(builder.out)
}

