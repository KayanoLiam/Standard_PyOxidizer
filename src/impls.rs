// use num_complex::Complex;
// use pyo3::prelude::*;
// use pyo3::types::PyComplex;
//
// impl<'source> FromPyObject<'source> for Complex<f64> {
//     fn extract(ob: &'source PyAny) -> PyResult<Self> {
//         let real = ob.getattr("real")?.extract()?;
//         let imag = ob.getattr("imag")?.extract()?;
//         Ok(Complex::new(real, imag))
//     }
//
//     fn extract_bound(ob: &Bound<'source, PyAny>) -> PyResult<Self> {
//         self::
//     }
// }