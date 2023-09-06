// src/main.rs

mod types;

use types::{Circuit, VoltageSource, Polarity, Resistor};

fn main() {
    
    // create a basic voltage source resistor circuit
    let mut circuit = Circuit::new();

    // add voltage source and resistor
    let voltage_source = VoltageSource::new("V1", 5.0, Polarity::Normal);
}
