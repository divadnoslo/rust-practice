enum Expression {
    Value(f64), 
    Add(f64, f64),
    Subtract(f64, f64), 
    Multiply(f64, f64), 
    Divide(f64, f64),
    SquareRoot(f64)
}

impl Expression {
    fn evaluate(&self) -> Option<f64> {
        match self {
            Expression::Value(val) => Some(*val), 
            Expression::Add(val1, val2) => Some(*val1 + *val2),
            Expression::Subtract(val1, val2) => Some(*val1 - *val2),
            Expression::Multiply(val1, val2) => Some(*val1 * *val2), 
            Expression::Divide(val1, val2) => {
                match val2 {
                    0.0 => None,
                    _ => Some(*val1 / *val2),
                }
            }
            Expression::SquareRoot(val) => {
                if *val < 0.0 {
                    None
                } else {
                    Some(val.sqrt())
                }
            }
        }
    }
}

fn main() {

    // 1. Create a Vector holding multiple Expression variants
    let expressions: Vec<Expression> = vec![
        Expression::Value(42.0),
        Expression::Add(10.0, 5.5),
        Expression::Subtract(20.0, 8.0),
        Expression::Multiply(3.0, 4.0),
        Expression::Divide(25.0, 15.0),
        Expression::Divide(1.0, 0.0),
        Expression::SquareRoot(4.0),
        Expression::SquareRoot(-4.0),
    ];

    // 2. Iterate through the vector by reference
    for expr in &expressions {
        // 3. Call evaluate() and handle the Option result
        match expr.evaluate() {
            Some(result) => println!("Result: {}", result),
            None => println!("Error: Invalid operation!"),
        }
    }
    
}
