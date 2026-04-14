use eframe::egui::{self};
use egui_plot::{HLine, Line, Plot, PlotPoints, Points, VLine};
use serde::{Deserialize, Serialize};
use std::fs;

use crate::lab2::equations::{EquationFn, get_non_linear_functions};
use crate::lab2::solvers::{RootCount, analyze_roots, dichotomy, secant, simple_iteration};

#[derive(Deserialize, Serialize)]
struct InputData {
    a: f64,
    b: f64,
    epsilon: f64,
    method: usize,
    equation_idx: usize,
}

pub fn run_ui() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 750.0])
            .with_title("Лабораторная работа №2 - Численные методы"),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Lab2 GUI",
        options,
        Box::new(|cc| {
            let mut style = (*cc.egui_ctx.style()).clone();
            for text_style in style.text_styles.values_mut() {
                text_style.size *= 1.5;
            }
            cc.egui_ctx.set_style(style);

            Box::new(Lab2App::new())
        }),
    );
}

struct Lab2App {
    equations: Vec<(String, EquationFn)>,
    selected_eq_idx: usize,
    a_str: String,
    b_str: String,
    eps_str: String,
    method: usize,

    // Результаты вычислений и статус
    result_text: String,
    io_message: String,

    // Данные для отрисовки и сохранения
    last_root: Option<f64>,
    last_f_value: Option<f64>,
    last_iterations: Option<usize>,

    // История
    prev_a: f64,
    prev_b: f64,
    prev_eq_idx: usize,
}

impl Lab2App {
    fn new() -> Self {
        let eq_map = get_non_linear_functions();
        let mut equations: Vec<(String, EquationFn)> = eq_map
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        equations.sort_by(|a, b| a.0.cmp(&b.0));

        Self {
            equations,
            selected_eq_idx: 0,
            a_str: "-5.0".to_string(),
            b_str: "5.0".to_string(),
            eps_str: "0.01".to_string(),
            method: 0,
            result_text: "Ожидание ввода данных...".into(),
            io_message: "Готово к работе".into(),
            last_root: None,
            last_f_value: None,
            last_iterations: None,
            prev_a: -5.0,
            prev_b: 5.0,
            prev_eq_idx: 0,
        }
    }

    fn load_from_file(&mut self) {
        match fs::read_to_string("input.json") {
            Ok(content) => match serde_json::from_str::<InputData>(&content) {
                Ok(data) => {
                    self.a_str = data.a.to_string();
                    self.b_str = data.b.to_string();
                    self.eps_str = data.epsilon.to_string();
                    self.method = data.method;
                    if data.equation_idx < self.equations.len() {
                        self.selected_eq_idx = data.equation_idx;
                    }
                    self.io_message = "Данные успешно загружены из input.json".into();
                    self.calculate();
                }
                Err(e) => self.io_message = format!("Ошибка парсинга JSON: {}", e),
            },
            Err(e) => self.io_message = format!("Ошибка чтения input.json: {}", e),
        }
    }

    fn save_to_file(&mut self) {
        if let (Some(root), Some(f_val), Some(iters)) =
            (self.last_root, self.last_f_value, self.last_iterations)
        {
            let eq_name = &self.equations[self.selected_eq_idx].0;
            let method_name = match self.method {
                0 => "Метод половинного деления",
                1 => "Метод секущих",
                _ => "Метод простой итерации",
            };

            let report = format!(
                "ОТЧЕТ О ВЫПОЛНЕНИИ ЛАБОРАТОРНОЙ РАБОТЫ №2\n\
                =========================================\n\
                Уравнение: f(x) = {}\n\
                Выбранный метод: {}\n\
                Интервал поиска: [{}, {}]\n\
                Заданная точность: {}\n\
                -----------------------------------------\n\
                РЕЗУЛЬТАТЫ:\n\
                Найденный корень x*: {:.10}\n\
                Значение функции f(x*): {:.2e}\n\
                Затрачено итераций: {}\n\
                =========================================\n",
                eq_name, method_name, self.a_str, self.b_str, self.eps_str, root, f_val, iters
            );

            match fs::write("output.txt", report) {
                Ok(_) => self.io_message = "Результат успешно сохранен в output.txt".into(),
                Err(e) => self.io_message = format!("Ошибка записи файла: {}", e),
            }
        } else {
            self.io_message = "Ошибка: Сначала вычислите корень, чтобы его сохранить!".into();
        }
    }

    fn calculate(&mut self) {
        self.last_root = None;
        self.last_f_value = None;
        self.last_iterations = None;

        let a: f64 = match self.a_str.replace(',', ".").parse() {
            Ok(val) => val,
            Err(_) => {
                self.result_text = "Ошибка: 'a' должно быть числом".into();
                return;
            }
        };
        let b: f64 = match self.b_str.replace(',', ".").parse() {
            Ok(val) => val,
            Err(_) => {
                self.result_text = "Ошибка: 'b' должно быть числом".into();
                return;
            }
        };
        let eps: f64 = match self.eps_str.replace(',', ".").parse() {
            Ok(val) => val,
            Err(_) => {
                self.result_text = "Ошибка: Точность должна быть числом".into();
                return;
            }
        };

        if a >= b {
            self.result_text = "Ошибка: Граница (a) должна быть меньше (b)!".into();
            return;
        }

        let f = self.equations[self.selected_eq_idx].1;

        match analyze_roots(f, a, b) {
            RootCount::Zero => {
                self.result_text = "На интервале корней не обнаружено.\nСмените границы.".into();
                return;
            }
            RootCount::Multiple(n) => {
                self.result_text = format!(
                    "Обнаружено несколько корней (минимум {}).\nСузьте интервал.",
                    n
                );
                return;
            }
            RootCount::One => {} // Все ок
        }

        let result = match self.method {
            0 => dichotomy(f, a, b, eps),
            1 => secant(f, a, b, eps),
            2 => simple_iteration(f, a, b, eps),
            _ => Err("Неизвестный метод".into()),
        };

        match result {
            Ok(res) => {
                self.last_root = Some(res.root);
                self.last_f_value = Some(res.f_value);
                self.last_iterations = Some(res.iterations);

                self.result_text = format!(
                    "УСПЕХ!\nКорень: {:.8}\nf(x): {:.2e}\nИтераций: {}",
                    res.root, res.f_value, res.iterations
                );
                self.io_message = "Вычисления успешно завершены.".into();
            }
            Err(e) => {
                self.result_text = format!("ОШИБКА:\n{}", e);
                self.io_message = "Произошла ошибка при вычислениях.".into();
            }
        }
    }

    fn get_y_range(&self, a: f64, b: f64) -> (f64, f64) {
        let f = self.equations[self.selected_eq_idx].1;
        let steps = 500;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for i in 0..=steps {
            let x = a + (b - a) * (i as f64) / steps as f64;
            let y = f(x);
            if y.is_finite() {
                min_y = min_y.min(y);
                max_y = max_y.max(y);
            }
        }

        if !min_y.is_finite() {
            (-10.0, 10.0)
        } else {
            (min_y, max_y)
        }
    }
}

impl eframe::App for Lab2App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.viewport().close_requested()) {
            std::process::exit(0);
        }

        //  ==================== Левая панель =======================
        egui::SidePanel::left("input_panel")
            .min_width(400.0)
            .show(ctx, |ui| {
                ui.heading("Параметры уравнения");
                ui.add_space(10.0);

                ui.label("Выберите функцию:");
                egui::ComboBox::from_id_source("eq_selector")
                    .selected_text(&self.equations[self.selected_eq_idx].0)
                    .width(380.0)
                    .show_ui(ui, |ui| {
                        for (i, (name, _)) in self.equations.iter().enumerate() {
                            ui.selectable_value(&mut self.selected_eq_idx, i, name);
                        }
                    });

                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.label("Граница a:");
                    ui.add(egui::TextEdit::singleline(&mut self.a_str).desired_width(80.0));
                    ui.label("Граница b:");
                    ui.add(egui::TextEdit::singleline(&mut self.b_str).desired_width(80.0));
                });

                ui.horizontal(|ui| {
                    ui.label("Точность:");
                    ui.add(egui::TextEdit::singleline(&mut self.eps_str).desired_width(100.0));
                });

                ui.add_space(20.0);
                ui.separator();
                ui.add_space(10.0);

                ui.heading("Метод решения");
                ui.radio_value(&mut self.method, 0, "Дихотомия (Половинное деление)");
                ui.radio_value(&mut self.method, 1, "Метод секущих");
                ui.radio_value(&mut self.method, 2, "Метод простой итерации");

                ui.add_space(20.0);

                // --- БЛОК КНОПОК ---
                ui.horizontal(|ui| {
                    if ui.button("▶ Вычислить").clicked() {
                        self.calculate();
                    }
                });

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    if ui.button("📂 Загрузить (input.json)").clicked() {
                        self.load_from_file();
                    }
                    if ui.button("💾 Сохранить (output.txt)").clicked() {
                        self.save_to_file();
                    }
                });

                ui.add_space(10.0);

                ui.label(egui::RichText::new(&self.io_message).color(egui::Color32::LIGHT_GREEN));

                ui.add_space(20.0);
                ui.separator();
                ui.add_space(10.0);

                ui.heading("Результат:");
                ui.label(&self.result_text);
            });

        //  ==================== Правая панель =======================
        egui::CentralPanel::default().show(ctx, |ui| {
            let a = self.a_str.replace(',', ".").parse::<f64>().unwrap_or(-5.0);
            let b = self.b_str.replace(',', ".").parse::<f64>().unwrap_or(5.0);
            let f = self.equations[self.selected_eq_idx].1;

            let params_changed =
                a != self.prev_a || b != self.prev_b || self.selected_eq_idx != self.prev_eq_idx;

            let plot = Plot::new("lab_plot")
                .view_aspect(2.0)
                .legend(egui_plot::Legend::default());

            plot.show(ui, |plot_ui| {
                if params_changed {
                    let (min_y, max_y) = self.get_y_range(a, b);
                    let margin_x = (b - a).abs() * 0.1;
                    let margin_y = (max_y - min_y).abs() * 0.1;

                    let bounds = egui_plot::PlotBounds::from_min_max(
                        [a - margin_x, min_y - margin_y.max(1.0)],
                        [b + margin_x, max_y + margin_y.max(1.0)],
                    );
                    plot_ui.set_plot_bounds(bounds);
                }

                let padding = (b - a).abs() * 0.1;
                let plot_start = a - padding;
                let plot_end = b + padding;

                let points: PlotPoints = (0..500)
                    .map(|i| {
                        let x = plot_start + (plot_end - plot_start) * (i as f64) / 500.0;
                        [x, f(x)]
                    })
                    .collect();

                plot_ui.line(
                    Line::new(points)
                        .name("f(x)")
                        .color(egui::Color32::LIGHT_BLUE),
                );

                plot_ui.hline(
                    HLine::new(0.0)
                        .color(egui::Color32::YELLOW)
                        .width(2.0)
                        .name("Ось X"),
                );
                plot_ui.vline(
                    VLine::new(0.0)
                        .color(egui::Color32::YELLOW)
                        .width(2.0)
                        .name("Ось Y"),
                );

                plot_ui.vline(
                    VLine::new(a)
                        .color(egui::Color32::GRAY)
                        .style(egui_plot::LineStyle::Dashed { length: 5.0 })
                        .name("Граница a"),
                );
                plot_ui.vline(
                    VLine::new(b)
                        .color(egui::Color32::GRAY)
                        .style(egui_plot::LineStyle::Dashed { length: 5.0 })
                        .name("Граница b"),
                );

                if let Some(root) = self.last_root {
                    let root_point = Points::new(vec![[root, 0.0]])
                        .color(egui::Color32::RED)
                        .radius(5.0)
                        .name(format!("Корень: {:.4}", root));
                    plot_ui.points(root_point);
                }
            });

            // Обновляем память
            if params_changed {
                self.prev_a = a;
                self.prev_b = b;
                self.prev_eq_idx = self.selected_eq_idx;

                self.last_root = None;
            }
        });
    }
}
