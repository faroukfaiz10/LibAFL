//! Generators may generate bytes or, in general, data, for inputs.

use alloc::vec::Vec;
use core::{cmp::min, marker::PhantomData};

use crate::{
    bolts::rands::Rand,
    inputs::{bytes::BytesInput, Input},
    state::HasRand,
    Error,
};

pub mod gramatron;
pub use gramatron::*;

#[cfg(feature = "nautilus")]
pub mod nautilus;
#[cfg(feature = "nautilus")]
pub use nautilus::*;

/// The maximum size of dummy bytes generated by _dummy generator methods
const DUMMY_BYTES_MAX: usize = 64;

/// Generators can generate ranges of bytes.
pub trait Generator<I, S>
where
    I: Input,
{
    /// Generate a new input
    fn generate(&mut self, state: &mut S) -> Result<I, Error>;

    /// Generate a new dummy input
    fn generate_dummy(&self, state: &mut S) -> I;
}

#[derive(Clone, Debug)]
/// Generates random bytes
pub struct RandBytesGenerator<S>
where
    S: HasRand,
{
    max_size: usize,
    phantom: PhantomData<S>,
}

impl<S> Generator<BytesInput, S> for RandBytesGenerator<S>
where
    S: HasRand,
{
    fn generate(&mut self, state: &mut S) -> Result<BytesInput, Error> {
        let mut size = state.rand_mut().below(self.max_size as u64);
        if size == 0 {
            size = 1;
        }
        let random_bytes: Vec<u8> = (0..size)
            .map(|_| state.rand_mut().below(256) as u8)
            .collect();
        Ok(BytesInput::new(random_bytes))
    }

    /// Generates up to `DUMMY_BYTES_MAX` non-random dummy bytes (0)
    fn generate_dummy(&self, _state: &mut S) -> BytesInput {
        let size = min(self.max_size, DUMMY_BYTES_MAX);
        BytesInput::new(vec![0; size])
    }
}

impl<S> RandBytesGenerator<S>
where
    S: HasRand,
{
    /// Returns a new [`RandBytesGenerator`], generating up to `max_size` random bytes.
    #[must_use]
    pub fn new(max_size: usize) -> Self {
        Self {
            max_size,
            phantom: PhantomData,
        }
    }
}

#[derive(Clone, Debug)]
/// Generates random printable characters
pub struct RandPrintablesGenerator<S>
where
    S: HasRand,
{
    max_size: usize,
    phantom: PhantomData<S>,
}

impl<S> Generator<BytesInput, S> for RandPrintablesGenerator<S>
where
    S: HasRand,
{
    fn generate(&mut self, state: &mut S) -> Result<BytesInput, Error> {
        let mut size = state.rand_mut().below(self.max_size as u64);
        if size == 0 {
            size = 1;
        }
        let printables = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz \t\n!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".as_bytes();
        let random_bytes: Vec<u8> = (0..size)
            .map(|_| *state.rand_mut().choose(printables))
            .collect();
        Ok(BytesInput::new(random_bytes))
    }

    /// Generates up to `DUMMY_BYTES_MAX` non-random dummy bytes (0)
    fn generate_dummy(&self, _state: &mut S) -> BytesInput {
        let size = min(self.max_size, DUMMY_BYTES_MAX);
        BytesInput::new(vec![0_u8; size])
    }
}

impl<S> RandPrintablesGenerator<S>
where
    S: HasRand,
{
    /// Creates a new [`RandPrintablesGenerator`], generating up to `max_size` random printable characters.
    #[must_use]
    pub fn new(max_size: usize) -> Self {
        Self {
            max_size,
            phantom: PhantomData,
        }
    }
}

#[cfg(feature = "python")]
pub mod pybind {
    use pyo3::prelude::*;
    use crate::bolts::rands::StdRand;
    use crate::corpus::{InMemoryCorpus, OnDiskCorpus};
    use crate::feedbacks::MapFeedbackState;
    use crate::inputs::BytesInput;
    use crate::state::StdState;
    use crate::generators::RandPrintablesGenerator;

    #[pyclass(unsendable, name = "RandPrintablesGenerator")]
    pub struct  PythonRandPrintablesGeneratorI32{ 
        pub rand_printable_generator: RandPrintablesGenerator<
            StdRand,
            StdState<
                InMemoryCorpus<BytesInput>, 
                (MapFeedbackState<i32>, ()), 
                BytesInput, 
                StdRand, 
                OnDiskCorpus<BytesInput>
            >,
        >
    }

    #[pymethods]
    impl PythonRandPrintablesGeneratorI32{
        #[new]
        fn new(
            max_size: usize
        ) -> Self{
            Self{
                rand_printable_generator: RandPrintablesGenerator::new(max_size)
            }
        }
    }



    pub fn register(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_class::<PythonRandPrintablesGeneratorI32>()?;
        Ok(())
    }
}