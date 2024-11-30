use crate::{bit::Bit, gate::Gate};

const MSB_MASK: u64 = 1 << 63;
const IDX_MASK: u64 = !MSB_MASK; // first bit zero other ones

/// A circuit that consists of binary boolean gates connected to each other.
/// It is represented as a binary tree in a vec for compact representation.
/// Example circuit representation:
///
///         [Gate A]                   [Gate B]
///          /    \                     /    \
///     [In 1]  [In 2]            [Gate C]  [In 3]
///                               /      \
///                           [In 1]    [In 3]
/// Each would be a circuit tree.
struct Circuit {
    // The gates in the circuit, which will be accessed by index later
    pub gates: Vec<Gate<2>>,
    // Inputs to the circuit
    pub input: Vec<Bit>,
    // The circuit trees, root of each represent the output
    pub circuit_trees: Vec<CircuitTree>,
    // Memoized evaluations of each gate, length must be same as `gates`
    pub gate_evals: Vec<Option<Bit>>, // TODO: think about parallel access/evaluation
}

impl Circuit {
    // Evaluate the circuit on given input
    pub fn eval(&mut self) -> Vec<Bit> {
        let mut evals = std::mem::take(&mut self.gate_evals);
        let output = self
            .circuit_trees
            .iter()
            .map(|t| self.eval_tree(0, t, &mut evals)) // Get the value of the root gate/node
            .collect();
        self.gate_evals = evals;
        output
    }

    pub fn eval_tree(
        &self,
        node_idx: u64,
        tree: &CircuitTree,
        evals: &mut Vec<Option<Bit>>,
    ) -> Bit {
        // Evaluate the children and run them on the root gate
        match tree.get(node_idx) {
            (NodeType::Input, iidx) => self.input[iidx as usize],
            (NodeType::Gate, gidx) => {
                if evals[gidx as usize].is_some() {
                    return evals[0].unwrap();
                }
                let gate = &self.gates[gidx as usize];
                let left_val = self.eval_tree(node_idx * 2 + 1, tree, evals);
                let right_val = self.eval_tree(node_idx * 2 + 2, tree, evals);
                let res = gate.evaluate(&[left_val, right_val]);
                evals[gidx as usize] = Some(res);
                res
            }
        }
    }
}

/// An Array representations of a binary tree where each item can either be a gate or input
/// Since it represents two kinds of items, the first msb bit will be used to indicate the type and
/// the rest represent the index of the item in some other indexed data structure.
/// The root of the tree is at the 0th index
struct CircuitTree {
    inner: Vec<u64>,
}

impl CircuitTree {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Push gate index
    pub fn push_gate_idx(&mut self, gidx: u64) {
        assert!(gidx < 1 << 62); // The index should be less than 2^63
        self.inner.push(gidx); // since gate is prefixed with 0, no need to do anything
    }

    /// Insert input index
    pub fn push_input_idx(&mut self, iidx: u64) {
        assert!(iidx < 1 << 62); // The index should be less than 2^63
        let val = 1 << 63 | iidx; // Add bit 1 as msb
        self.inner.push(val); // since gate is prefixed with 0, no need to do anything
    }

    // Get the type and index, will panic if out of bound
    pub fn get(&self, idx: u64) -> (NodeType, u64) {
        let val = self.inner[idx as usize];
        let msb = val & MSB_MASK;
        let actual_idx = val & IDX_MASK;
        (msb.into(), actual_idx)
    }
}

pub enum NodeType {
    Gate,  // corresponds to 0
    Input, // corresponds to 1
}

impl From<u64> for NodeType {
    fn from(value: u64) -> Self {
        match value {
            0 => NodeType::Gate,
            _ => NodeType::Input,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::gate::{ANDGATE, ORGATE, XORGATE};

    use super::*;

    #[test]
    fn test_circuit_eval() {
        // Create some input bits
        let input_bits = vec![
            Bit::One,  // Input 1: true
            Bit::Zero, // Input 2: false
            Bit::One,  // Input 3: true
            Bit::Zero, // Input 4: false
        ];

        // Create gates (assuming Gate<2> takes a function and two inputs)
        let gate1 = ANDGATE; // AND gate
        let gate2 = ORGATE; // OR gate
        let gate3 = XORGATE; // XOR gate

        // Add gates to the circuit
        let gates = vec![gate1, gate2, gate3];

        // Create the circuit tree
        let mut circuit_tree = CircuitTree::new();
        // Tree structure:
        //        Gate 1 (AND)
        //        /      \
        //   Gate 2      Gate 3
        //   (OR)         (XOR)
        //   / \          /  \
        // In1 In2     In3  In4

        // Add nodes to the tree
        circuit_tree.push_gate_idx(0); // Gate 1
        circuit_tree.push_gate_idx(1); // Gate 2 (left child of Gate 1)
        circuit_tree.push_gate_idx(2); // Gate 3 (right child of Gate 1)
        circuit_tree.push_input_idx(0); // Input 1 (left child of Gate 2)
        circuit_tree.push_input_idx(1); // Input 2 (right child of Gate 2)
        circuit_tree.push_input_idx(2); // Input 3 (left child of Gate 3)
        circuit_tree.push_input_idx(3); // Input 4 (right child of Gate 3)
        let circuit_trees = vec![circuit_tree];
        let num_gates = gates.len();

        // Initialize the circuit
        let mut circuit = Circuit {
            gates,
            input: input_bits,
            circuit_trees,
            gate_evals: (0..num_gates).map(|_| None).collect(),
        };

        // Evaluate the circuit
        let result = circuit.eval();

        // Expected result:
        // - Gate 2 (OR): Input 1 || Input 2 = true || false = true
        // - Gate 3 (XOR): Input 3 ^ Input 4 = true ^ false = true
        // - Gate 1 (AND): Gate 2 && Gate 3 = true && true = true
        assert_eq!(result, vec![Bit::One]);
    }
}
