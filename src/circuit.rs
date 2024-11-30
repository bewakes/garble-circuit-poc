use crate::{bit::Bit, gate::Gate};

/// A circuit that consists of binary boolean gates connected to each other.
/// Example circuit representation:
///
///         [Gate A]                   [Gate B]
///          /    \                     /    \
///     [In 1]  [In 2]            [Gate C]  [In 3]
///                               /      \
///                           [In 1]    [In 3]
/// Each would be a circuit tree.
#[derive(Clone, Debug)]
struct Circuit {
    // The gates in the circuit, which will be accessed by index later
    pub gates: Vec<Gate<2>>,
    // Inputs to the circuit
    pub input: Vec<Bit>,
    // Gate childs
    pub gate_childs: Vec<(Child, Child)>, // should have same length as gates
    // Outputs
    pub outputs: Vec<u64>, // output gate indices
    // Memoized evaluations of each gate, length must be same as `gates`
    pub gate_evals: Vec<Option<Bit>>, // TODO: think about parallel access/evaluation
}

#[derive(Clone, Debug)]
pub enum Child {
    Input(u64), // Input index
    Gate(u64),  // Gate index
}

impl Circuit {
    // Evaluate the circuit on given input
    pub fn eval(&mut self) -> Vec<Bit> {
        let mut evals = std::mem::take(&mut self.gate_evals);
        let output = self
            .outputs
            .iter()
            .map(|i| self.eval_gate(*i, &mut evals)) // Get the value of the root gate/node
            .collect();
        self.gate_evals = evals;
        output
    }

    pub fn eval_gate(&self, gate_idx: u64, evals: &mut Vec<Option<Bit>>) -> Bit {
        // Evaluate the children and run them on the root gate
        let idx = gate_idx as usize;
        if evals[idx].is_some() {
            return evals[idx].unwrap();
        }
        let (l, r) = &self.gate_childs[idx];
        let (lo, ro) = (self.eval_child(l, evals), self.eval_child(r, evals));
        let gate = &self.gates[idx];
        let o = gate.evaluate(&[lo, ro]);
        evals[idx] = Some(o);
        o
    }

    fn eval_child(&self, child: &Child, evals: &mut Vec<Option<Bit>>) -> Bit {
        match child {
            Child::Input(iidx) => self.input[*iidx as usize],
            Child::Gate(gidx) => self.eval_gate(*gidx, evals),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::gate::{ANDGATE, ORGATE, XORGATE};

    use super::*;

    #[test]
    fn test_unbalanced_circuit_eval() {
        // ASCII representation of the circuit:
        //
        //         [Gate AND (0)]                   [Gate OR (1)]
        //          /         \                      /      \
        //     [Input 1]  [Input 2]          [Gate XOR (2)] [Input 3]
        //                                       /      \
        //                                  [Input 1] [Input 3]

        // Add gates to the circuit
        let gates = vec![ANDGATE, ORGATE, XORGATE];

        // Define inputs
        let input_bits = vec![Bit::One, Bit::Zero, Bit::One];

        // Define gate-child relationships
        // gate_childs[i] represents the children of gates[i]
        let gate_childs = vec![
            (Child::Input(0), Child::Input(1)), // Gate AND (0): Input 1 && Input 2
            (Child::Gate(2), Child::Input(2)),  // Gate OR (1): Gate XOR || Input 3
            (Child::Input(0), Child::Input(2)), // Gate XOR (2): Input 1 ^ Input 3
        ];

        // Define output gates
        let outputs = vec![0, 1]; // Gate AND (0) and Gate OR (1)

        // Initialize the circuit
        let mut circuit = Circuit {
            gates,
            input: input_bits,
            gate_childs,
            outputs,
            gate_evals: vec![None; 3], // One slot for each gate
        };
        // Evaluate the circuit
        let result = circuit.eval();

        // Expected outputs:
        // - Gate AND (0): Input 1 && Input 2 = true && false = false
        // - Gate XOR (2): Input 1 ^ Input 3 = true ^ true = false
        // - Gate OR (1): Gate XOR || Input 3 = false || true = true
        assert_eq!(result, vec![Bit::Zero, Bit::One]);
    }
}
