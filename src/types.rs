// src/types.rs

// Circuits
pub struct Circuit {
    pub nodes: Vec<Node>,
    pub wires: Vec<Wire>,
    pub components: Vec<Component>,
}

// Connection Types
pub struct Node {
    pub id: usize,
    pub voltage: Option<f64>,
}

pub struct Wire {
    pub node1: *mut Node,
    pub node2: *mut Node,
}

// Component Types
pub enum Component {
    Resistor(Resistor),
    Capacitor(Capacitor),
    Inductor(Inductor),
    VoltageSource(VoltageSource),
    CurrentSource(CurrentSource),
}

pub struct BaseComponent {
    pub node1: *mut Node,
    pub node2: *mut Node,
    
    pub current: Option<f64>,
    pub voltage: Option<f64>,
}

pub struct Resistor {
    pub component: BaseComponent,
    pub resistance: f64,
}

pub struct Capacitor {
    pub component: BaseComponent,
    pub capacitance: f64,
}

pub struct Inductor {
    pub component: BaseComponent,
    pub inductance: f64,
}

pub struct VoltageSource {
    pub component: BaseComponent,
    pub voltage: f64,
    pub direction: bool, // if true, node1 should be plus and node2 should be minus
}

pub struct CurrentSource {
    pub component: BaseComponent,
    pub current: f64,
    pub direction: bool, // if true, current will flow from node2 to node1
}