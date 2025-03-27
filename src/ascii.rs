use pyo3::pyfunction;


#[pyfunction]
pub fn ascii(s: &str) -> String {
    s.chars().map(|c| {
        if c.is_ascii() {
            c.to_string()
        } else {
            format!("\\u{{{:04x}}}", c as u32)
        }
    }).collect()
}
