pub trait Day {
    fn run(&self, input: Input) -> i32;
}

pub enum Input {
    Example,
    Custom
}