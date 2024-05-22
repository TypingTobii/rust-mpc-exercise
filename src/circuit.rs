
#[derive(Debug)]
pub enum Gate {
    // Currently, EQ, EQW, and MAND gates are not yet implemented
    // Each gate has one field for each input and each output, denoting the wire connected to the port, respectively
    XOR {
        input_a: u32,
        input_b: u32,
        output: u32
    },
    AND {
        input_a: u32,
        input_b: u32,
        output: u32
    },
    INV {
        input: u32,
        output: u32
    }
}


#[derive(Debug)]
pub struct Header {
    // Header information of a bristol circuit

    // Total number of gates and total number of wires in the circuit
    num_gates: u32,
    num_wires: u32,

    // Number of wires for each input/output port, with the i-th entry in the Vec corresponding to the i-th port
    // Number of input/output ports may be inferred from the length of the Vecs
    num_input_wires: Vec<u32>,
    num_output_wires: Vec<u32>
}

#[derive(Debug)]
pub struct Circuit {
    // a circuit consists of a header and the gates of a circuit
    header: Header,
    gates: Vec<Gate>
}

impl Circuit {
    /// Parses the bristol file contents into a circuit
    pub fn parse(circuit: &str) -> Self {
        // Collect all non-empty lines of the str input into a Vec
        let circuit : Vec<&str> = circuit.lines().filter(|line| line.len() > 0).collect();

        let header = parse_header(&circuit[0..3]);

        let mut gates : Vec<Gate> = Vec::new();
        for line in &circuit[3..] {
            gates.push(parse_gate(line));
        }

        Circuit {header, gates}
    }
}

/// Parses the bristol file header, expecting to get the first three lines as an argument
fn parse_header(header_lines: &[&str]) -> Header {

    let (num_gates, num_wires) = parse_header_general(header_lines[0]);
    let num_input_wires = parse_header_io_wires(header_lines[1]);
    let num_output_wires = parse_header_io_wires(header_lines[2]);

    Header {
        num_gates,
        num_wires,
        num_input_wires,
        num_output_wires
    }
}

/// Parses the first line of the bristol file header containing the total number of gates and the total number of wires
fn parse_header_general(header_line: &str) -> (u32, u32) {
    let header_line : Vec<&str> = header_line.split_whitespace().collect();
    (header_line[0].parse().unwrap(),
     header_line[1].parse().unwrap())
}

/// Parses the second/third line of the bristol file header containing the number of wires per input/output
fn parse_header_io_wires(header_line: &str) -> Vec<u32> {
    let header_line : Vec<&str> = header_line.split_whitespace().collect();
    let num_ports : usize = header_line[0].parse().unwrap();
    let mut num_wires : Vec<u32> = Vec::new();

    for i in 1..num_ports+1 {
        num_wires.push(header_line[i].parse().unwrap());
    }

    num_wires
}

/// Parses a line of the bristol file describing one gate
fn parse_gate(gate_line: &str) -> Gate {
    let gate_line : Vec<&str> = gate_line.split_whitespace().collect();

    match *gate_line.last().unwrap() {
        "XOR" => parse_gate_xor(&gate_line),
        "AND" => parse_gate_and(&gate_line),
        "INV" | "NOT" => parse_gate_inv(&gate_line),
        "EQ" | "EQW" | "MAND" => unimplemented!(),
        _ => panic!("Unknown gate type!")
    }
}

/// helper function to parse a XOR gate line
fn parse_gate_xor(gate_line_vec: &Vec<&str>) -> Gate {
    // ensure that the number of input and output wires in the gate_line_vec is correct
    assert_eq!(gate_line_vec[0], "2", "Number of input wires must be 2 for every XOR gate");
    assert_eq!(gate_line_vec[1], "1", "Number of output wires must be 1 for every gate");

    let input_a : u32 = gate_line_vec[2].parse().unwrap();
    let input_b : u32 = gate_line_vec[3].parse().unwrap();
    let output  : u32 = gate_line_vec[4].parse().unwrap();

    Gate::XOR {input_a, input_b, output}
}

/// helper function to parse a AND gate line
fn parse_gate_and(gate_line_vec: &Vec<&str>) -> Gate {
    // ensure that the number of input and output wires in the gate_line_vec is correct
    assert_eq!(gate_line_vec[0], "2", "Number of input wires must be 2 for every AND gate");
    assert_eq!(gate_line_vec[1], "1", "Number of output wires must be 1 for every gate");

    let input_a : u32 = gate_line_vec[2].parse().unwrap();
    let input_b : u32 = gate_line_vec[3].parse().unwrap();
    let output  : u32 = gate_line_vec[4].parse().unwrap();

    Gate::AND {input_a, input_b, output}
}

/// helper function to parse a NOT/INV gate line
fn parse_gate_inv(gate_line_vec: &Vec<&str>) -> Gate {
    // ensure that the number of input and output wires in the gate_line_vec is correct
    assert_eq!(gate_line_vec[0], "1", "Number of input wires must be 1 for every INV/NOT gate");
    assert_eq!(gate_line_vec[1], "1", "Number of output wires must be 1 for every gate");

    let input  : u32 = gate_line_vec[2].parse().unwrap();
    let output : u32 = gate_line_vec[3].parse().unwrap();

    Gate::INV {input, output}
}

// A `#[cfg(test)]` marks the following block as conditionally included only for test builds.
// cfg directives can achieve similar things as preprocessor directives in C/C++.
#[cfg(test)]
mod tests {

    // Functions marked with `#[test]` are automatically run when you execute `cargo test`.
    #[test]
    fn test() {
        todo!("Writing tests is good!")
    }

}
