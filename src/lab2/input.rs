use crate::{
    data_io::{print_header, read_choice},
    lab2::equations::{EquationFn, get_non_linear_functions},
};
use std::io::{self, Write};

pub(crate) fn select_non_linear() -> EquationFn {
    let functions = get_non_linear_functions();

    let mut names: Vec<&&str> = functions.keys().collect();
    names.sort();

    loop {
        print_header("Выбор графика", 3);
        for (i, name) in names.iter().enumerate() {
            println!("{}. {}", i + 1, name);
        }
        print!("Выберите номер: ");
        io::stdout().flush().unwrap();

        match read_choice() {
            Some(n) if n > 0 && n <= (names.len() as u32) => {
                let selected_name = names[(n - 1) as usize];
                return *functions.get(selected_name).unwrap();
            }
            _ => println!("Ошибка. Попробуйте еще раз"),
        }
    }
}
