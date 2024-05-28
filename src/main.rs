use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let mut stdout = std::io::stdout();
    let stdin = std::io::stdin();

    let Some(file) = std::env::args().nth(1) else {
        return writeln!(stdout, "brainfuck FILE");
    };
    execute(std::fs::read(file)?, stdout, stdin)
}

fn execute(
    instructions: impl AsRef<[u8]>,
    mut stdout: impl Write,
    mut stdin: impl Read,
) -> std::io::Result<()> {
    let instructions = instructions.as_ref();
    let mut current_inst = 0;
    let mut cells = std::collections::VecDeque::from_iter([0u8]);
    let mut current_cell = 0;

    while let Some(inst) = instructions.get(current_inst) {
        match inst {
            b'+' => cells[current_cell] += 1,
            b'-' => cells[current_cell] -= 1,
            b'.' => stdout.write_all(&[cells[current_cell]])?,
            b',' => {
                let mut input = [0u8];
                let _ = stdin.read(&mut input);
                cells[current_cell] = input[0];
            }
            b'<' => {
                if current_cell == 0 {
                    cells.push_front(0);
                } else {
                    current_cell -= 1;
                }
            }
            b'>' => {
                current_cell += 1;
                if current_cell == cells.len() {
                    cells.push_back(0);
                }
            }
            b'[' if cells[current_cell] == 0 => {
                let mut brackets = 1;
                while brackets > 0 {
                    current_inst += 1;
                    let inst = instructions[current_inst];
                    brackets += usize::from(inst == b'[') - usize::from(inst == b']');
                }
            }
            b']' if cells[current_cell] != 0 => {
                let mut brackets = 1;
                while brackets > 0 {
                    current_inst -= 1;
                    let inst = instructions[current_inst];
                    brackets += usize::from(inst == b']') - usize::from(inst == b'[');
                }
            }
            _ => {}
        }
        current_inst += 1;
    }
    Ok(())
}
