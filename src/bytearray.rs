// src/lib.rs
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyByteArray};
use pyo3::exceptions::PyTypeError;

#[derive(Debug, Clone)]
#[pyclass]
pub struct ByteArray {
    #[pyo3(get)]  // 暴露为只读属性
    data: Vec<u8>,
}

#[pymethods]
impl ByteArray {
    #[new]
    #[pyo3(signature = (py_data))]
    fn new(_py: Python, py_data: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(py_bytes) = py_data.downcast::<PyBytes>() {
            Ok(Self {
                data: py_bytes.as_bytes().to_vec()
            })
        } else if let Ok(s) = py_data.extract::<String>() {
            Ok(Self {
                data: s.into_bytes()
            })
        } else if let Ok(bytearray) = py_data.downcast::<PyByteArray>() {
            let data = unsafe { bytearray.as_bytes() };
            Ok(Self {
                data: data.to_vec()
            })
        } else {
            Err(PyTypeError::new_err("Unsupported input type. Must be bytes, bytearray or str"))
        }
    }

    /// 转换为Python bytes对象
    fn to_bytes(&self, py: Python) -> Py<PyBytes> {
        PyBytes::new(py, &self.data).into()
    }

    /// 转换为Python bytearray
    fn to_bytearray(&self, py: Python) -> Py<PyByteArray> {
        PyByteArray::new(py, &self.data).into()
    }

    /// 实现__repr__
    #[pyo3(name = "__repr__")]
    fn __repr__(&self) -> PyResult<String> {
        Ok(match String::from_utf8(self.data.clone()) {
            Ok(s) => format!("ByteArray('{}')", s),
            Err(_) => format!("ByteArray({:?})", self.data),
        })
    }

    /// 实现__str__
    #[pyo3(name = "__str__")]
    fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }

    /// 实现__len__
    fn __len__(&self) -> usize {
        self.data.len()
    }

    /// 实现__getitem__
    fn __getitem__(&self, idx: isize) -> PyResult<u8> {
        let len = self.data.len() as isize;
        if idx < -len || idx >= len {
            return Err(PyTypeError::new_err("Index out of range"));
        }

        let idx = if idx < 0 {
            (len + idx) as usize
        } else {
            idx as usize
        };

        Ok(self.data[idx])
    }

    /// 实现__add__
    fn __add__(&self, other: &Self) -> Self {
        let mut new_data = self.data.clone();
        new_data.extend(&other.data);
        Self { data: new_data }
    }

    /// 追加数据
    fn append(&mut self, other: &Self) {
        self.data.extend(&other.data);
    }

    /// 支持切片
    fn slice(&self, start: Option<isize>, end: Option<isize>, step: Option<isize>) -> PyResult<Self> {
        let len = self.data.len() as isize;
        let step = step.unwrap_or(1);

        if step == 0 {
            return Err(PyTypeError::new_err("Slice step cannot be zero"));
        }

        let (start, end) = py_slice_indices(start, end, step, len)?;

        let mut result = Vec::new();
        let mut i = start;
        while (step > 0 && i < end) || (step < 0 && i > end) {
            result.push(self.data[i as usize]);
            i += step;
        }

        Ok(Self { data: result })
    }
}

/// 实现Python切片索引计算
fn py_slice_indices(
    start: Option<isize>,
    end: Option<isize>,
    step: isize,
    len: isize,
) -> PyResult<(isize, isize)> {
    let (mut start, mut end) = match step.signum() {
        1 => {
            let start = start.unwrap_or(0);
            let end = end.unwrap_or(len);
            (start.clamp(0, len), end.clamp(0, len))
        }
        -1 => {
            let start = start.unwrap_or(len - 1);
            let end = end.unwrap_or(-1);
            (start.clamp(-1, len - 1), end.clamp(-1, len - 1))
        }
        _ => unreachable!(),
    };

    // 处理负索引
    if start < 0 {
        start += len;
    }
    if end < 0 {
        end += len;
    }

    Ok((start, end))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::Python;

    #[test]
    fn test_from_bytes() {
        Python::with_gil(|py| {
            let bytes = PyBytes::new(py, b"hello");
            let ba = ByteArray::new(py, &bytes).unwrap();
            assert_eq!(ba.data, b"hello");
        });
    }

    #[test]
    fn test_from_str() {
        Python::with_gil(|py| {
            let s = py.eval_bound("'hello'", None, None).unwrap();
            let ba = ByteArray::new(py, &s).unwrap();
            assert_eq!(ba.data, b"hello");
        });
    }

    #[test]
    fn test_to_bytes() {
        Python::with_gil(|py| {
            let ba = ByteArray { data: b"hello".to_vec() };
            let py_bytes = ba.to_bytes(py);
            assert_eq!(py_bytes.as_bytes(py), b"hello");
        });
    }

    #[test]
    fn test_len() {
        let ba = ByteArray { data: b"hello".to_vec() };
        assert_eq!(ba.__len__(), 5);
    }

    #[test]
    fn test_getitem() {
        let ba = ByteArray { data: b"hello".to_vec() };
        assert_eq!(ba.__getitem__(0).unwrap(), b'h');
        assert_eq!(ba.__getitem__(-1).unwrap(), b'o');
    }

    #[test]
    fn test_add() {
        let ba1 = ByteArray { data: b"hel".to_vec() };
        let ba2 = ByteArray { data: b"lo".to_vec() };
        let result = ba1.__add__(&ba2);
        assert_eq!(result.data, b"hello");
    }

    #[test]
    fn test_slice() {
        let ba = ByteArray { data: b"hello".to_vec() };
        let sliced = ba.slice(Some(1), Some(4), None).unwrap();
        assert_eq!(sliced.data, b"ell");
    }
}
