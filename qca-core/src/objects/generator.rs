use std::marker::PhantomData;

/// Configuration for different generators
pub trait GeneratorConfig: Clone {}

/// A trait for value generators that produce an iterator of values
pub trait Generator {
    /// The configuration type for this generator
    type Config: GeneratorConfig;
    /// The output type produced by this generator
    type Output;

    /// Create a new generator with the given configuration and sample count
    fn new(config: Self::Config, num_samples: usize) -> Self;

    /// Generate a single value at the given sample index
    fn generate(&self, sample: usize) -> Self::Output;

    /// Create an iterator that produces all values from 0 to num_samples
    fn iter(&self) -> GeneratorIterator<Self>
    where
        Self: Sized,
    {
        GeneratorIterator {
            generator: self,
            current: 0,
            num_samples: self.num_samples(),
            _marker: PhantomData,
        }
    }

    /// Get the total number of samples this generator will produce
    fn num_samples(&self) -> usize;
}

/// An iterator that produces values from a generator
pub struct GeneratorIterator<'a, G: Generator> {
    generator: &'a G,
    current: usize,
    num_samples: usize,
    _marker: PhantomData<G::Output>,
}

impl<'a, G: Generator> Iterator for GeneratorIterator<'a, G> {
    type Item = G::Output;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.num_samples {
            let sample = self.current;
            self.current += 1;
            Some(self.generator.generate(sample))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.num_samples - self.current;
        (remaining, Some(remaining))
    }
}
