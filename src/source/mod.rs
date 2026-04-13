pub mod file_source;

pub trait LogSource {
    fn read_line(&mut self) -> Option<String>;
}
