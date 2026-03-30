use std::io::Write;
use io::print_header;

use crate::io::{print, print_sep_line, read_choice};
use crate::io::Align::Right;
mod lab1;

mod io;
mod lab2;

fn main() {
    print_header("Вычислительная математика. Лабораторные работы", 1);
    print("Пшеничников Артём Дмитриевич, P3207, 467205", Right);

    loop {
        print_header("Выберите лабораторную для запуска", 2);
        println!("1.  Лабораторная работа 1");
        println!("2.  Лабораторная работа 2");

        print_sep_line(2);
        print!("Выберите пункт: ");
        std::io::stdout().flush().unwrap();
        match read_choice() {
            Some(1) => {
                lab1::solve();
                break;
            }
            _ => println!("Ошибка. Введите корректный номер лабораторной")
        }
    }
    print_header("ЗАВЕРШЕНИЕ РАБОТЫ. СПАСИБО ЗА ИСПОЛЬЗОВАНИЕ ПРОГРАММЫ", 1);
}
