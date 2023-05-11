use crate::rumdis;
use std::io::{stdin, Read, Write, stdout};

//Object responsible for holding Virtual Machine related data
pub struct VM{
    registers: Vec<u32>,
    memory: Vec<Vec<u32>>,
    unmap_index_values: Vec<usize>,
    program_counter: usize
}

/// Performs a Conditional Move if $r[C] != 0
/// Modifies the a register in the VM object
/// 
/// # Arguments:
/// * um: A Virtual Machine object
/// * a: The a register
/// * b: The b register
/// * c: The c register
pub fn opcode0(um: &mut VM, a: usize, b: usize, c: usize){
    if um.registers[c] != 0{
        um.registers[a] = um.registers[b];
    }
}

/// Performs a Segmented Load
/// Modifies the a register in the VM object
/// 
/// # Arguments:
/// * um: A Virtual Machine object
/// * a: The a register
/// * b: The b register
/// * c: The c register
pub fn opcode1(um: &mut VM, a: usize, b: usize, c: usize){
    um.registers[a] = um.memory[um.registers[b] as usize][um.registers[c] as usize];
}

/// Performs a Segmented Store
/// Modifies the memory address at the $m[$r[a]][$r[b]] index
/// 
/// # Arguments:
/// * um: A Virtual Machine object
/// * a: The a register
/// * b: The b register
/// * c: The c register
pub fn opcode2(um: &mut VM, a: usize, b: usize, c: usize){
    um.memory[um.registers[a] as usize][um.registers[b] as usize] = um.registers[c];
}

/// Performs an Addition operation
/// Modifies the a register in the VM object
/// 
/// # Arguments:
/// * um: A Virtual Machine object
/// * a: The a register
/// * b: The b register
/// * c: The c register
pub fn opcode3(um: &mut VM, a: usize, b: usize, c: usize){
    um.registers[a] = um.registers[b].wrapping_add(um.registers[c]);
}

/// Performs a Multiplication operation
/// Modifies the a register in the VM object
/// 
/// # Arguments:
/// * um: A Virtual Machine object
/// * a: The a register
/// * b: The b register
/// * c: The c register
pub fn opcode4(um: &mut VM, a: usize, b: usize, c: usize){
    um.registers[a] = um.registers[b].wrapping_mul(um.registers[c]);
}

/// Performs integer division
/// Modifies the a register in the VM object
/// 
/// # Arguments:
/// * um: A Virtual Machine object
/// * a: The a register
/// * b: The b register
/// * c: The c register
pub fn opcode5(um: &mut VM, a: usize, b: usize, c: usize){
    //If a segmented load or segmented store refers to an unmapped segment, the machine may fail.
    if um.registers[c] == 0{
        panic!("Cannot divide by 0")
    }
    um.registers[a] = um.registers[b] / um.registers[c];
}

/// Performs Bitwise NAND
/// Modifies the a register in the VM object
/// 
/// # Arguments:
/// * um: A Virtual Machine object
/// * a: The a register
/// * b: The b register
/// * c: The c register
pub fn opcode6(um: &mut VM, a: usize, b: usize, c: usize){
    um.registers[a] = !(um.registers[b] & um.registers[c]);
}

/// Ends the program
pub fn opcode7(){
    std::process::exit(0);
}

/// Maps a segment
/// The new segment is mapped as $m[$r[b]]
/// 
/// # Arguments:
/// * um: A Virtual Machine object
/// * b: The b register
/// * c: The c register
pub fn opcode8(um: &mut VM, b: usize, c: usize){
    //A new segment is created with a number of words equal to the value in $r[C]
    //Each word in the new segment is initialized to zero
    let length = um.registers[c] as usize;
    let new_segment = vec![0_u32; length];

    //A bit pattern that is not all zeroes and does not identify any currently mapped segment is placed in $r[B].
    if um.unmap_index_values.len() != 0{
        um.registers[b] = (um.unmap_index_values.pop().unwrap()) as u32;

        //The new segment is mapped as $m[$r[B]].
        um.memory[um.registers[b] as usize] = new_segment;
    }else {
        //The new segment is mapped as $m[$r[B]].
        um.memory.push(new_segment.clone());
        um.registers[b] = (um.memory.len() - 1) as u32;
    }
}

/// Unmaps a segment
/// The segment $m[$r[c]] is unmapped
/// 
/// # Arguments:
/// * um: A Virtual Machine object
/// * c: The c register
pub fn opcode9(um: &mut VM, c: usize){
    //The segment $m[$r[C]] is unmapped.
    //If an instruction unmaps $m[0], or if it unmaps a segment that is not mapped, the machine may fail.
    if um.registers[c] as usize == 0{
        panic!("Instruction is trying to unmap $m[0]")
    }else{
        um.unmap_index_values.push(um.registers[c] as usize);
    }
    
}

/// Outputs a specified value
/// Only valid values to output between 0 and 255
/// 
/// # Arguments:
/// * um: A Virtual Machine object
/// * c: The c register
pub fn opcode10(um: &mut VM, c: usize){
    let value = u8::try_from(um.registers[c]).unwrap();
    let mut buffer = std::io::stdout();
    match buffer.write(&[value]).unwrap() {
        1 =>{
            stdout().flush().unwrap();
        },
        _ =>{
            panic!("Wrong output value")
        }
    }
}

/// Reads an input from standard in
/// When the input arrives, $r[c] is loaded with the input
/// 
/// # Arguments:
/// * um: A Virtual Machine object
/// * c: The c register
pub fn opcode11(um: &mut VM, c: usize){
    let mut input = [0_u8; 1];

    let mut number = stdin();

    um.registers[c] = match number.read(&mut input).expect("Failed to read line") {
        1 =>{
            input[0] as u32
        },
        _ => {
            u32::MAX
        }
    }
}

/// Performs the load program
/// Segment $m[$r[b]] is duplicated, and the duplicate replaces $m[0]
/// 
/// # Arguments:
/// * um: A Virtual Machine object
/// * b: The b register
/// * c: The c register
pub fn opcode12(um: &mut VM, b: usize, c: usize){
    //Program counter is set to point to $m[0][$r[c]]
    um.program_counter = um.registers[c] as usize;
    
    if um.registers[b] != 0{
        //duplicate memory segment at $m[$r[b]]
        let new_segment = um.memory[um.registers[b] as usize].clone();

        //replace and abandonds the $m[0] value with the new_segment value
        um.memory[0] = new_segment;
    }
}

/// Loads a value
/// 
/// # Arguments:
/// * um: A Virtual Machine object
/// * rl: The a register
/// * vl: The value
pub fn opcode13(um: &mut VM, rl: usize, vl: u32){
    um.registers[rl] = vl;
}

/// Handle the input of instructions
/// Is responsible for determining which instructions to execute
/// 
/// # Arguments:
/// * instructions: A vector containing 32-bit words which are instructions
pub fn handle_input(instructions: Vec<u32>){
    //initialize registers to 0
    let registers: Vec<u32> = vec![0; 8];

    //create program counter and initialize to 0,0
    let program_counter = 0;

    //2-d array for memory segments
    let mut memory: Vec<Vec<u32>> = vec![];
  
    memory.push(instructions.clone());


    let unmap_index_values: Vec<usize> = vec![];

    let mut um = VM{
        registers,
        memory,
        unmap_index_values,
        program_counter
    };

    //If at the beginning of a machine cycle the program counter points outside the bounds of $m[0], the machine may fail.
    if um.program_counter > 0{
        panic!("Program Counter outside the bounds of $m[0]")
    }

    //If at the beginning of a cycle, the word pointed to by the program counter does not code for a valid instruction, the machine may fail.
    if rumdis::get(&rumdis::OP, um.memory[0][um.program_counter]) > 13{
        panic!("Word being pointed to does not code for valid instructions")
    }

    loop{
        let instruction = um.memory[0][um.program_counter];

        //get the opcode
        let opcode = rumdis::get(&rumdis::OP, instruction);
        let a = (rumdis::get(&rumdis::RA, instruction)) as usize;
        let b = (rumdis::get(&rumdis::RB, instruction)) as usize;
        let c = (rumdis::get(&rumdis::RC, instruction)) as usize;
        //let rl = (rumdis::get(&rumdis::RL, instruction)) as usize;
        //let vl = rumdis::get(&rumdis::VL, instruction);
        um.program_counter += 1;

        if opcode == 0{
            if um.registers[c] != 0{
                um.registers[a] = um.registers[b];
            }
            //opcode0(&mut um, a, b, c);
        }
        if opcode == 1{
            um.registers[a] = um.memory[um.registers[b] as usize][um.registers[c] as usize];
            //opcode1(&mut um, a, b, c);
        }
        if opcode == 2{
            um.memory[um.registers[a] as usize][um.registers[b] as usize] = um.registers[c];
            //opcode2(&mut um, a, b, c);
        }
        if opcode == 3{
            um.registers[a] = um.registers[b].wrapping_add(um.registers[c]);
            //opcode3(&mut um, a, b, c);
        }
        if opcode == 4{
            um.registers[a] = um.registers[b].wrapping_mul(um.registers[c]);
            //opcode4(&mut um, a, b, c);
        }
        if opcode == 5{
            if um.registers[c] == 0{
                panic!("Cannot divide by 0")
            }
            um.registers[a] = um.registers[b] / um.registers[c];
            //opcode5(&mut um, a, b, c);
        }
        if opcode == 6{
            um.registers[a] = !(um.registers[b] & um.registers[c]);
            //opcode6(&mut um, a, b, c);
        }
        if opcode == 7{
            std::process::exit(0);
            //opcode7();
        }
        if opcode == 8{
            let length = um.registers[c] as usize;
            let new_segment = vec![0_u32; length];
        
            //A bit pattern that is not all zeroes and does not identify any currently mapped segment is placed in $r[B].
            if um.unmap_index_values.len() != 0{
                um.registers[b] = (um.unmap_index_values.pop().unwrap()) as u32;
        
                //The new segment is mapped as $m[$r[B]].
                um.memory[um.registers[b] as usize] = new_segment;
            }else {
                //The new segment is mapped as $m[$r[B]].
                //um.memory.push(new_segment.clone());
                um.memory.push(new_segment);
                um.registers[b] = (um.memory.len() - 1) as u32;
            }
            //opcode8(&mut um, b, c);
        }
        if opcode == 9{
            if um.registers[c] as usize == 0{
                panic!("Instruction is trying to unmap $m[0]")
            }else{
                um.unmap_index_values.push(um.registers[c] as usize);
            }
            //opcode9(&mut um, c);
        }
        if opcode == 10{
            let value = u8::try_from(um.registers[c]).unwrap();
            let mut buffer = std::io::stdout();
            match buffer.write(&[value]).unwrap() {
                1 =>{
                    stdout().flush().unwrap();
                },
                _ =>{
                    panic!("Wrong output value")
                }
            }
            //opcode10(&mut um, c);
        }
        if opcode == 11{
            let mut input = [0_u8; 1];

            let mut number = stdin();
        
            um.registers[c] = match number.read(&mut input).expect("Failed to read line") {
                1 =>{
                    input[0] as u32
                },
                _ => {
                    u32::MAX
                }
            }
            //opcode11(&mut um, c);
        }
        if opcode == 12{
            um.program_counter = um.registers[c] as usize;
    
            if um.registers[b] != 0{
                //duplicate memory segment at $m[$r[b]]
                //let new_segment = um.memory[um.registers[b] as usize].clone();
                let new_segment = &um.memory[um.registers[b] as usize];
        
                //replace and abandonds the $m[0] value with the new_segment value
                um.memory[0] = (new_segment).to_vec();
            }
            //opcode12(&mut um, b, c);
        }
        if opcode == 13{
            let rl = (rumdis::get(&rumdis::RL, instruction)) as usize;
            let vl = rumdis::get(&rumdis::VL, instruction);
            um.registers[rl] = vl;
            //opcode13(&mut um, rl, vl);
        }
    }
}
