use crate::rum::Vm;
use std::io;
use std::io::Read;

pub struct Field {
	width: u32,
	lsb: u32,
}

// These static values represent different fields in an instruction word, with RA, RB, and RC
// representing fields with 3 bits each and RL and VL representing fields with 3 and 25 bits, respectively.
pub static RA: Field = Field {width: 3, lsb: 6};
pub static RB: Field = Field {width: 3, lsb: 3};
pub static RC: Field = Field {width: 3, lsb: 0};
pub static RL: Field = Field {width: 3, lsb: 25};
pub static VL: Field = Field {width: 25, lsb: 0};

// extracts the value of a field from an instruction word. It does this by shifting the instruction 
// word to the right by the number of least significant bits specified by the lsb field of the Field struct, 
// and then masking the result
pub fn get(field: &Field, instruction: u32) -> usize {
	((instruction >> field.lsb) & mask(field.width)).try_into().unwrap()
}

// generates a mask for a given number of bits by shifting the value 1 to 
// the left by the given number of bits and then subtracting 1.
fn mask(bits: u32) -> u32 {
	(1 << bits) - 1
}

// Conditional Load Operator
pub fn cond_move(vm: &mut Vm, word: u32) {
	// Conditional Load
	let a = get(&RA, word);
	let b = get(&RB, word);
	let c = get(&RC, word);

	match vm.registers[c] {
		0 => {} // Do nothing if c is 0
		_ => vm.registers[a] = vm.registers[b], // Otherwise, set a to b
	}
}

// Segmented Load Operator
// This function is using indexing to access the value in the memory array at the indices specified by the b and c registers.
pub fn seg_load(vm: &mut Vm, word: u32) {
	// Segmented Load
	let a = get(&RA, word);
	let b = get(&RB, word);
	let c = get(&RC, word);
	vm.registers[a] = vm.memory[vm.registers[b] as usize][vm.registers[c] as usize];
}

// Segmented Store Operator
pub fn seg_store(vm: &mut Vm, word: u32) {
	let a = get(&RA, word);
	let b = get(&RB, word);
	let c = get(&RC, word);

	 // Use indexing to set the value in the memory array at the indices specified by the a and b registers.
	vm.memory[vm.registers[a] as usize][vm.registers[b] as usize] = vm.registers[c];
}

// Add Operator  
//This function adds the values stored in the bth and cth elements of the vm object's 
//registers array and stores the result in the ath element of the array.
pub fn add(vm: &mut Vm, word: u32) {
	let a = get(&RA, word);
	let b = get(&RB, word);
	let c = get(&RC, word);
	vm.registers[a] = ((vm.registers[b] as u64 + vm.registers[c] as u64) % (1_u64 << 32)).try_into().unwrap();
}

// Multiply Operator
//This function multiplys the values stored in the bth and cth elements of the vm object's 
//registers array and stores the result in the ath element of the array.
pub fn mul(vm: &mut Vm, word: u32) {
	let a = get(&RA, word);
	let b = get(&RB, word);
	let c = get(&RC, word);
	vm.registers[a] = ((vm.registers[b] as u64 * vm.registers[c] as u64) % (1_u64 << 32)).try_into().unwrap();
}

// Divide Operator
// this code performs a division operation on the values 
// stored in two of the virtual machine's registers, storing the result in a third register.
pub fn div(vm: &mut Vm, word: u32) {
	let a = get(&RA, word);
	let b = get(&RB, word);
	let c = get(&RC, word);
	vm.registers[a] = vm.registers[b] / vm.registers[c];	
}

// Bitwise NAND Operator
// Takes a vm object and an integer as arguments, extracts the values
// of three registers from the integer, performs a bitwise AND 
// operation on two of the register values, negates the result, and stores it in the third register.
pub fn nand(vm: &mut Vm, word: u32) {
	let a = get(&RA, word);
	let b = get(&RB, word);
	let c = get(&RC, word);
	vm.registers[a] = !(vm.registers[b] & vm.registers[c]);	
}

// Halt Operator
pub fn halt(_vm: &mut Vm) {
	std::process::exit(0);
}

// Map Segment Operator
// Function for managing the allocation of memory segments in a 
// virtual machine. It allows for the creation of new segments or the re-use of previously unmapped segments
pub fn map_seg(vm: &mut Vm, word: u32) {
	let b = get(&RB, word);
	let c = get(&RC, word);
	if vm.unmapped_segs.len() != 0 {
		let segment_number = vm.unmapped_segs.pop().unwrap();
		vm.memory[segment_number] = vec![0; vm.registers[c] as usize];
		vm.registers[b] = segment_number as u32;
	} 
	else {
		vm.max_mapped_seg += 1;
		vm.memory.push(vec![0; vm.registers[c] as usize]);
		vm.registers[b] = vm.max_mapped_seg as u32;
	}
}

// Unmap Segment Operator
// Used for managing the memory segments in a 
// virtual machine for unmapping or removing a memory segment fromm memory
pub fn unmap_seg(vm: &mut Vm, word: u32) {
	let c = get(&RC, word);

	vm.memory[vm.registers[c] as usize].clear();
	vm.unmapped_segs.push(vm.registers[c].try_into().unwrap());
}

// Output Operator
//Uses the word to extract a single register value 
//from the vm's registers array and prints 
//the char representation of the register
pub fn output(vm: &mut Vm, word: u32) {
	let c = get(&RC, word);
	print!("{}", vm.registers[c] as u8 as char);
}

//Input Operator
//The function assigns the value of value to the c register of the vm struct
pub fn input(vm: &mut Vm, word: u32) {
	let c = get(&RC, word);
    let mut buffer: [u8; 1] = [0; 1];
	let num = io::stdin().read(&mut buffer);
    let value = match num {
        Ok(byte) => byte as u32,
        Err(_) => !0_u32
    };
	vm.registers[c] = value;
}

// Load Program Operator
pub fn load_prog(vm: &mut Vm, word: u32) {
	let b = get(&RB, word);
	let c = get(&RC, word);


	if vm.registers[b] != 0 {
		vm.memory[0] = vm.memory[vm.registers[b] as usize].clone();
    }
	vm.prog_count = vm.registers[c];
}

// Load Value
pub fn load_val(vm: &mut Vm, word: u32) {
	let value = get(&VL, word);
	let a = get(&RL, word);
	vm.registers[a as usize] = value as u32;
}