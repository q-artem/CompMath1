use std::io::{self, Write};
pub mod input;

use crate::{
    data_io::{print_header, print_sep_line, read_choice},
    lab2::utils::plot_equation,
};

pub mod utils;
pub mod equations;

pub(crate) fn solve() {
    loop {
        print_menu();
        match read_choice() {
            Some(1) => {
                non_linear_equation();
            }
            Some(2) => {}
            Some(0) => break,
            _ => println!("Ошибка. Попробуйте еще раз"),
        }
    }
}

fn non_linear_equation() {
    match plot_equation(input::select_non_linear(), -5.0, 5.0, "graph.png") {
        Ok(_) => println!("График успешно сохранен в файл graph.png"),
        Err(e) => println!("Ошибка при рисовании графика: {}", e),
    }
}

fn print_menu() {
    print_header("ГЛАВНОЕ МЕНЮ", 2);
    println!("1.  Нелинейное уравнение");
    println!("2.  Система нелинейных уравнений");
    println!("0.  Выход");
    print_sep_line(2);
    print!("Выберите пункт: ");

    io::stdout().flush().unwrap();
}
