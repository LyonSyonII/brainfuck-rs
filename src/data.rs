
type Value = i8;
pub struct Brainfuck {
    instructions: Vec<u8>,
    current: nonmax::NonMaxUsize,
    data: Vec<Cell>
}

pub struct Cell {
    value: Value,
    prev: Option<nonmax::NonMaxUsize>,
    next: Option<nonmax::NonMaxUsize>,
}

impl Brainfuck {
    pub fn new() -> Self {
        Brainfuck {
            instructions: Vec::new(),
            current: nonmax::NonMaxUsize::ZERO,
            data: vec![Cell { value: 0, prev: None, next: None }],
        }
    }

    pub fn inc(&mut self) {
        self.current_cell_mut().value += 1;
    }

    pub fn dec(&mut self) {
        self.current_cell_mut().value -= 1;
    }

    pub fn print(&self) {
        print!("{}", self.current())
    }

    /// Returns the current Cell's value.
    pub fn current(&self) -> Value {
        self.data[self.current.get()].value
    }

    fn current_cell(&self) -> &Cell {
        &self.data[self.current.get()]
    }

    fn current_cell_mut(&mut self) -> &mut Cell {
        &mut self.data[self.current.get()]
    }

    /// Retrocedes to previous Cell.
    pub fn prev(&mut self) {
        if let Some(prev) = self.current_cell().prev {
            self.current = prev;
        } else {
            self.data.push(Cell {
                value: 0,
                prev: None,
                next: Some(self.current),
            });
            let prev = nonmax::NonMaxUsize::new(self.data.len() - 1).unwrap();
            self.current_cell_mut().prev = Some(prev);
            self.current = prev;
        }
    }
    
    /// Advances to next Cell.
    pub fn next(&mut self) {
        if let Some(next) = self.current_cell().next {
            self.current = next;
        } else {
            self.data.push(Cell {
                value: 0,
                prev: Some(self.current),
                next: None,
            });
            let next = nonmax::NonMaxUsize::new(self.data.len() - 1).unwrap();
            self.current_cell_mut().next = Some(next);
            self.current = next;
        }
    }
}


mod tests {
    use super::Brainfuck;

    #[test]
    fn cells() {
        let mut brainfuck = Brainfuck::new();
        assert_eq!(brainfuck.current(), 0);
        
        brainfuck.inc();
        brainfuck.inc();
        assert_eq!(brainfuck.current(), 2);
        brainfuck.next();
        assert_eq!(brainfuck.current(), 0);

        brainfuck.inc();
        brainfuck.dec();
        brainfuck.inc();
        assert_eq!(brainfuck.current(), 1);
        brainfuck.prev();
        assert_eq!(brainfuck.current(), 2);

        brainfuck.prev();
        assert_eq!(brainfuck.current(), 0);
    }
}