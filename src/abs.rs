use num_complex::Complex;
use pyo3::pyfunction;

#[pyfunction]
pub fn abs_i32(number: i32) -> i32 {
    number.abs()
}

#[pyfunction]
pub fn abs_i64(number: i64) -> i64 {
    number.abs()
}

#[pyfunction]
pub fn abs_f32(number: f32) -> f32 {
    number.abs()
}

#[pyfunction]
pub fn abs_f64(number: f64) -> f64 {
    number.abs()
}

#[pyfunction]
pub fn abs_i128(number: i128) -> i128 {
    number.abs()
}

#[pyfunction]
pub fn abs_complex(real_number:f64,imag_number:f64) -> f64 {
    Complex::new(real_number,imag_number).norm()
}
