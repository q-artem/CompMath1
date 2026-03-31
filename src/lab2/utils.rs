use std::io::{Write, stdin, stdout};

use color_eyre::Result;
use plotters::{
    chart::ChartBuilder,
    prelude::{BitMapBackend, Circle, IntoDrawingArea, PathElement},
    series::LineSeries,
    style::{BLACK, BLUE, Color, IntoFont, RED, WHITE},
};

pub type EquationFn = fn(f64) -> f64;
pub type SystemFn = (fn(f64, f64) -> f64, fn(f64, f64) -> f64);

pub fn diff(f: EquationFn, x: f64, eps: f64) -> f64 {
    (f(x + eps) - f(x)) / eps
}

pub fn second_diff(f: EquationFn, x: f64, eps: f64) -> f64 {
    (diff(f, x + eps, eps) - diff(f, x, eps)) / eps
}

pub fn diff_x(f: fn(f64, f64) -> f64, x: f64, y: f64, eps: f64) -> f64 {
    (f(x + eps, y) - f(x, y)) / eps
}

pub fn diff_y(f: fn(f64, f64) -> f64, x: f64, y: f64, eps: f64) -> f64 {
    (f(x, y + eps) - f(x, y)) / eps
}

pub fn input(prompt: &str) -> Result<String> {
    stdout().write(prompt.as_bytes())?;
    stdout().flush()?;

    let mut buf = String::new();
    stdin().read_line(&mut buf)?;

    if buf.ends_with('\n') {
        buf.pop();
    }

    Ok(buf)
}

pub fn plot_equation(f: impl Fn(f64) -> f64, a: f64, b: f64, file_name: &str) -> Result<()> {
    let root = BitMapBackend::new(file_name, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let steps = 1000;
    let step_size = (b - a) / steps as f64;

    let mut min_y: f64 = 1000.0;
    let mut max_y: f64 = -1000.0;

    let points: Vec<(f64, f64)> = (0..=steps)
        .filter_map(|i| {
            let x = a + (i as f64) * step_size;
            let y = f(x);

            if y.is_finite() {
                if y < min_y {
                    min_y = y;
                }
                if y > max_y {
                    max_y = y;
                }
                Some((x, y))
            } else {
                None
            }
        })
        .collect();

    let y_padding = (max_y - min_y) * 0.1;
    let (y_min_padded, y_max_padded) = if y_padding == 0.0 {
        (min_y - 1.0, max_y + 1.0) // Если функция константа
    } else {
        (min_y - y_padding, max_y + y_padding)
    };

    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("График функции на отрезке [{}, {}]", a, b),
            ("sans-serif", 30).into_font(),
        )
        .margin(15)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(a..b, y_min_padded..y_max_padded)?;

    chart
        .configure_mesh()
        .x_desc("Ось X")
        .y_desc("Ось Y")
        .draw()?;

    chart
        .draw_series(LineSeries::new(points, &RED))?
        .label("f(x)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}

fn find_zero_crossings(
    func: impl Fn(f64, f64) -> f64,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    steps: usize,
) -> Vec<(f64, f64)> {
    let mut points = Vec::new();
    let dx = (x_max - x_min) / steps as f64;
    let dy = (y_max - y_min) / steps as f64;

    for i in 0..steps {
        for j in 0..steps {
            let x = x_min + (i as f64) * dx;
            let y = y_min + (j as f64) * dy;

            let v_center = func(x, y);
            let v_right = func(x + dx, y);
            let v_top = func(x, y + dy);

            if v_center * v_right <= 0.0 || v_center * v_top <= 0.0 {
                points.push((x, y));
            }
        }
    }
    points
}

pub fn plot_system(
    f: impl Fn(f64, f64) -> f64,
    g: impl Fn(f64, f64) -> f64,
    center: (f64, f64),
    span_x: f64,
    span_y: f64,
    file_name: &str,
) -> Result<()> {
    let x_min = center.0 - span_x;
    let x_max = center.0 + span_x;
    let y_min = center.1 - span_y;
    let y_max = center.1 + span_y;

    // 1. Создаем холст 800x800, так как область обычно квадратная
    let root = BitMapBackend::new(file_name, (800, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    // 2. Настраиваем график
    let mut chart = ChartBuilder::on(&root)
        .caption("f(x,y)=0 и g(x,y)=0", ("sans-serif", 30).into_font())
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart
        .configure_mesh()
        .x_desc("Ось X")
        .y_desc("Ось Y")
        .draw()?;

    let resolution = 800;
    let points_f = find_zero_crossings(&f, x_min, x_max, y_min, y_max, resolution);
    let points_g = find_zero_crossings(&g, x_min, x_max, y_min, y_max, resolution);

    chart
        .draw_series(
            points_f
                .into_iter()
                .map(|point| Circle::new(point, 1, RED.filled())),
        )?
        .label("f(x,y) = 0")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .draw_series(
            points_g
                .into_iter()
                .map(|point| Circle::new(point, 1, BLUE.filled())),
        )?
        .label("g(x,y) = 0")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    Ok(())
}
