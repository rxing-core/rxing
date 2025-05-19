use std::fmt;

use crate::common::Result;
use crate::Exceptions;

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericGF {
    expTable: Vec<i32>,
    logTable: Vec<i32>,
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
    pub fn new(primitive: i32, size: usize, b: i32) -> Self {
        let mut expTable = vec![0; size];
        let mut logTable = vec![0; size];
        let mut x = 1;
        for expTableEntry in expTable.iter_mut().take(size) {
            // for i in 0..size {
            //for (int i = 0; i < size; i++) {
            //expTable.push(x);
            *expTableEntry = x;
            x *= 2; // we're assuming the generator alpha is 2
            if x >= size as i32 {
                x ^= primitive;
                let sz_m_1: i32 = size as i32 - 1;
                x &= sz_m_1;
            }
        }
        for (i, loc) in expTable.iter().enumerate().take(size - 1) {
            // for i in 0..size - 1 {
            //for (int i = 0; i < size - 1; i++) {
            // let loc: usize = expTable[i] as usize;
            logTable[*loc as usize] = i as i32;
        }
        logTable[0] = 0;

        //     let mut p:u32;
        // //int i;
        // /*Initialize the table of powers of a primtive root, alpha=0x02.*/
        // p = 1;
        // for i in 0..size {
        // // for (i = 0; i < 256; i++) {
        //     expTable[i] = expTable[i + size - 1] = p;
        // p = ((p << 1) ^ (-(p as i32 >> 7) & primitive) as u32) & 0xFF;
        // }
        // /*Invert the table to recover the logs.*/
        // for i in 0..size-1 {
        // // for (i = 0; i < 255; i++)
        // logTable[expTable[i].try_into().unwrap()] = i;
        // /*Note that we rely on the fact that _gf->log[0]=0 below.*/
        Self {
            expTable,
            logTable,
            size,
            primitive,
            generatorBase: b,
        }

        // logTable[0] == 0 but this should never be used
        // new_ggf.zero = Box::new(GenericGFPoly::new(Box::new(new_ggf), &vec![0]).unwrap());
        // new_ggf.one = Box::new(GenericGFPoly::new(Box::new(new_ggf), &vec![1]).unwrap());

        //new_ggf
    }

    // pub fn getZero(&self) -> Box<GenericGFPoly> {
    //     return self.zero;
    // }

    // pub fn getOne(&self) -> Box<GenericGFPoly> {
    //     return self.one;
    // }

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
    pub fn exp(&self, a: i32) -> i32 {
        // let pos: usize = a.try_into().unwrap();
        self.expTable[a as usize]
    }

    /**
     * @return base 2 log of a in GF(size)
     */
    pub fn log(&self, a: i32) -> Result<i32> {
        if a == 0 {
            return Err(Exceptions::ILLEGAL_ARGUMENT);
        }
        // let pos: usize = a.try_into().unwrap();
        Ok(self.logTable[a as usize])
    }

    /**
     * @return multiplicative inverse of a
     */
    pub fn inverse(&self, a: i32) -> Result<i32> {
        if a == 0 {
            return Err(Exceptions::ARITHMETIC);
        }
        let log_t_loc: usize = a as usize;
        let loc: usize = ((self.size as i32) - self.logTable[log_t_loc] - 1) as usize;
        Ok(self.expTable[loc])
    }

    /**
     * @return product of a and b in GF(size)
     */
    pub fn multiply(&self, a: i32, b: i32) -> i32 {
        if a == 0 || b == 0 {
            return 0;
        }
        let a_loc: usize = a as usize; //.try_into().unwrap();
        let b_loc: usize = b as usize; //.try_into().unwrap();
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
