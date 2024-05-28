use std::io::Read;

pub struct Brainfuck {
    instructions: Vec<u8>,
    current_inst: usize,
    data: Vec<Cell>,
    current_cell: usize,
}

pub struct Cell {
    value: u8,
    prev: usize,
    next: usize,
}

/// Represents the None value.
const NONE: usize = usize::MAX;

impl Brainfuck {
    pub fn new(instructions: impl Into<Vec<u8>>) -> Self {
        Brainfuck {
            instructions: instructions.into(),
            current_inst: 0,
            current_cell: 0,
            data: vec![Cell {
                value: 0,
                prev: NONE,
                next: NONE,
            }],
        }
    }

    /// Returns the current Cell's value.
    fn current(&self) -> u8 {
        self.data[self.current_cell].value
    }

    fn current_cell(&self) -> &Cell {
        &self.data[self.current_cell]
    }

    fn current_cell_mut(&mut self) -> &mut Cell {
        &mut self.data[self.current_cell]
    }

    fn current_inst(&self) -> u8 {
        self.instructions[self.current_inst]
    }

    /// Retrocedes to previous Cell.
    fn prev(&mut self) {
        if self.current_cell().prev != NONE {
            self.current_cell = self.current_cell().prev;
            return;
        }

        self.data.push(Cell {
            value: 0,
            prev: usize::MAX,
            next: self.current_cell,
        });
        let prev = self.data.len() - 1;
        self.current_cell_mut().prev = prev;
        self.current_cell = prev;
    }

    /// Advances to next Cell.
    fn next(&mut self) {
        if self.current_cell().next != NONE {
            self.current_cell = self.current_cell().next;
            return;
        }

        self.data.push(Cell {
            value: 0,
            prev: self.current_cell,
            next: NONE,
        });
        let next = self.data.len() - 1;
        self.current_cell_mut().next = next;
        self.current_cell = next;
    }

    pub fn execute(mut self) -> std::io::Result<()> {
        while let Some(inst) = self.instructions.get(self.current_inst) {
            match inst {
                b'<' => self.prev(),
                b'>' => self.next(),
                b'+' => self.current_cell_mut().value += 1,
                b'-' => self.current_cell_mut().value -= 1,
                b'.' => print!("{}", self.current() as char),
                b',' => 'read: {
                    let mut input: u8 = 0;
                    let Ok(_) = std::io::stdin().read_exact(std::slice::from_mut(&mut input))
                    else {
                        break 'read;
                    };
                    self.current_cell_mut().value = input;
                }
                b'[' => 'bracketopen: {
                    if self.current() != 0 {
                        break 'bracketopen;
                    }

                    let mut brackets = 1;
                    while brackets > 0 {
                        self.current_inst += 1;
                        match self.current_inst() {
                            b'[' => brackets += 1,
                            b']' => brackets -= 1,
                            _ => {}
                        }
                    }
                }
                b']' => 'bracketclose: {
                    if self.current() == 0 {
                        break 'bracketclose;
                    }

                    let mut brackets = 1;
                    while brackets > 0 {
                        self.current_inst -= 1;
                        match self.current_inst() {
                            b'[' => brackets -= 1,
                            b']' => brackets += 1,
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
            self.current_inst += 1
        }

        Ok(())
    }
}
