pub mod stupidlgrn;
pub mod dsl;

use logos::Logos;
use stupidlgrn::*;

// Fundemental Element and Link stuff

type ElemAnyId      = usize;
type ElemLocalId    = usize;
type ElemTypeId     = usize;
type LinkId         = usize;
pub type PortId         = usize;

#[derive(Clone, Default, Debug)]
pub struct PerElemType {
    pub free_elem_locals:   Vec<ElemLocalId>,
    pub elem_local_to_any:  Vec<ElemAnyId>,
    
    // for updating (shouldn't be here but i'm lazy)
    pub local_dirty:        BitVec
}

#[derive(Default, Debug)]
pub struct Elements {
    pub free_elem_any:      Vec<ElemAnyId>,
    pub elem_any_types:     Vec<(ElemTypeId, ElemLocalId)>,
    pub elem_type_data:     Vec<PerElemType>
}

#[derive(Default, Debug)]
pub struct PerLinkType
{
    pub free_links:             Vec<LinkId>,
    pub link_to_elems:          Vec< Vec<(ElemTypeId, ElemLocalId)> >,
    pub elem_to_port_to_link:   Vec< Vec<LinkId> >
}

#[derive(Default, Debug)]
pub struct LinkValues<T> {
    pub link_values:        Vec<T>,
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct LinkNewValue<T> {
    pub new_value:  T,
    pub sender:     ElemAnyId
}

//-----------------------------------------------------------------------------


#[derive(Default, Debug)]
pub struct ElementUpdater {
    
}

#[derive(Default, Debug)]
pub struct LinkUpdater<T: PartialEq> {
    pub link_values_next:   Vec< LinkNewValue<T> >,
    pub link_dirty:         BitVec
}

impl<T: PartialEq> LinkUpdater<T> {
    fn write(&mut self, link: LinkId, new_value: T, sender: ElemAnyId) -> bool {
        let foo = LinkNewValue{new_value, sender};
        if self.link_values_next[link] != foo {
            bitvec_set(&mut self.link_dirty, link);
            self.link_values_next[link] = foo;
            return true;
        }
        return false;
    }
}

//-----------------------------------------------------------------------------

// Custom values

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LogicFunc {
    And, Or, Xor, Xor2
}

impl Default for LogicFunc {
    fn default() -> Self { LogicFunc::And }
}

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Point { // actually the 133769th point class in the history of all programming
    pub x: f64,
    pub y: f64
}

//-----------------------------------------------------------------------------

// Spaghetti

#[derive(Debug, PartialEq, Eq)]
pub enum SpaghettiLinkType
{
    Bool,
    F64,
    Point,
    Logic,
    Str
}

#[derive(Default, Debug)]
pub struct SpaghettiLink<T: Clone+Default+PartialEq> {
    pub connections:    PerLinkType, 
    pub values:         LinkValues<T>,
    pub updater:        LinkUpdater<T>
}

impl<T: Clone+Default+PartialEq> SpaghettiLink<T> {
    fn resize_updaters(&mut self) {
        self.updater.link_dirty.resize(self.connections.link_to_elems.len()/64 + 1, 0);
        self.updater.link_values_next.resize(self.connections.link_to_elems.len(), Default::default());
    }
    
    fn mark_all_dirty(&mut self) {
        for n in 0..self.updater.link_values_next.len() {
            bitvec_set(&mut self.updater.link_dirty, n);
            self.updater.link_values_next[n].sender = 133769;
            self.updater.link_values_next[n].new_value = self.values.link_values[n].clone();
        }
    }
}


#[derive(Default, Debug)]
pub struct SpaghettiLinkGraph {
    pub elements:       Elements,
    pub values_bool:    SpaghettiLink<bool>,
    pub values_f64:     SpaghettiLink<f64>,
    pub values_point:   SpaghettiLink<Point>,
    pub values_logic:   SpaghettiLink<LogicFunc>,
    pub values_string:  SpaghettiLink<String>
}

impl SpaghettiLinkGraph {
    fn connections(&mut self, select: SpaghettiLinkType) -> &mut PerLinkType {
        match select {
            SpaghettiLinkType::Bool     => &mut self.values_bool    .connections,
            SpaghettiLinkType::F64      => &mut self.values_f64     .connections,
            SpaghettiLinkType::Point    => &mut self.values_point   .connections,
            SpaghettiLinkType::Logic    => &mut self.values_logic   .connections,
            SpaghettiLinkType::Str      => &mut self.values_string  .connections
        }
    }
    
    pub fn resize_updaters(&mut self) {
        for elem_data in &mut self.elements.elem_type_data {
            elem_data.local_dirty.resize(elem_data.elem_local_to_any.len()/64 + 1, 0);
        }
        
        self.values_bool    .resize_updaters();
        self.values_f64     .resize_updaters();
        self.values_point   .resize_updaters();
        self.values_logic   .resize_updaters();
        self.values_string  .resize_updaters();
    }
    pub fn mark_all_dirty(&mut self) {
        self.values_bool    .mark_all_dirty();
        self.values_f64     .mark_all_dirty();
        self.values_point   .mark_all_dirty();
        self.values_logic   .mark_all_dirty();
        self.values_string  .mark_all_dirty();
    }
    
    pub fn update_until_stable(&mut self, max_steps: u32) {
        
        let mut steps = 0;
        
        while steps < max_steps {
            
            update_links(&mut self.values_bool,    &mut self.elements);
            update_links(&mut self.values_f64,     &mut self.elements);
            update_links(&mut self.values_point,   &mut self.elements);
            update_links(&mut self.values_logic,   &mut self.elements);
            update_links(&mut self.values_string,  &mut self.elements);
            
            update_elem_center(self, 1);
            
            steps += 1;
        }
    }
}

//-----------------------------------------------------------------------------



pub fn update_links<T: Clone+Default+PartialEq>(values: &mut SpaghettiLink<T>, elements: &mut Elements) {
    for link_id in bitslice_it(&values.updater.link_dirty) {
    
        let next: &LinkNewValue<T> = &values.updater.link_values_next[link_id];
        values.values.link_values[link_id] = next.new_value.clone();
        
        for &(elem_type, elem_local) in values.connections.link_to_elems[link_id].iter() {
            let type_data = &mut elements.elem_type_data[elem_type];
            
            if type_data.elem_local_to_any[elem_local] == next.sender {
                continue;
            }
            
            bitvec_set(&mut type_data.local_dirty, elem_local);
            
        }
        
    }
    bitvec_clear(&mut values.updater.link_dirty);
}

pub fn update_elem_center(graph: &mut SpaghettiLinkGraph, elem_type: ElemTypeId) -> bool {

    let mut changed = false;
    let elem_data = &mut graph.elements.elem_type_data[elem_type];
    for elem_local in bitslice_it(&elem_data.local_dirty) {
        
        let elem_any: ElemAnyId = elem_data.elem_local_to_any[elem_local];
    
        let Some(ports) = graph.values_point.connections.elem_to_port_to_link.get(elem_any) else {
            continue;
        };
        let Some(&link_tl_in0) = ports.get(0) else { continue; };
        let Some(&link_br_in0) = ports.get(1) else { continue; };
        let Some(&link_tl_in1) = ports.get(0) else { continue; };
        let Some(&link_br_in1) = ports.get(1) else { continue; };
        let Some(&link_tl_out) = ports.get(0) else { continue; };
        let Some(&link_br_out) = ports.get(1) else { continue; };
        
        let point_tl_in0: &Point = &graph.values_point.values.link_values[link_tl_in0];
        let point_br_in0: &Point = &graph.values_point.values.link_values[link_br_in0];
        
        let point_tl_in1: &Point = &graph.values_point.values.link_values[link_tl_in1];
        let point_br_in1: &Point = &graph.values_point.values.link_values[link_br_in1];
        
        let dist_x: f64 = (point_tl_in0.x+point_br_in0.x)*0.5 - (point_tl_in1.x+point_br_in1.x)*0.5;
        let dist_y: f64 = (point_tl_in0.y+point_br_in0.y)*0.5 - (point_tl_in1.y+point_br_in1.y)*0.5;
        
        let point_tl_out = Point{
            x: point_tl_in1.x + dist_x,
            y: point_tl_in1.y + dist_y
        };

        let point_br_out = Point{
            x: point_br_in1.x + dist_x,
            y: point_br_in1.y + dist_y
        };
        
        println!("HAHAHAHAH");

        changed |= graph.values_point.updater.write(link_tl_out, point_tl_out, elem_any);
        changed |= graph.values_point.updater.write(link_br_out, point_br_out, elem_any);

    }
    bitvec_clear(&mut elem_data.local_dirty);
    return changed;
}


