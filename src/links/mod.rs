pub mod stupidlgrn;
pub mod dsl;

use stupidlgrn::*;

type ElemAnyId      = usize;
type ElemLocalId    = usize;
type ElemTypeId     = usize;
type LinkId         = usize;
type PortId         = usize;

#[derive(Default)]
pub struct PerElemType {
    free_elem_locals:   Vec<ElemLocalId>,
    elem_local_to_any:  Vec<ElemAnyId>
}

#[derive(Default)]
pub struct Elements {
    free_elem_any:      Vec<ElemAnyId>,
    elem_any_types:     Vec<(ElemTypeId, ElemLocalId)>,
    elem_type_data:     Vec<PerElemType>
}

#[derive(Default)]
pub struct PerLinkType
{
    free_links:         Vec<LinkId>,
    link_publ_to_elem:  Vec< Vec<(ElemTypeId, ElemLocalId)> >,
    elem_subs_to_link:  Vec< Vec<LinkId> >
}

#[derive(Default)]
pub struct LinkValues<T> {

    link_values:        Vec<T>,
}

#[derive(Default)]
pub struct LinkUpdater<T> {
    link_values_next:   Vec<T>,
    link_dirty:         BitVec
}

impl<T> LinkUpdater<T> {
    fn assign(&mut self, link: LinkId, value: T) {
        bitvec_set(&mut self.link_dirty, link);
        self.link_values_next[link] = value;
    }
}

//-----------------------------------------------------------------------------

pub enum LogicFunc {
    And, Or, Xor, Xor2
}

impl Default for LogicFunc {
    fn default() -> Self { LogicFunc::And }
}

#[derive(Default)]
pub struct Point { // actually the 133769th point class in the history of all programming
    x: f64,
    y: f64
}

//-----------------------------------------------------------------------------


#[derive(Default)]
pub struct SpaghettiLink<T> {
    values: LinkValues<T>,
    updater: LinkUpdater<T>
}

#[derive(Default)]
pub struct SpaghettiLinkApp {
    elements:       Elements,
    values_bool:    SpaghettiLink<bool>,
    values_f64:     SpaghettiLink<f64>,
    values_point:   SpaghettiLink<Point>,
    values_logic:   SpaghettiLink<LogicFunc>,
    values_string:  SpaghettiLink<String>
}




pub fn make_spaghetti(stuff: &str) -> SpaghettiLinkApp {
    let mut out:SpaghettiLinkApp = Default::default();
    
    
    
    
    
    
    out
}




