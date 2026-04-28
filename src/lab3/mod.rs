pub mod functions;
pub mod input;
pub mod methods;

use crate::data_io::{print_header, print_sep_line};
use crate::lab3::input::{choose_function, choose_method, read_limits, read_precision};
use crate::lab3::methods::integrate_with_precision;

pub fn solve() {
    print_header("Лабораторная работа №3. Численное интегрирование", 1);

    loop {
        let function_data = match choose_function() {
            Some(f) => f,
            None => break,
        };

        let method = choose_method();
        let (mut a, mut b) = read_limits();
        let epsilon = read_precision();
        let initial_n = 4;

        let mut multiplier = 1.0;
        if a > b {
            std::mem::swap(&mut a, &mut b);
            multiplier = -1.0;
        }

        // Простая проверка на разрывы (для 1/x)
        if function_data.description == "1/x" && a <= 0.0 && b >= 0.0 {
            println!(
                "\nОШИБКА: Функция 1/x имеет неустранимый разрыв в точке x=0 на интервале [{:.3}, {:.3}].",
                a, b
            );
            println!("Интеграл расходится или не определен в обычном смысле.");
            print_sep_line(2);
            continue;
        }

        print_sep_line(2);
        println!(
            "Вычисляем интеграл f(x) = {} на интервале [{:.3}, {:.3}]",
            function_data.description, a, b
        );
        println!("Метод: {}", method.name());
        println!("Требуемая точность: {}", epsilon);

        match integrate_with_precision(method, function_data.func, a, b, epsilon, initial_n) {
            Ok(result) => {
                let final_value = result.value * multiplier;
                print_header("РЕЗУЛЬТАТ:", 3);
                println!("Значение интеграла: {:.10}", final_value);
                println!("Число разбиений (n): {}", result.n);
                println!("Оценочная погрешность (по Рунге): {:.10}", result.error);
            }
            Err(e) => {
                println!("ОШИБКА ПРИ ВЫЧИСЛЕНИИ: {}", e);
            }
        }
    }
}
