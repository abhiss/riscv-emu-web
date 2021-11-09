extern crate riscv_emu_rust;
extern crate serde;

use riscv_emu_rust::*;
use wasm_bindgen::prelude::*;

use riscv_emu_rust::default_terminal::DefaultTerminal;
use riscv_emu_rust::Emulator;

#[wasm_bindgen]
pub struct WasmLibRiscv {
	emulator: Emulator,
}

#[wasm_bindgen]
impl WasmLibRiscv {
	pub fn ExecuteInstruction(instr: u32) -> Option<String> {
		match execution_instruction(instr) {
			Ok(x) => {
				return Some(serde_json::to_string(&x).unwrap());

			}, 
			Err(_) => return None
		}
	}
}

fn execution_instruction(instr: u32) -> Result<Vec<i64>, cpu::Trap> {
	let mut return_data: Vec<i64> = Vec::new();
	use mmu::DRAM_BASE;

	let mut cpu = cpu::Cpu::new(Box::new(DefaultTerminal::new()));

	cpu.get_mut_mmu().init_memory(4);
	cpu.update_pc(DRAM_BASE);

	// write non-compressed "addi a0, a0, 12" instruction
	match cpu.get_mut_mmu().store_word(DRAM_BASE, instr) {
		Ok(()) => {}
		Err(_e) => return Err(_e),
	};

	assert_eq!(DRAM_BASE, cpu.read_pc());
	assert_eq!(0, cpu.read_register(10));
	cpu.tick();

	for i in 0..31 {
		return_data.push(cpu.read_register(i).to_owned());
	}

	Ok(return_data)
}

#[cfg(test)]
mod tests {
	use crate::*;
	use core::panic;

	use riscv_emu_rust::*;

	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}

	#[test]
	fn test_1() {
		use mmu::DRAM_BASE;

		let mut cpu = cpu::Cpu::new(Box::new(DefaultTerminal::new()));

		cpu.get_mut_mmu().init_memory(4);
		cpu.update_pc(DRAM_BASE);

		// write non-compressed "addi a0, a0, 12" instruction
		match cpu.get_mut_mmu().store_word(DRAM_BASE, 0xc50513) {
			Ok(()) => {}
			Err(_e) => panic!("Failed to store"),
		};

		println!("\n");

		assert_eq!(DRAM_BASE, cpu.read_pc());
		assert_eq!(0, cpu.read_register(10));
		cpu.tick();

		// .tick() increments the program counter by 4 for
		// non-compressed instruction.
		assert_eq!(DRAM_BASE + 4, cpu.read_pc());
		// "addi a0, a0, a12" instruction writes 12 to a0 register.
		assert_eq!(12, cpu.read_register(10));

		for i in 0..31 {
			println!("Register x{}; Value: {};", i, &cpu.read_register(i));
		}
	}

	#[test]
	fn test_2() {
		match execution_instruction(0b110001010000010100010011) {
			Ok(x) => {
				for i in 0..x.len() {
					println!("Register x{}; Value: {};", i, x[i]);
				}
			}, 
			Err(trap) => panic!("An error occured")
		}
	}
}
