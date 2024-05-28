use std::io::Read as _;

fn main() {
    let Some(file) = std::env::args().nth(1) else {
        println!("brainfuck FILE");
        return;
    };
    execute(std::fs::read(file).unwrap());
}

const NONE: usize = usize::MAX;

fn execute(instructions: impl AsRef<[u8]>) {
    let instructions = instructions.as_ref();
    let mut current_inst = 0;
    let mut cells = vec![(0u8, NONE, NONE)]; // vec![(value, prev, next)]
    let mut current_cell = 0;

    while let Some(inst) = instructions.get(current_inst) {
        match inst {
            b'+' => cells[current_cell].0 += 1,
            b'-' => cells[current_cell].0 -= 1,
            b'.' => print!("{}", cells[current_cell].0 as char),
            b',' => {
                let mut input = [0u8];
                let _ = std::io::stdin().read_exact(&mut input);
                cells[current_cell].0 = input[0];
            }
            b'<' => {
                if cells[current_cell].1 == NONE {
                    cells.push((0, NONE, current_cell));
                    cells[current_cell].1 = cells.len()-1;
                };
                current_cell = cells[current_cell].1;
            }
            b'>' => {
                if cells[current_cell].2 == NONE {
                    cells.push((0, current_cell, NONE));
                    cells[current_cell].2 = cells.len()-1;
                };
                current_cell = cells[current_cell].2;
            }
            b'[' if cells[current_cell].0 == 0 => {
                let mut brackets = 1;
                while brackets > 0 {
                    current_inst += 1;
                    let current_inst = instructions[current_inst];
                    brackets += usize::from(current_inst == b'[') - usize::from(current_inst == b']');
                }
            }
            b']' if cells[current_cell].0 != 0 => {
                let mut brackets = 1;
                while brackets > 0 {
                    current_inst -= 1;
                    let current_inst = instructions[current_inst];
                    brackets += usize::from(current_inst == b']') - usize::from(current_inst == b'[');
                }
            }
            
            _ => {}
        }
        current_inst += 1;
    }
}
