use pyo3::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::hash::{Hash, Hasher};
use streaming_algorithms;

#[derive(Clone)]
enum TKPyHashable {
    TKPyInt(i32),
    TKPyString(String),
    TKPyBool(bool),
    TKPyBytes(Vec<u8>),
    TKPyNone(),
    TKPyHashed(isize, Py<PyAny>),
}

impl FromPyObject<'_> for TKPyHashable {
    fn extract(ob: &'_ PyAny) -> PyResult<Self> {
        if let Ok(val) = ob.extract() {
            Ok(TKPyHashable::TKPyInt(val))
        } else if let Ok(val) = ob.extract() {
            Ok(TKPyHashable::TKPyString(val))
        } else if let Ok(val) = ob.extract() {
            Ok(TKPyHashable::TKPyBool(val))
        } else if let Ok(val) = ob.extract() {
            Ok(TKPyHashable::TKPyBytes(val))
        } else if ob.is_none() {
            Ok(TKPyHashable::TKPyNone())
        } else {
            Ok(TKPyHashable::TKPyHashed(ob.hash()?, ob.into()))
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
            TKPyHashable::TKPyNone() => py.None(),
            TKPyHashable::TKPyHashed(_, ob) => ob,
        }
    }
}

impl Hash for TKPyHashable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            TKPyHashable::TKPyInt(val) => val.hash(state),
            TKPyHashable::TKPyString(val) => val.hash(state),
            TKPyHashable::TKPyBool(val) => val.hash(state),
            TKPyHashable::TKPyBytes(val) => val.hash(state),
            TKPyHashable::TKPyNone() => ().hash(state),
            TKPyHashable::TKPyHashed(val, _) => val.hash(state),
        }
    }
}

impl PartialEq for TKPyHashable {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TKPyHashable::TKPyInt(self_val), TKPyHashable::TKPyInt(other_val)) => {
                self_val == other_val
            }
            (TKPyHashable::TKPyString(self_val), TKPyHashable::TKPyString(other_val)) => {
                self_val == other_val
            }
            (TKPyHashable::TKPyBool(self_val), TKPyHashable::TKPyBool(other_val)) => {
                self_val == other_val
            }
            (TKPyHashable::TKPyBytes(self_val), TKPyHashable::TKPyBytes(other_val)) => {
                self_val == other_val
            }
            (TKPyHashable::TKPyNone(), TKPyHashable::TKPyNone()) => true,
            (TKPyHashable::TKPyHashed(self_val, _), TKPyHashable::TKPyHashed(other_val, _)) => {
                self_val == other_val
            }
            _ => false,
        }
    }
}

impl Eq for TKPyHashable {}

/// A wrapper class for the HyperLogLog algorithm.
#[pyclass]
struct HyperLogLog {
    inner: streaming_algorithms::HyperLogLog<TKPyHashable>,
}

#[pymethods]
impl HyperLogLog {
    /// __init__(error_rate)
    /// --
    ///
    /// Initializes instance with specified error tolerance.
    ///
    /// Args:
    ///     error_rate: Accepted error tolerance. Only very specific values seem to work, more documentation necessary.
    #[new]
    fn new(error_rate: f64) -> Self {
        Self {
            inner: streaming_algorithms::HyperLogLog::new(error_rate),
        }
    }
    /// push(value)
    /// --
    ///
    /// Adds a new element to the set.
    fn push(&mut self, value: TKPyHashable) {
        self.inner.push(&value);
    }
    /// len()
    /// --
    ///
    /// Returns the approximate cardinality of the set as a float.
    fn len(&self) -> f64 {
        self.inner.len()
    }
    /// Returns a boolean representing whether the set is empty.
    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    /// union(src)
    /// --
    ///
    /// Merges a second HyperLogLog object into this one.
    ///
    /// Modifies the HyperLogLog it's called on such that its cardinality approximates the cardinality of the combination of its set and the second HyperLogLog's set.
    ///
    /// Args:
    ///     src: A different HyperLogLog instance.
    fn union(&mut self, src: &Self) {
        self.inner.union(&src.inner);
    }
    /// intersect(src)
    /// --
    ///
    /// Intersects a second HyperLogLog object into this one.
    ///
    /// TODO
    fn intersect(&mut self, src: &Self) {
        self.inner.intersect(&src.inner);
    }
    /// clear()
    /// --
    ///
    /// Empties the set of the HyperLogLog object.
    fn clear(&mut self) {
        self.inner.clear();
    }
}

/// A wrapper class for the Top-K algorithm.
#[pyclass]
struct TopK {
    inner: streaming_algorithms::Top<TKPyHashable, u64>,
}

#[pymethods]
impl TopK {
    /// __init__(k, probability, tolerance)
    /// --
    ///
    /// Initializes instance with size k, probability, and tolerance. More documentation needed.
    ///
    /// Args:
    ///     k: Size of top values set to keep track of.
    ///     probability: TODO
    ///     tolerance: TODO
    #[new]
    fn new(k: usize, probability: f64, tolerance: f64) -> Self {
        Self {
            inner: streaming_algorithms::Top::new(k, probability, tolerance, ()),
        }
    }
    /// push(item, count)
    /// --
    ///
    /// Adds a new element to the set.
    ///
    /// Pushes a key and a count number to the total counted set.
    ///
    /// Args:
    ///     item: A Python object representing the key to be pushed.
    ///     count: The count of items to be pushed.
    fn push(&mut self, item: TKPyHashable, count: u64) {
        self.inner.push(item, &count);
    }
    /// top()
    /// --
    ///
    /// Returns the top n counted items from the set.
    fn top(&self) -> Vec<(TKPyHashable, u64)> {
        self.inner
            .iter()
            .map(|(key, count)| (key.clone(), count.clone()))
            .collect()
    }
    /// capacity()
    /// --
    ///
    /// Returns the capacity of the TopK object.
    fn capacity(&self) -> usize {
        self.inner.capacity()
    }
    /// clear()
    /// --
    ///
    /// Empties the set of the TopK object.
    fn clear(&mut self) {
        self.inner.clear();
    }
}

/// Given population and sample sizes, returns true if this element is in the sample. Without replacement.
///
/// Needs clarification, possible removal? Doesn't seem too complicated to just implement on your own.
#[pyclass]
struct SampleTotal {
    inner: streaming_algorithms::SampleTotal,
    rng: StdRng,
}

#[pymethods]
impl SampleTotal {
    /// __init__(total, samples, seed)
    /// --
    ///
    /// Initializes instance with population size, sample size, and optional random seed.
    ///
    /// More documentation on args needed.
    ///
    /// Args:
    ///     total: Population size.
    ///     samples: Sample size.
    ///     seed: RNG seed, optional.
    #[new]
    fn new(total: usize, samples: usize, seed: Option<u64>) -> Self {
        let rng: StdRng = if let Some(seed) = seed {
            StdRng::seed_from_u64(seed)
        } else {
            StdRng::from_entropy()
        };
        Self {
            inner: streaming_algorithms::SampleTotal::new(total, samples),
            rng,
        }
    }
    /// sample()
    /// --
    ///
    /// Picks a random element from the set, then returns true if within sample.
    fn sample(&mut self) -> bool {
        self.inner.sample(&mut self.rng)
    }
}

/// A wrapper class for a rolling implementation of reservoir sampling.
#[pyclass]
struct SampleUnstable {
    inner: streaming_algorithms::SampleUnstable<Py<PyAny>>,
    rng: StdRng,
}

#[pymethods]
impl SampleUnstable {
    /// __init__(samples, seed)
    /// --
    ///
    /// Initializes instance with sample size and optional random seed.
    ///
    /// Args:
    ///     samples: Size of random sample set to keep track of.
    ///     seed: RNG seed, optional.
    #[new]
    fn new(samples: usize, seed: Option<u64>) -> Self {
        let rng: StdRng = if let Some(seed) = seed {
            StdRng::seed_from_u64(seed)
        } else {
            StdRng::from_entropy()
        };
        Self {
            inner: streaming_algorithms::SampleUnstable::new(samples),
            rng,
        }
    }
    /// push(t)
    /// --
    ///
    /// Pushes a new item into the population.
    ///
    /// Args:
    ///     t: Item to push.
    fn push(&mut self, t: Py<PyAny>) {
        self.inner.push(t, &mut self.rng)
    }
    /// reservoir()
    /// --
    ///
    /// Returns the current reservoir as a list.
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
