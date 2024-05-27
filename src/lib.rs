type Str = std::borrow::Cow<'static, str>;

mod data;

fn read_file() -> Result<ascii::AsciiString, Str> {
    let Some(file) = std::env::args().nth(1) else {
        return Err("".into());
    };
    todo!()
}
