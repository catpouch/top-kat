use pyo3::prelude::*;
use pyo3::exceptions::PyTypeError;
use streaming_algorithms;
// use streaming_algorithms::New;

#[derive(Hash, Clone, PartialEq, Eq)]
enum TKPyHashable {
    TKPyInt(i32),
    TKPyString(String),
    TKPyBool(bool),
    TKPyBytes(Vec<u8>),
}

// #[derive(New)] //oops!
// enum TKPyIntersectable {
//     TKPyU64(u64),
//     TKPyUSize(usize)
// }

impl FromPyObject<'_> for TKPyHashable {
    fn extract(ob: &'_ PyAny) -> PyResult<Self> {
        if let Ok(val) = ob.extract::<i32>() {
            return Ok(TKPyHashable::TKPyInt(val));
        } if let Ok(val) = ob.extract::<String>() {
            return Ok(TKPyHashable::TKPyString(val));
        } if let Ok(val) = ob.extract::<bool>() {
            return Ok(TKPyHashable::TKPyBool(val));
        } if let Ok(val) = ob.extract::<Vec<u8>>() {
            return Ok(TKPyHashable::TKPyBytes(val));
        } else {
            return Err(PyTypeError::new_err("HyperLogLog only takes strings or ints!"));
        }
    }
}

impl IntoPy<PyObject> for TKPyHashable {
    fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            TKPyHashable::TKPyInt(val) => val.into_py(py),
            TKPyHashable::TKPyString(val) => val.into_py(py),
            TKPyHashable::TKPyBool(val) => val.into_py(py),
            TKPyHashable::TKPyBytes(val) => val.into_py(py),
        }
    }
}

#[pyclass]
struct HyperLogLog {
    inner: streaming_algorithms::HyperLogLog<TKPyHashable>,
}

#[pymethods]
impl HyperLogLog {
    #[new]
    fn new(error_rate: f64) -> Self {
        Self {
            inner: streaming_algorithms::HyperLogLog::new(error_rate),
        }
    }
    fn push(&mut self, value: TKPyHashable) {
        self.inner.push(&value);
    }
    fn len(&self) -> f64 {
        self.inner.len()
    }
    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    fn union(&mut self, src: &Self) {
        self.inner.union(&src.inner);
    }
    fn intersect(&mut self, src: &Self) {
        self.inner.intersect(&src.inner);
    }
    fn clear(&mut self) {
        self.inner.clear();
    }
}

#[pyclass]
struct TopK {
    inner: streaming_algorithms::Top<TKPyHashable, u64>,
}

#[pymethods]
impl TopK {
    #[new]
    fn new(n: usize, probability: f64, tolerance: f64) -> Self {
        Self {
            inner: streaming_algorithms::Top::new(n, probability, tolerance, ())
        }
    }
    fn push(&mut self, item: TKPyHashable, value: u64) {
        self.inner.push(item, &value);
    }
    fn top(&self) -> Vec<(TKPyHashable, u64)> {
        self.inner.iter().map(|(key, count)| (key.clone(), count.clone())).collect()
    }
    fn capacity(&self) -> usize {
        self.inner.capacity()
    }
    fn clear(&mut self) {
        self.inner.clear();
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn top_kat(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<HyperLogLog>()?;
    m.add_class::<TopK>()?;
    Ok(())
}
