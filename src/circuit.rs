

// An enum is a good choice for a type which can be of either one of several variants.
// Since there is a fixed choice of gate types in a Bristol circuit, an enum is a natural
// way to represent it.
// A rust enum is similar to a tagged union in C/C++.
#[derive(Debug)]
pub enum Gate {
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


// We can 'derive' some traits like Debug and Clone on types via a derive attribute. This is a
// macro which expands to the corresponding trait implementation of the trait.
// cargo-expand (https://github.com/dtolnay/cargo-expand) can show you the expanded code.
#[derive(Debug)]
pub struct Header {
    // Header information of a bristol circuit
    num_gates: u32,
    num_wires: u32,
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
        // This method parses the circuit string representation into the Circuit type
        let circuit : Vec<&str> = circuit.lines().filter(|line| line.len() > 0).collect();
        let mut gates : Vec<Gate> = Vec::new();

        let header = parse_header(&circuit[0..3]);

        for line in &circuit[3..] {
            gates.push(parse_gate(line));
        }

        Circuit {header, gates}
    }
}

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


fn parse_header_general(header_line: &str) -> (u32, u32) {
    let header_line : Vec<&str> = header_line.split_whitespace().collect();
    (header_line[0].parse().unwrap(),
     header_line[1].parse().unwrap())
}

fn parse_header_io_wires(header_line: &str) -> Vec<u32> {
    let header_line : Vec<&str> = header_line.split_whitespace().collect();
    let num_ports : usize = header_line[0].parse().unwrap();
    let mut num_wires : Vec<u32> = Vec::new();

    for i in 1..num_ports+1 {
        num_wires.push(header_line[i].parse().unwrap());
    }

    num_wires
}

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

fn parse_gate_xor(gate_line_vec: &Vec<&str>) -> Gate {
    assert_eq!(gate_line_vec[0], "2", "Number of input wires must be 2 for every XOR gate");
    assert_eq!(gate_line_vec[1], "1", "Number of output wires must be 1 for every gate");

    let input_a : u32 = gate_line_vec[2].parse().unwrap();
    let input_b : u32 = gate_line_vec[3].parse().unwrap();
    let output  : u32 = gate_line_vec[4].parse().unwrap();

    Gate::XOR {input_a, input_b, output}
}

fn parse_gate_and(gate_line_vec: &Vec<&str>) -> Gate {
    assert_eq!(gate_line_vec[0], "2", "Number of input wires must be 2 for every AND gate");
    assert_eq!(gate_line_vec[1], "1", "Number of output wires must be 1 for every gate");

    let input_a : u32 = gate_line_vec[2].parse().unwrap();
    let input_b : u32 = gate_line_vec[3].parse().unwrap();
    let output  : u32 = gate_line_vec[4].parse().unwrap();

    Gate::AND {input_a, input_b, output}
}

fn parse_gate_inv(gate_line_vec: &Vec<&str>) -> Gate {
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
