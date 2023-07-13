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

/// A wrapper class for the Top-K algorithm.
#[pyclass]
struct TopK {
    inner: streaming_algorithms::Top<TKPyHashable, u64>,
}

#[pymethods]
impl TopK {
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
    /// Pushes a key and a count number to the total counted set.
    fn push(&mut self, item: TKPyHashable, count: u64) {
        self.inner.push(item, &count);
    }
    /// Returns the top n counted items from the set.
    fn top(&self) -> Vec<(TKPyHashable, u64)> {
        self.inner
            .iter()
            .map(|(key, count)| (key.clone(), count.clone()))
            .collect()
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

/// A wrapper class for a simple random sample algorithm.
///
/// Takes a population size and sample size, then randomly removes elements and returns true if they are within the sample.
#[pyclass]
struct SimpleRandomSample {
    inner: streaming_algorithms::SampleTotal,
    rng: StdRng,
}

#[pymethods]
impl SimpleRandomSample {
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
    /// Picks a random element from the set, then returns true if within sample.
    fn sample(&mut self) -> bool {
        self.inner.sample(&mut self.rng)
    }
}

/// A wrapper class for an implementation of reservoir sampling.
///
/// Holds a reservoir which fills with items as they are added to the stream. The order of items in the reservoir is unstable.
#[pyclass]
struct UnstableReservoirSample {
    inner: streaming_algorithms::SampleUnstable<Py<PyAny>>,
    rng: StdRng,
}

#[pymethods]
impl UnstableReservoirSample {
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
    /// Pushes a new item into the population.
    fn push(&mut self, t: Py<PyAny>) {
        self.inner.push(t, &mut self.rng)
    }
    /// Returns the current reservoir as a list.
    fn reservoir(&mut self) -> Vec<Py<PyAny>> {
        self.inner.clone().into_iter().collect()
    }
}

/// An implementation of the CountMinSketch data structure.
/// 
/// Also referred to as the counting Bloom filter, keeps approximate track of values associated with keys while using sub-linear space.
#[pyclass]
struct CountMinSketch {
    inner: streaming_algorithms::CountMinSketch<TKPyHashable, u64>,
}

#[pymethods]
impl CountMinSketch {
    /// Initializes instance with probability and error tolerance.
    /// 
    /// Args:
    ///     probability: TODO
    ///     tolerance: Accepted error tolerance. TODO
    #[new]
    fn new(probability: f64, tolerance: f64) -> Self {
        Self {
            inner: streaming_algorithms::CountMinSketch::new(probability, tolerance, ()),
        }
    }
    /// Pushes a key/value pair to the total counted set.
    fn push(&mut self, key: TKPyHashable, value: u64) -> u64 {
        self.inner.push(&key, &value)
    }
    /// Unions the aggregated value for a key with another value.
    fn union_assign(&mut self, key: TKPyHashable, value: u64) {
        self.inner.union_assign(&key, &value);
    }
    /// Gets the estimate aggregate value for a specified key.
    fn get(&self, key: TKPyHashable) -> u64 {
        self.inner.get(&key)
    }
    /// Empties the set of the CountMinSketch object.
    fn clear(&mut self) {
        self.inner.clear();
    }
}

#[pymodule]
fn top_kat(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<HyperLogLog>()?;
    m.add_class::<TopK>()?;
    m.add_class::<SimpleRandomSample>()?;
    m.add_class::<UnstableReservoirSample>()?;
    m.add_class::<CountMinSketch>()?;
    Ok(())
}
