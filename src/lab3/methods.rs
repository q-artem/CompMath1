use crate::lab3::functions::MathFunction;

#[derive(Clone, Copy)]
pub enum IntegrationMethodType {
    LeftRectangle,
    RightRectangle,
    MidpointRectangle,
    Trapezoid,
    Simpson,
}

impl IntegrationMethodType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::LeftRectangle => "Метод левых прямоугольников",
            Self::RightRectangle => "Метод правых прямоугольников",
            Self::MidpointRectangle => "Метод средних прямоугольников",
            Self::Trapezoid => "Метод трапеций",
            Self::Simpson => "Метод Симпсона",
        }
    }

    pub fn algebraic_order(&self) -> u32 {
        match self {
            Self::LeftRectangle | Self::RightRectangle => 1,
            Self::MidpointRectangle | Self::Trapezoid => 2,
            Self::Simpson => 4,
        }
    }
}

pub fn integrate(method: IntegrationMethodType, f: MathFunction, a: f64, b: f64, n: usize) -> f64 {
    let h = (b - a) / n as f64;
    let mut sum = 0.0;

    match method {
        IntegrationMethodType::LeftRectangle => {
            for i in 0..n {
                sum += f(a + i as f64 * h);
            }
            sum * h
        }
        IntegrationMethodType::RightRectangle => {
            for i in 1..=n {
                sum += f(a + i as f64 * h);
            }
            sum * h
        }
        IntegrationMethodType::MidpointRectangle => {
            for i in 0..n {
                sum += f(a + (i as f64 + 0.5) * h);
            }
            sum * h
        }
        IntegrationMethodType::Trapezoid => {
            sum = (f(a) + f(b)) / 2.0;
            for i in 1..n {
                sum += f(a + i as f64 * h);
            }
            sum * h
        }
        IntegrationMethodType::Simpson => {
            sum = f(a) + f(b);
            for i in 1..n {
                let x = a + i as f64 * h;
                if i % 2 == 0 {
                    sum += 2.0 * f(x);
                } else {
                    sum += 4.0 * f(x);
                }
            }
            sum * h / 3.0
        }
    }
}

pub struct IntegrationResult {
    pub value: f64,
    pub n: usize,
    pub error: f64,
}

pub fn integrate_with_precision(
    method: IntegrationMethodType,
    f: MathFunction,
    a: f64,
    b: f64,
    epsilon: f64,
    mut n: usize,
) -> Result<IntegrationResult, String> {
    if matches!(method, IntegrationMethodType::Simpson) && n % 2 != 0 {
        n += 1;
    }

    let k = method.algebraic_order();
    let mut i_n = integrate(method, f, a, b, n);

    loop {
        if n > 1_000_000 {
            return Err(
                "Превышено максимальное число разбиений (1,000,000). Проверьте функцию на разрывы."
                    .to_string(),
            );
        }

        let n_new = n * 2;
        let i_2n = integrate(method, f, a, b, n_new);

        let runge_error = (i_2n - i_n).abs() / ((2.0_f64.powi(k as i32)) - 1.0);

        if runge_error <= epsilon {
            return Ok(IntegrationResult {
                value: i_2n,
                n: n_new,
                error: runge_error,
            });
        }

        i_n = i_2n;
        n = n_new;
    }
}
