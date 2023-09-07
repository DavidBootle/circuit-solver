// src/types.rs

use std::collections::HashMap;

// Circuits
pub struct Circuit {
    pub nodes: Vec<Node>,
    pub wires: HashMap<usize, Wire>,
    pub components: HashMap<String, Box<dyn Component>>,
}
impl Circuit {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            wires: HashMap::new(),
            components: HashMap::new(),
        }
    }

    pub fn add_component(&mut self, mut component: impl Component + 'static) {
        // create new nodes for the component
        let node1 = self.new_node();
        let node2 = self.new_node();

        // add the nodes to the component
        component.component_mut().node1 = Some(node1);
        component.component_mut().node2 = Some(node2);

        // get the component's name
        let name = component.component().name.clone();

        // add the component to the circuit with it's id as the key
        self.components.insert(name, Box::new(component));
    }

    fn new_node(&mut self) -> usize {
        // create a new node and return a pointer to it
        let node_id = self.nodes.len();

        let node = Node {
            id: node_id,
            voltage: None,
        };

        self.nodes.push(node);

        node_id
    }

}

// Connection Types
pub struct Node {
    pub id: usize,
    pub voltage: Option<f64>,
}

pub struct Wire {
    pub node1: usize,
    pub node2: usize,
}

// Component Types
pub trait Component {
    fn component(&self) -> &BaseComponent;
    fn component_mut(&mut self) -> &mut BaseComponent;
}
pub trait DirectionalComponent {
    fn input_node(&self) -> Option<usize>;
    fn output_node(&self) -> Option<usize>;
}

pub struct BaseComponent {
    pub node1: Option<usize>,
    pub node2: Option<usize>,

    pub name: String,
    
    pub current: Option<f64>,
    pub voltage: Option<f64>,
}

pub struct Resistor {
    pub component: BaseComponent,
    pub resistance: f64,
}
impl Component for Resistor {
    fn component(&self) -> &BaseComponent { &self.component }
    fn component_mut(&mut self) -> &mut BaseComponent { &mut self.component }
}
impl Resistor {
    pub fn new(name: &str, resistance: f64) -> Self {
        Self {
            component: BaseComponent {
                node1: None,
                node2: None,
                name: name.to_string(),
                current: None,
                voltage: None,
            },
            resistance: resistance,
        }
    }
}

pub struct Capacitor {
    pub component: BaseComponent,
    pub capacitance: f64,
}
impl Component for Capacitor {
    fn component(&self) -> &BaseComponent { &self.component }
    fn component_mut(&mut self) -> &mut BaseComponent { &mut self.component }
}
impl Capacitor {
    pub fn new(name: &str, capacitance: f64) -> Self {
        Self {
            component: BaseComponent {
                node1: None,
                node2: None,
                name: name.to_string(),
                current: None,
                voltage: None,
            },
            capacitance: capacitance,
        }
    }
}

pub struct Inductor {
    pub component: BaseComponent,
    pub inductance: f64,
}
impl Component for Inductor {
    fn component(&self) -> &BaseComponent { &self.component }
    fn component_mut(&mut self) -> &mut BaseComponent { &mut self.component }
}
impl Inductor {
    pub fn new(name: &str, inductance: f64) -> Self {
        Self {
            component: BaseComponent {
                node1: None,
                node2: None,
                name: name.to_string(),
                current: None,
                voltage: None,
            },
            inductance: inductance,
        }
    }
}

pub struct VoltageSource {
    pub component: BaseComponent,
    pub voltage: f64,
    pub polarity: Polarity, // if normal, node1 should be plus and node2 should be minus
}
impl Component for VoltageSource {
    fn component(&self) -> &BaseComponent { &self.component }
    fn component_mut(&mut self) -> &mut BaseComponent { &mut self.component }
}
impl DirectionalComponent for VoltageSource {
    fn input_node(&self) -> Option<usize> {
        match self.polarity {
            Polarity::Normal => self.component.node1,
            Polarity::Inverted => self.component.node2,
        }
    }

    fn output_node(&self) -> Option<usize> {
        match self.polarity {
            Polarity::Normal => self.component.node2,
            Polarity::Inverted => self.component.node1
        }
    }
}
impl VoltageSource {
    pub fn new(name: &str, voltage: f64, polarity: Polarity) -> Self {
        Self {
            component: BaseComponent {
                node1: None,
                node2: None,
                name: name.to_string(),
                current: None,
                voltage: None,
            },
            voltage: voltage,
            polarity: polarity,
        }
    }
}

pub struct CurrentSource {
    pub component: BaseComponent,
    pub current: f64,
    pub polarity: Polarity, // if normal, current will flow from node2 to node1
}
impl DirectionalComponent for CurrentSource {
    fn input_node(&self) -> Option<usize> {
        match self.polarity {
            Polarity::Normal => self.component.node1,
            Polarity::Inverted => self.component.node2,
        }
    }

    fn output_node(&self) -> Option<usize> {
        match self.polarity {
            Polarity::Normal => self.component.node2,
            Polarity::Inverted => self.component.node1
        }
    }
}
impl Component for CurrentSource {
    fn component(&self) -> &BaseComponent { &self.component }
    fn component_mut(&mut self) -> &mut BaseComponent { &mut self.component }
}
impl CurrentSource {
    pub fn new(name: &str, current: f64, polarity: Polarity) -> Self {
        Self {
            component: BaseComponent {
                node1: None,
                node2: None,
                name: name.to_string(),
                current: None,
                voltage: None,
            },
            current: current,
            polarity: polarity,
        }
    }
}

pub enum Polarity {
    Normal,
    Inverted,
}