use pyo3::pyfunction;

#[pyfunction]
pub fn bin(number: i128, param: bool) -> String {
    if param {
        if number < 0 {
            format!("-0b{:b}", -number)
        } else {
            format!("0b{:b}", number)
        }
    } else {
        if number < 0 {
            format!("-{:b}", -number)
        } else {
            format!("{:b}", number)
        }
    }
}
