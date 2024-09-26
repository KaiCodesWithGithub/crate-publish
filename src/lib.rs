//! A crate to calculate the fibonacci sequence
//! 
//! This crate is used to demonstrate how to publish a crate to crates.io

pub use self::fibonacci::Fibonacci;

// Calculate the fibonacci sequence for a given number of times
pub mod fibonacci {
    use std::fmt;

    /// Calculate the fibonacci sequence for a given number of times
    pub struct Fibonacci {
        /// The number of times to calculate the fibonacci sequence
        times: usize,
    }

    impl Fibonacci {
        /// Create a new Fibonacci struct
        /// 
        /// # Arguments
        /// 
        /// * `times` - The number of times to calculate the fibonacci sequence
        /// 
        /// # Returns
        /// 
        /// A new Fibonacci struct
        /// 
        /// # Example
        /// 
        /// ```
        /// use kct_crate_publish::fibonacci::Fibonacci;
        /// 
        /// let fibonacci = Fibonacci::new(10);
        /// println!("{}", fibonacci);
        /// assert_eq!(fibonacci.result(), 55);
        /// ```
        pub fn new(times: usize) -> Self {
            Self { times }
        }

        /// Return the result of the fibonacci sequence
        /// 
        /// # Returns
        /// 
        /// The result of the fibonacci sequence
        /// 
        /// # Example
        /// 
        /// ```
        /// use kct_crate_publish::fibonacci::Fibonacci;
        /// 
        /// let fibonacci = Fibonacci::new(10);
        /// println!("{}", fibonacci);
        /// assert_eq!(fibonacci.result(), 55);
        /// ```
        pub fn result(&self) -> usize {
            let sequence = self.calculate_sequence();
            *sequence.last().unwrap()
        }

        fn calculate_sequence(&self) -> Vec<usize> {
            let mut sequence = vec![0, 1];
            for _ in 0..self.times {
                let next = sequence.last().unwrap() + sequence.last().unwrap();
                sequence.push(next);
            }
            sequence
        }
    }

    impl fmt::Display for Fibonacci {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let sequence = self.calculate_sequence();
            let sequence_str = sequence.last().unwrap().to_string();
            write!(f, "{}", sequence_str)
        }
    }
}