mod abs;
mod impls;
mod ascii;
mod Number_system;
mod bytearray;

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
// fn Standard_PyOxidizer(m: &Bound<'_, PyModule>) -> PyResult<()> {
fn Standard_PyOxidizer(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(abs::abs_i32, m)?)?;
    m.add_function(wrap_pyfunction!(abs::abs_i64, m)?)?;
    m.add_function(wrap_pyfunction!(abs::abs_i128, m)?)?;
    m.add_function(wrap_pyfunction!(abs::abs_f32, m)?)?;
    m.add_function(wrap_pyfunction!(abs::abs_f64, m)?)?;
    m.add_function(wrap_pyfunction!(abs::abs_complex, m)?)?;
    m.add_function(wrap_pyfunction!(ascii::ascii, m)?)?;
    m.add_function(wrap_pyfunction!(Number_system::bin, m)?)?;
    m.add_class::<bytearray::ByteArray>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex;

    #[test]
    fn abs_i32_test() {
        assert_eq!(abs::abs_i32(-1), 1);
    }

    #[test]
    fn abs_i64_test() {
        assert_eq!(abs::abs_i64(-1), 1);
    }

    #[test]
    fn abs_f32_test() {
        assert_eq!(abs::abs_f32(-1.0), 1.0);
    }

    #[test]
    fn abs_f64_test() {
        assert_eq!(abs::abs_f64(-1.0), 1.0);
    }

    #[test]
    fn abs_complex_test() {
        assert_eq!(abs::abs_complex(-1.0,-1.0), Complex::new(1.0,1.0).norm());
    }
    #[test]
    fn ascii_test() {
        assert_eq!(ascii::ascii("hello"), "hello".escape_debug().to_string());
    }
    #[test]
    fn bin_test() {
        assert_eq!(Number_system::bin(10,true), "0b10".to_string());
    }
}
