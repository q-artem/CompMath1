use std::io::{self, Write};

const CONSOLE_WIDTH: usize = 80;

pub fn print_header(title: &str, level: u8) {
    let title_len = title.chars().count();
    if title_len + 2 >= CONSOLE_WIDTH {
        println!(" {} ", title);
        return;
    }
    let total = CONSOLE_WIDTH - title_len - 2;

    if level <= 3 {
        let symbol = match level {
            1 => "=",
            2 => "-",
            _ => "#",
        };
        let left = symbol.repeat(total / 2);
        let right = symbol.repeat(total - total / 2);
        println!("{} {} {}", left, title, right);
    }
}

pub fn print_sep_line(level: u8) {
    let symbol = match level {
        1 => "=",
        2 => "-",
        _ => "#",
    };
    println!("{}", symbol.repeat(CONSOLE_WIDTH));
}

pub enum Align {
    Left,
    Center,
    Right,
}

pub fn print(text: &str, align: Align) {
    let chars_count = text.chars().count();
    if chars_count >= CONSOLE_WIDTH {
        println!("{}", text);
        return;
    }

    let padding = CONSOLE_WIDTH - chars_count;
    let spaces = " ".repeat(padding);

    match align {
        Align::Left => {
            println!("{}{}", text, spaces);
        }
        Align::Right => {
            println!("{}{}", spaces, text);
        }
        Align::Center => {
            let left_pad = padding / 2;
            let right_pad = padding - left_pad;
            println!("{}{}{}", " ".repeat(left_pad), text, " ".repeat(right_pad));
        }
    }
}

pub fn print_menu() {
    print_header("ГЛАВНОЕ МЕНЮ", 2);
    println!("1.  Ввести матрицу с клавиатуры");
    println!("2.  Прочитать матрицу из файла");
    println!("3.  Запустить тестовый пример (из методички)");
    println!("4.  Сгенерировать случайную матрицу");
    println!("42. Ответ на главный вопрос жизни, вселенной и всего такого");
    println!("0.  Выход");
    print_sep_line(2);
    print!("Выберите пункт: ");

    io::stdout().flush().unwrap();
}

pub fn read_choice() -> Option<u32> {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return None;
    }
    input.trim().parse::<u32>().ok()
}

pub fn print_matrix(a: &Vec<Vec<f64>>, b: &Vec<f64>, precision: usize) {
    let n = a.len();

    let cell_width = precision + 4;

    let sep_line = "-".repeat((n + 1) * (cell_width + 1) + 5);
    println!("{}", sep_line);
    for i in 0..n {
        print!("| ");

        for j in 0..n {
            print!(
                "{:>width$.prec$} ",
                a[i][j],
                width = cell_width,
                prec = precision
            );
        }

        println!(
            "| {:>width$.prec$} |",
            b[i],
            width = cell_width,
            prec = precision
        );
    }
    println!("{}", sep_line);
}
