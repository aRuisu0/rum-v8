use crate::instructs::*;
use std::env;
use std::io::Read;

// This public structure of the virtual machine will contain the registers and memory of the segments
// of the opcode instructions and will contain the counter for the program when machine is running
pub struct Vm {
	pub registers: Vec<u32>,
	pub memory: Vec<Vec<u32>>,
	pub unmapped_segs: Vec<usize>,
	pub max_mapped_seg: usize,
	pub prog_count: u32,
}

// Virtual machine that will start to boot and set memory and increment counter 
impl Vm {

	// constructs and initializes a new instance of the Vm struct. 
	// this allows to create new instances of the struct
	pub fn new_vm() -> Self {
		Vm {
			registers: vec![0; 8],
            memory: Vec::new(),
            unmapped_segs: Vec::new(),
            max_mapped_seg: 0,
            prog_count: 0,
		}
	}
	// Initliaze VM by taking argument
	pub fn boot(&mut self) {

		let args: Vec<String> = env::args().collect();
	    let input: Option<&str>;
	    
	    if args.len() == 2 {
	    	input = Some(&args[1]);
	    } else {
	    	input = None;
	    }
	    
	    	// Reused from Rumdump to read in all input data and defines a vector called instructions
			// and sets collection of u32 values and then pushes vector into memory field of the self object
	    	let mut raw_reader: Box<dyn std::io::BufRead> = match input {
			None => Box::new(std::io::BufReader::new(std::io::stdin())),
			Some(filename) => Box::new(std::io::BufReader::new(
				std::fs::File::open(filename).unwrap(),
			)),
		};
		
			let mut buf = Vec::<u8>::new();
			raw_reader.read_to_end(&mut buf).unwrap();
		
			let instructions: Vec<u32> = buf
				.chunks_exact(4)
				.map(|x| u32::from_be_bytes(x.try_into().unwrap()))
				.collect();
			self.memory.push(instructions); 
	}

	// Run instruction executbales
	pub fn run(&mut self) {
		loop {
			let instruction = self.get_instruct();
			self.execute(instruction);
		}
	}
	// Retrieve instruction counts
	fn get_instruct(&mut self) -> u32 {
		let instruction = self.memory[0][self.prog_count as usize];
		self.prog_count += 1;
		instruction
	}

	pub fn execute(&mut self, word: u32){

		// The >> operator shifts the bits of word to the right by 28 places, effectively moving the opcode bits to the rightmost position in the 
		// resulting value. The & operator then performs a bitwise AND operation with the value (1 << 4) - 1, 
		// which is a mask that has the first 4 bits set to 1 and the rest set to 0. This mask is used to isolate the first 4 bits of the word, which contain the opcode.
		// The resulting value is the opcode extracted from the word.
		let opcode = (word >> 28) & (1 << 4) - 1;

		// Excecute our Opcode conditions
		match opcode {
			0 =>  cond_move(self, word),
			1 =>  seg_load(self, word),
			2 =>  seg_store(self, word),
			3 =>  add(self, word),
			4 =>  mul(self, word),
			5 =>  div(self, word),
			6 =>  nand(self, word),
			7 =>  halt(self),
			8 =>  map_seg(self, word),
			9 =>  unmap_seg(self, word),
			10 => output(self, word),
			11 => input(self, word),
			12 => load_prog(self, word),
			13 => load_val(self, word),
			 _ => panic!("Error")

		};
	}
} 