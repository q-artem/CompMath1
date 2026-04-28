use data_io::print_header;
use std::io::Write;

use crate::data_io::Align::Right;
use crate::data_io::{print, print_sep_line, read_choice};

mod data_io;
mod lab1;
mod lab2;
mod lab3;

fn main() {
    print_header("Вычислительная математика. Лабораторные работы", 1);
    print("Пшеничников Артём Дмитриевич, P3207, 467205", Right);

    loop {
        print_header("Выберите лабораторную для запуска", 2);
        println!("1.  Лабораторная работа 1");
        println!("2.  Лабораторная работа 2 (CLI)");
        println!("3.  Лабораторная работа 2 (UI)");
        println!("4.  Лабораторная работа 3 (Численное интегрирование)");
        println!("0.  Выход");

        print_sep_line(2);
        print!("Выберите пункт: ");
        std::io::stdout().flush().unwrap();

        match read_choice() {
            Some(1) => {
                lab1::solve();
            }
            Some(2) => {
                lab2::solve();
            }
            Some(3) => {
                lab2::ui::run_ui();
            }
            Some(4) => {
                lab3::solve();
            }
            Some(0) => break,
            _ => println!("Ошибка. Введите корректный номер лабораторной"),
        }
    }
    print_header("ЗАВЕРШЕНИЕ РАБОТЫ. СПАСИБО ЗА ИСПОЛЬЗОВАНИЕ ПРОГРАММЫ", 1);
}
