// src/main.rs

mod types;

use types::{Circuit, VoltageSource, Polarity, Resistor};

fn main() {
    
    // create a basic voltage source resistor circuit
    let mut circuit = Circuit::new();

    // add voltage source and resistor
    let voltage_source = VoltageSource::new("V1", 5.0, Polarity::Normal);
    let resistor: Resistor = Resistor::new("R1", 100.0);

    circuit.add_component(voltage_source);
    circuit.add_component(resistor);

    // connect the two components
    let voltage_source_ref = circuit.get_component("V1").unwrap();
    let resistor_ref = circuit.get_component("R1").unwrap();
    match circuit.connect(voltage_source_ref.component().node1.unwrap(), resistor_ref.component().node2.unwrap()) {
        Ok(_) => (),
        Err(e) => println!("{}", e)
    }
}