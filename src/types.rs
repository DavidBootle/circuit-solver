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
        self.components.insert(name.clone(), Box::new(component));

        // add the new connection to the nodes
        let connection = ConnectionItem::Component(name);
        self.get_node_mut(node1).unwrap().add_connection(connection.clone());
        self.get_node_mut(node2).unwrap().add_connection(connection);
    }

    pub fn get_component(&self, name: &str) -> Option<&dyn Component> {
        // get the component from the circuit
        let component = self.components.get(name);
        match component {
            Some(component) => Some(component.as_ref()),
            None => None,
        }
    }

    pub fn get_component_mut(&mut self, name: &str) -> Option<&mut dyn Component> {
        // get the component from the circuit
        let component = self.components.get_mut(name);
        match component {
            Some(component) => Some(component.as_mut()),
            None => None,
        }
    }

    pub fn get_node(&self, id: usize) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn get_node_mut(&mut self, id: usize) -> Option<&mut Node> {
        self.nodes.get_mut(id)
    }

    pub fn connect(&mut self, node1: usize, node2: usize) -> Result<Wire, &str> {
        // verify that the nodes exist
        if node1 >= self.nodes.len() || node2 >= self.nodes.len() {
            return Err("Node does not exist");
        }
        if node1 == node2 {
            return Err("Cannot connect a node to itself");
        }

        // create a new wire
        let wire_id = self.wires.len();
        let wire = Wire::new(wire_id, node1, node2);

        // add the new connection to the nodes
        let connection = ConnectionItem::Wire(wire_id);
        self.get_node_mut(node1).unwrap().add_connection(connection.clone());
        self.get_node_mut(node2).unwrap().add_connection(connection);

        return Ok(wire);
    }

    fn new_node(&mut self) -> usize {
        // create a new node and return a pointer to it
        let node_id = self.nodes.len();

        let node = Node::new(node_id);

        self.nodes.push(node);

        node_id
    }

}

// Connection Types
pub struct Node {
    pub id: usize,
    pub voltage: Option<f64>,
    pub connected: Vec<ConnectionItem>,
}
impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            id: id,
            voltage: None,
            connected: Vec::new(),
        }
    }

    pub fn add_connection(&mut self, connection: ConnectionItem) {
        self.connected.push(connection);
    }
}

#[derive(Clone)]
pub enum ConnectionItem {
    Wire(usize),
    Component(String),
}

pub struct Wire {
    pub node1: usize,
    pub node2: usize,
    pub id: usize,
}
impl Wire {
    pub fn new(id: usize, node1: usize, node2: usize) -> Self {
        Self {
            node1: node1,
            node2: node2,
            id: id,
        }
    }
}

// Component Types
pub trait Component {
    fn component(&self) -> &BaseComponent;
    fn component_mut(&mut self) -> &mut BaseComponent;
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

    pub fn positive_node(&self) -> Option<usize> {
        match self.polarity {
            Polarity::Normal => self.component.node1,
            Polarity::Inverted => self.component.node2,
        }
    }

    pub fn negative_node(&self) -> Option<usize> {
        match self.polarity {
            Polarity::Normal => self.component.node2,
            Polarity::Inverted => self.component.node1
        }
    }
}

pub struct CurrentSource {
    pub component: BaseComponent,
    pub current: f64,
    pub polarity: Polarity, // if normal, current will flow from node2 to node1
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

    pub fn input_node(&self) -> Option<usize> {
        match self.polarity {
            Polarity::Normal => self.component.node2,
            Polarity::Inverted => self.component.node1,
        }
    }

    pub fn output_node(&self) -> Option<usize> {
        match self.polarity {
            Polarity::Normal => self.component.node1,
            Polarity::Inverted => self.component.node2
        }
    }
}

pub enum Polarity {
    Normal,
    Inverted,
}