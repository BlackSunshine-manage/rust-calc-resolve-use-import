mod calc_module;
mod calculator;

use calc_module::StandartCalculator;
use calculator::Calculator;

fn main() {
    let calc = StandartCalculator::new();
    let sum = calc.add(10, 20);
    println!("Sum is: {}", sum);

    let difference = calc.subtract(30, 15);
    println!("Difference is: {}", difference);

    let mult = calc.mult(5, 10);
    println!("Difference is: {}", mult);  
}