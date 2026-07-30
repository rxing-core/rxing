use std::fmt;

use crate::Exceptions;
use crate::common::Result;

use super::{GenericGFPoly, GenericGFRef};

/**
 * <p>This class contains utility methods for performing mathematical operations over
 * the Galois Fields. Operations use a given primitive polynomial in calculations.</p>
 *
 * <p>Throughout this package, elements of the GF are represented as an {@code int}
 * for convenience and speed (but at the cost of memory).
 * </p>
 *
 * @author Sean Owen
 * @author David Olivier
 */
/// Largest field size among the predefined [`super::PredefinedGenericGF`] variants
/// (`AztecData12`, GF(4096)). `expTable`/`logTable` are allocated at this fixed size so
/// `GenericGF::new` can be a `const fn`; fields smaller than this (e.g. `AztecParam`,
/// GF(16)) leave the tail of each table unused. Raise this if a predefined field with a
/// larger size is ever added.
pub(crate) const MAX_GF_SIZE: usize = 4096;

#[derive(Debug, PartialEq, Eq)]
pub struct GenericGF {
    expTable: [i32; MAX_GF_SIZE],
    logTable: [i32; MAX_GF_SIZE],
    // zero: Box<GenericGFPoly>,
    // one: Box<GenericGFPoly>,
    size: usize,
    primitive: i32,
    generatorBase: i32,
}

impl GenericGF {
    /**
     * Create a representation of GF(size) using the given primitive polynomial.
     *
     * @param primitive irreducible polynomial whose coefficients are represented by
     *  the bits of an int, where the least-significant bit represents the constant
     *  coefficient
     * @param size the size of the field
     * @param b the factor b in the generator polynomial can be 0- or 1-based
     *  (g(x) = (x+a^b)(x+a^(b+1))...(x+a^(b+2t-1))).
     *  In most cases it should be 1, but for QR code it is 0.
     */
    pub const fn new(primitive: i32, size: usize, b: i32) -> Self {
        debug_assert!(
            size <= MAX_GF_SIZE,
            "GenericGF field size exceeds MAX_GF_SIZE"
        );
        let mut expTable = [0; MAX_GF_SIZE];
        let mut logTable = [0; MAX_GF_SIZE];
        let mut x = 1;

        let mut i = 0;
        while i < size {
            expTable[i] = x;
            x *= 2; // we're assuming the generator alpha is 2
            if x >= size as i32 {
                x ^= primitive;
                let sz_m_1: i32 = size as i32 - 1;
                x &= sz_m_1;
            }
            i += 1;
        }
        let mut i = 0;
        while i < size - 1 {
            let loc = expTable[i] as usize;
            logTable[loc] = i as i32;

            i += 1;
        }
        logTable[0] = 0;

        Self {
            expTable,
            logTable,
            size,
            primitive,
            generatorBase: b,
        }
    }

    /**
     * @return the monomial representing coefficient * x^degree
     */
    pub fn buildMonomial(source: GenericGFRef, degree: usize, coefficient: i32) -> GenericGFPoly {
        if coefficient == 0 {
            return GenericGFPoly::new(source, &[0]).unwrap();
        }
        let mut coefficients = vec![0; degree + 1];
        coefficients[0] = coefficient;
        GenericGFPoly::new(source, &coefficients).unwrap()
    }

    /**
     * Implements both addition and subtraction -- they are the same in GF(size).
     *
     * @return sum/difference of a and b
     */
    pub const fn addOrSubtract(a: i32, b: i32) -> i32 {
        a ^ b
    }

    /**
     * @return 2 to the power of a in GF(size)
     */
    pub const fn exp(&self, a: i32) -> i32 {
        debug_assert!((a as usize) < self.size, "GF element out of range");
        self.expTable[a as usize]
    }

    /**
     * @return base 2 log of a in GF(size)
     */
    pub const fn log(&self, a: i32) -> Result<i32> {
        if a == 0 {
            return Err(Exceptions::ILLEGAL_ARGUMENT);
        }
        if (a as usize) >= self.size {
            return Err(Exceptions::ILLEGAL_ARGUMENT);
        }
        Ok(self.logTable[a as usize])
    }

    /**
     * @return multiplicative inverse of a
     */
    pub const fn inverse(&self, a: i32) -> Result<i32> {
        if a == 0 {
            return Err(Exceptions::ARITHMETIC);
        }
        let log_t_loc: usize = a as usize;
        if log_t_loc >= self.size {
            return Err(Exceptions::ILLEGAL_ARGUMENT);
        }
        let loc: usize = ((self.size as i32) - self.logTable[log_t_loc] - 1) as usize;
        Ok(self.expTable[loc])
    }

    /**
     * @return product of a and b in GF(size)
     */
    pub const fn multiply(&self, a: i32, b: i32) -> i32 {
        if a == 0 || b == 0 {
            return 0;
        }
        let a_loc: usize = a as usize;
        let b_loc: usize = b as usize;
        debug_assert!(
            a_loc < self.size && b_loc < self.size,
            "GF element out of range"
        );
        let comb_loc: usize = (self.logTable[a_loc] + self.logTable[b_loc]) as usize;
        self.expTable[comb_loc % (self.size - 1)]
    }

    pub const fn getSize(&self) -> usize {
        self.size
    }

    pub const fn getGeneratorBase(&self) -> i32 {
        self.generatorBase
    }
}

impl fmt::Display for GenericGF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GF({:#06x},{}", self.primitive, self.size)
    }
}
