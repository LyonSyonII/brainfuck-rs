use std::io::Read;

use ascii::AsAsciiStr;

type Value = u8;
pub struct Brainfuck {
    instructions: ascii::AsciiString,
    current_inst: usize,
    data: Vec<Cell>,
    current_cell: nonmax::NonMaxUsize,
}

pub struct Cell {
    value: Value,
    prev: Option<nonmax::NonMaxUsize>,
    next: Option<nonmax::NonMaxUsize>,
}

impl Brainfuck {
    pub fn new(instructions: impl ascii::IntoAsciiString) -> Self {
        Brainfuck {
            instructions: instructions.into_ascii_string().unwrap(),
            current_inst: 0,
            current_cell: nonmax::NonMaxUsize::ZERO,
            data: vec![Cell {
                value: 0,
                prev: None,
                next: None,
            }],
        }
    }

    fn inc(&mut self) {
        self.current_cell_mut().value += 1;
    }

    fn dec(&mut self) {
        self.current_cell_mut().value -= 1;
    }

    fn print(&self) {
        print!("{}", self.current() as char)
    }

    /// Returns the current Cell's value.
    fn current(&self) -> Value {
        self.data[self.current_cell.get()].value
    }

    fn current_cell(&self) -> &Cell {
        &self.data[self.current_cell.get()]
    }

    fn current_cell_mut(&mut self) -> &mut Cell {
        &mut self.data[self.current_cell.get()]
    }

    fn current_inst(&self) -> ascii::AsciiChar {
        self.instructions[self.current_inst]
    }

    /// Retrocedes to previous Cell.
    fn prev(&mut self) {
        if let Some(prev) = self.current_cell().prev {
            self.current_cell = prev;
        } else {
            self.data.push(Cell {
                value: 0,
                prev: None,
                next: Some(self.current_cell),
            });
            let prev = nonmax::NonMaxUsize::new(self.data.len() - 1).unwrap();
            self.current_cell_mut().prev = Some(prev);
            self.current_cell = prev;
        }
    }

    /// Advances to next Cell.
    fn next(&mut self) {
        if let Some(next) = self.current_cell().next {
            self.current_cell = next;
        } else {
            self.data.push(Cell {
                value: 0,
                prev: Some(self.current_cell),
                next: None,
            });
            let next = nonmax::NonMaxUsize::new(self.data.len() - 1).unwrap();
            self.current_cell_mut().next = Some(next);
            self.current_cell = next;
        }
    }

    pub fn execute(mut self) -> std::io::Result<()> {
        while let Some(inst) = self.instructions.get_ascii(self.current_inst) {
            match inst {
                ascii::AsciiChar::LessThan => self.prev(),
                ascii::AsciiChar::GreaterThan => self.next(),
                ascii::AsciiChar::Plus => self.inc(),
                ascii::AsciiChar::Minus => self.dec(),
                ascii::AsciiChar::Dot => self.print(),
                ascii::AsciiChar::Comma => {
                    let mut input: u8 = 0;
                    std::io::stdin().read_exact(std::slice::from_mut(&mut input))?;
                    self.current_cell_mut().value = input;
                }
                ascii::AsciiChar::BracketOpen => 'bracketopen: {
                    if self.current() != 0 { 
                        break 'bracketopen;
                    }
           
                    let mut brackets = 1;
                    while brackets > 0 {
                        self.current_inst += 1;
                        match self.current_inst() {
                            ascii::AsciiChar::BracketOpen => brackets += 1,
                            ascii::AsciiChar::BracketClose => brackets -= 1,
                            _ => {}
                        }
                    }
                },
                ascii::AsciiChar::BracketClose => 'bracketclose: {
                    if self.current() == 0 { 
                        break 'bracketclose;
                    }
           
                    let mut brackets = 1;
                    while brackets > 0 {
                        self.current_inst -= 1;
                        match self.current_inst() {
                            ascii::AsciiChar::BracketOpen => brackets -= 1,
                            ascii::AsciiChar::BracketClose => brackets += 1,
                            _ => {}
                        }
                    }
                },
                _ => {}
            }
            self.current_inst += 1
        }

        Ok(())
    }
}