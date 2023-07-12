use pyo3::prelude::*;
use pyo3::exceptions::PyTypeError;
use streaming_algorithms;
use rand::rngs::StdRng;
use rand::SeedableRng;

#[derive(Hash, Clone, PartialEq, Eq)]
enum TKPyHashable {
    TKPyInt(i32),
    TKPyString(String),
    TKPyBool(bool),
    TKPyBytes(Vec<u8>),
}

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
    /// Adds a new element to the set.
    fn push(&mut self, value: TKPyHashable) {
        self.inner.push(&value);
    }
    /// Returns the approximate cardinality of the set as a float.
    fn len(&self) -> f64 {
        self.inner.len()
    }
    /// Returns a boolean representing whether the set is empty.
    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    /// Merges a second HyperLogLog object into this one.
    /// 
    /// Modifies the HyperLogLog it's called on such that its cardinality approximates the cardinality of the combination of its set and the second HyperLogLog's set.
    /// 
    /// Args:
    ///     src: A different HyperLogLog instance.
    fn union(&mut self, src: &Self) {
        self.inner.union(&src.inner);
    }
    /// Intersects a second HyperLogLog object into this one.
    /// 
    /// TODO
    fn intersect(&mut self, src: &Self) {
        self.inner.intersect(&src.inner);
    }
    /// Empties the set of the HyperLogLog object.
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
    /// Adds a new element to the set
    /// 
    /// Pushes a key and a count number to the total counted set.
    /// 
    /// Args:
    ///     item: A Python object representing the key to be pushed.
    ///     value: The count of items to be pushed.
    fn push(&mut self, item: TKPyHashable, value: u64) {
        self.inner.push(item, &value);
    }
    /// Returns the top n counted items from the set.
    fn top(&self) -> Vec<(TKPyHashable, u64)> {
        self.inner.iter().map(|(key, count)| (key.clone(), count.clone())).collect()
    }
    /// Returns the capacity of the TopK object.
    fn capacity(&self) -> usize {
        self.inner.capacity()
    }
    /// Empties the set of the TopK object.
    fn clear(&mut self) {
        self.inner.clear();
    }
}

#[pyclass]
struct SampleTotal {
    inner: streaming_algorithms::SampleTotal,
    rng: StdRng,
}

#[pymethods]
impl SampleTotal {
    #[new]
    fn new(total: usize, samples: usize, seed: Option<u64>) -> Self {
        Self {
            inner: streaming_algorithms::SampleTotal::new(total, samples),
            rng: if seed.is_some() {StdRng::seed_from_u64(seed.unwrap_or(0))} else {StdRng::from_entropy()}
        }
    }
    fn sample(&mut self) -> bool {
        self.inner.sample(&mut self.rng)
    }
}

#[pyclass]
struct SampleUnstable {
    inner: streaming_algorithms::SampleUnstable<Py<PyAny>>,
    rng: StdRng,
}

#[pymethods]
impl SampleUnstable {
    #[new]
    fn new(samples: usize, seed: Option<u64>) -> Self {
        Self {
            inner: streaming_algorithms::SampleUnstable::new(samples),
            rng: if seed.is_some() {StdRng::seed_from_u64(seed.unwrap_or(0))} else {StdRng::from_entropy()}
        }
    }
    fn push(&mut self, t: Py<PyAny>) {
        self.inner.push(t, &mut self.rng)
    }
    fn reservoir(&mut self) -> Vec<Py<PyAny>> {
        self.inner.clone().into_iter().collect()
    }
}

#[pymodule]
fn top_kat(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<HyperLogLog>()?;
    m.add_class::<TopK>()?;
    m.add_class::<SampleTotal>()?;
    m.add_class::<SampleUnstable>()?;
    Ok(())
}
