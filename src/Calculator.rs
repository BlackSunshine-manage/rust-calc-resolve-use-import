pub trait Calculator {
    fn new() -> Self;
    fn add(&self, a: i32, b: i32) -> i32;
    fn mult(&self, a: i32, b: i32) -> i32;
    fn subtract(&self, a: i32, b: i32) -> i32;
}