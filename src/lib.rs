use data::Brainfuck;

type Str = std::borrow::Cow<'static, str>;

mod data;

pub fn run_brainfuck() {
    let Some(file) = std::env::args().nth(1) else {
        println!("brainfuck FILE");
        return;
    };
    let file = std::fs::read_to_string(file).unwrap();
    Brainfuck::new(file).execute().unwrap();
}
