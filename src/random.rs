/* Millington:
    Keeps track of one random stream: i.e. a seed and its output.
    This is used to get random numbers. Rather than a function, this allows there to be several
    streams of repeatable random numbers at teh same time. Uses the RandRotB algorithm.
 */

use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::Vector3;
use crate::precision::Real;


// This is translation of Millington's cyclone::Random
// Keeps track of one random stream (seed & current), so we can have multiple independent,
//      repeatable streams.

#[derive(Debug, Clone)]
pub struct Random {
    p1: usize,
    p2: usize,
    buffer: [u32; 17],
}

impl Random {
    // Creates a new random number stream with a seed based on timing data
    pub fn new() -> Self {
        let mut rng = Self {
            p1: 0,
            p2: 10,
            buffer: [0; 17],
        };
        rng.seed(0);
        rng
    }
    // Creats a new random stream with the given seed.
    pub fn with_seed(seed: u32) -> Self {
        let mut rng = Self {
            p1: 0,
            p2: 10,
            buffer: [0; 17],
        };
        rng.seed(seed);
        rng
    }

    // Left bitwise rotation (rotl).
    #[inline]
    fn rotl(n: u32, r: u32) -> u32 {
        n.rotate_left(r)
    }

    // Right bitwise rotation (rotr).
    #[allow(dead_code)]
    fn rotr(n: u32, r: u32) -> u32 {
        n.rotate_right(r)
    }

    // Sets the seed value for the random stream.
    // If 'seed == 0', uses timing data as entropy (similar to 'clock()').
    pub fn seed(&mut self, mut seed: u32) {
        if seed == 0 {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default();
            // Use subsec_nanos as a simple stand-in for 'clock()'
            seed = now.subsec_nanos();
        }

        // Fill the buffer with some basic random numbers
        for i in 0..17 {
            //Simple linear congruential generator, same as Millington: s = s * 2891336453 + 1
            seed = seed.wrapping_mul(2891336453u32).wrapping_add(1);
            self.buffer[i] = seed;
        }

        // Initialize pointers into the buffer
        self.p1 = 0;
        self.p2 = 10;
    }

    // Returns the next random bitstring from the stream; matches Millington's 'randomBits()'
    pub fn random_bits (&mut self) -> u32 {
        // Rotate the buffer and store it back to itself
        let result = {
            let v1 = self.buffer[self.p2];
            let v2 = self.buffer[self.p1];

            // Use wrapping_add to match C++ unsigned overflow behavior
            let new_val = Self::rotl(v1, 13).wrapping_add(Self::rotl(v2, 9));

            self.buffer[self.p1] = new_val;
            new_val
        };

        // Rotate pointers (wrap 0..=16)
        if self.p1 == 0 {
            self.p1 = 16;
        } else {
            self.p1 -= 1;
        }

        if self.p2 == 0 {
            self.p2 = 16;
        } else {
            self.p2 -= 1;
        }

        result
    }

    // Returns a random floating number in [0,1).
    // This follows the same IEEE754 tick as Millington's SINGLE_PRECISION path.
    pub fn random_real(&mut self) -> Real {
        let bits = self.random_bits();

        // Construct a float in [1,2) by fixing sign/exponent and using bits as the fraction
        //  then subtract 1 to get [0,1).
        let word = (bits >> 9) | 0x3f80_0000; //0x3f800000 is 1.0f
        let value = f32::from_bits(word);
        value - 1.0
    }

    // Returns a random floating point number between 0 and 'scale'.
    pub fn random_real_scale(&mut self, scale: Real) -> Real {
        self.random_real() * scale

    }

    // Returns a random floating point number between 'min' and 'max'
    pub fn random_real_range(&mut self, min: Real, max: Real) -> Real {
        self.random_real() * (max - min) + min
    }

    // Returns a random integer in [0, max).
    pub fn random_in(&mut self, max: u32) -> u32 {
        self.random_bits() % max
    }

    // Returns a random binomially distributed number between -scale and +scale
    pub fn random_binomial (&mut self, scale: Real) -> Real {
        (self.random_real() - self.random_real()) * scale
    }

    // Returns a random vector where each component is binomially distributed in (-scale, scale),
    // mean 0.
    pub fn random_vector_scale(&mut self, scale: Real) -> Vector3 {
        Vector3::new(
            self.random_binomial(scale),
            self.random_binomial(scale),
            self.random_binomial(scale),
        )
    }

    // Returns a random vector where each component is binomially distributed in (-scale_i, scale_i)
    //  for each component of 'scale'.
    pub fn random_vector_component_scale(&mut self, scale: &Vector3) -> Vector3 {
        Vector3::new(
            self.random_binomial(scale.x),
            self.random_binomial(scale.y),
            self.random_binomial(scale.z),
        )
    }

    // Returns a random vector in the axis-aligned box [min, max], uniformly distributed.
    pub fn random_vector_range(&mut self, min: &Vector3, max: &Vector3) -> Vector3 {
        Vector3::new(
            self.random_real_range(min.x, max.x),
            self.random_real_range(min.y, max.y),
            self.random_real_range(min.z, max.z),
        )
    }

    // Returns a random XZ vector where x,z are binomial in (-scale, scale) and y = 0.
    pub fn random_xz_vector(&mut self, scale: Real) -> Vector3 {
        Vector3::new(
            self.random_binomial(scale),
            0.0,
            self.random_binomial(scale),
        )
    }
    // Note to self: randomQuaternion() will be added once Quaternion is defined in core.rs.
}