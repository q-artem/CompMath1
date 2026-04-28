pub type MathFunction = fn(f64) -> f64;

pub struct IntegralFunction {
    pub description: &'static str,
    pub func: MathFunction,
}

pub fn get_functions() -> Vec<IntegralFunction> {
    vec![
        IntegralFunction {
            description: "x^3 - 3x^2 + 5x + 12",
            func: |x| x.powi(3) - 3.0 * x.powi(2) + 5.0 * x + 12.0,
        },
        IntegralFunction {
            description: "sin(x)",
            func: |x| x.sin(),
        },
        IntegralFunction {
            description: "e^x",
            func: |x| x.exp(),
        },
        IntegralFunction {
            description: "1/x",
            func: |x| 1.0 / x,
        },
        IntegralFunction {
            description: "x^2",
            func: |x| x.powi(2),
        },
    ]
}
