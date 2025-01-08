use std::f64::{consts::PI, INFINITY, NEG_INFINITY};

use ndarray::prelude::*;
use ndarray_linalg::Solve;

use plotters::prelude::*;

pub fn plot_phi_into_file<F>(phi: F, filename: &str, sample_n: usize) -> Result<(), Box<dyn std::error::Error>> 
where
    F: Fn(f64) -> f64
{
    let domain = (0..=sample_n).map(|x| 3. * (x as f64) / (sample_n as f64));
    let phi_vector = domain.map(|x| (x, phi(x)));

    let (miny, maxy) = phi_vector
        .clone()
        .fold((INFINITY, NEG_INFINITY),|(miny, maxy), (_, y)| (miny.min(y), maxy.max(y)));

    let root = BitMapBackend::new(filename, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=φ(x)", ("sans-serif", 24).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..3f64, miny..maxy)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            phi_vector,
            &RED,
        ))?;

    root.present()?;

    Ok(())
}

pub fn phi_bilinear_matrix(n: usize) -> Array2<f64> {
    let mut matrix = Array2::<f64>::zeros((n - 1, n - 1));
    let h: f64 = 3.0 / n as f64;

    let x_1 = h;
    let x_2 = 2. * h;
    let de_1 = |x| de(1, x, h);
    let de_2 = |x| de(2, x, h);
    matrix[[0, 0]] = -bilinear_functional(de_1, de_1, 0., x_2);
    matrix[[0, 1]] = -bilinear_functional(de_1, de_2, x_1, x_2);

    for i in 2..n - 1 {
        let x_i = i as f64 * h;
        let x_i_dec = (i - 1) as f64 * h;
        let x_i_inc = (i + 1) as f64 * h;
        let de_i = |x| de(i, x, h);
        let de_i_dec = |x| de(i - 1, x, h);
        let de_i_inc = |x| de(i + 1, x, h);

        matrix[[i - 1, i - 1]] = -bilinear_functional(de_i, de_i, x_i_dec, x_i_inc);
        matrix[[i - 1, i - 2]] = -bilinear_functional(de_i, de_i_dec, x_i_dec, x_i);
        matrix[[i - 1, i]] = -bilinear_functional(de_i, de_i_inc, x_i, x_i_inc);
    }

    let x_n = n as f64 * h;
    let x_n_dec = (n - 1) as f64 * h;
    let x_n_dec_dec = (n - 2) as f64 * h;
    let de_n_dec = |x| de(n - 1, x, h);
    let de_n_dec_dec = |x| de(n - 2, x, h);
    matrix[[n - 2, n - 3]] = -bilinear_functional(de_n_dec, de_n_dec_dec, x_n_dec_dec, x_n_dec);
    matrix[[n - 2, n - 2]] = -bilinear_functional(de_n_dec, de_n_dec, x_n_dec_dec, x_n);

    matrix
}

pub fn phi_linear_matrix(n: usize, grav_const: f64) -> Array1<f64> {
    let mut matrix = Array1::<f64>::zeros(n - 1);
    let h: f64 = 3.0 / n as f64;

    for i in 1..n {
        let x_i = i as f64 * h;
        if x_i < 1. {
            continue;
        }
        if x_i > 2. {
            break;
        }

        matrix[i - 1] =
            - 4. * PI * grav_const * linear_functional(|x| e(i, x, h), (x_i - h).max(1.), (x_i + h).min(2.))
    }
    matrix
}

pub fn solve_for_phi_ext(bilinear_matrix: &Array2<f64>, linear_matrix: &Array1<f64>) -> Array1<f64> {
    bilinear_matrix.solve(&linear_matrix).unwrap()
}

pub fn construct_phi(phi_ext: &Array1<f64>, n: usize) -> impl Fn(f64) -> f64 + '_ {
    move |x| {
        5. - (x / 3.) 
        + (1..n).fold(0., |acc, i|  acc + phi_ext[i - 1] * e(i, x, 3. / n as f64))
    }
}

// basis function
fn e(k: usize, x: f64, h: f64) -> f64 {
    let x_k = k as f64 * h;

    if x > x_k - h && x <= x_k {
        x / h - (k - 1) as f64
    } else if x > x_k && x < x_k + h {
        -x / h + (k + 1) as f64
    } else {
        0.
    }
}

// derivative of basis function
fn de(k: usize, x: f64, h: f64) -> f64 {
    let x_k = k as f64 * h;

    if x > x_k - h && x <= x_k {
        1. / h
    } else if x > x_k && x < x_k + h {
        -1. / h
    } else {
        0.
    }
}

fn estimate_integral<F>(f: F, a: f64, b: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    let coef1: f64 = (b - a) as f64 / 2.;
    let coef2: f64 = (a + b) as f64 / 2.;

    let inv_sqrt3 = 1. / f64::sqrt(3.);

    coef1 * (f(coef1 * inv_sqrt3 + coef2) + f(-coef1 * inv_sqrt3 + coef2))
}

fn bilinear_functional<F1, F2>(f1: F1, f2: F2, a: f64, b: f64) -> f64
where
    F1: Fn(f64) -> f64,
    F2: Fn(f64) -> f64,
{
    estimate_integral(|x| f1(x) * f2(x), a, b)
}

fn linear_functional<F>(f: F, a: f64, b: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    estimate_integral(f, a, b)
}
