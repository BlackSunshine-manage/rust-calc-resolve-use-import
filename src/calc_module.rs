use crate::calculator::Calculator;

pub struct StandartCalculator {}

impl Calculator for StandartCalculator {
    fn new() -> Self {
        StandartCalculator {} // Конструктор для создания нового экземпляра
    }

    fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    fn mult(&self, a: i32, b: i32) -> i32 {
        a * b
    }

    fn subtract(&self, a: i32, b: i32) -> i32 {
        a - b
    }
}