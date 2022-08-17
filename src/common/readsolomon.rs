// GenericGFPoly.java
/**
 * <p>Represents a polynomial whose coefficients are elements of a GF.
 * Instances of this class are immutable.</p>
 *
 * <p>Much credit is due to William Rucklidge since portions of this code are an indirect
 * port of his C++ Reed-Solomon implementation.</p>
 *
 * @author Sean Owen
 */
struct GenericGFPoly {

     field: GenericGF,

     coefficients: Vec<i32>
}

impl GenericGFPoly {

   /**
  * @param field the {@link GenericGF} instance representing the field to use
  * to perform computations
  * @param coefficients coefficients as ints representing elements of GF(size), arranged
  * from most significant (highest-power term) coefficient to least significant
  * @throws IllegalArgumentException if argument is null or empty,
  * or if leading coefficient is 0 and this is not a
  * constant polynomial (that is, it is not the monomial "0")
  */
   fn new( field: &GenericGF,  coefficients: &Vec<i32>) -> Result<Self,IllegalArgumentException> {
    let mut new_poly: GenericGFPoly;
    if coefficients.len() == 0 {
           return Err(IllegalArgumentException::new());
       }
       new_poly.field = field;
        let coefficients_length: i32 = coefficients.len();
       if coefficients_length > 1 && coefficients[0] == 0 {
           // Leading term must be non-zero for anything except the constant polynomial "0"
            let first_non_zero: i32 = 1;
           while first_non_zero < coefficients_length && coefficients[first_non_zero] == 0 {
               first_non_zero += 1;
           }
           if first_non_zero == coefficients_length {
               new_poly.coefficients = vec![0, ];
           } else {
               new_poly.coefficients = coefficients;
               //System::arraycopy(&coefficients, first_non_zero, let .coefficients, 0, let .coefficients.len());
           }
       } else {
           new_poly.coefficients = coefficients;
       }
       Ok(new_poly)
   }

   fn  get_coefficients(&self) -> Vec<i32>  {
       return self.coefficients;
   }

   /**
  * @return degree of this polynomial
  */
   fn  get_degree(&self) -> i32  {
       return self.coefficients.len() - 1;
   }

   /**
  * @return true iff this polynomial is the monomial "0"
  */
   fn  is_zero(&self) -> bool  {
       return self.coefficients[0] == 0;
   }

   /**
  * @return coefficient of x^degree term in this polynomial
  */
   fn  get_coefficient(&self,  degree: i32) -> i32  {
       return self.coefficients[self.coefficients.len() - 1 - degree];
   }

   /**
  * @return evaluation of this polynomial at a given point
  */
   fn  evaluate_at(&self,  a: i32) -> i32  {
       if a == 0 {
           // Just return the x^0 coefficient
           return self.get_coefficient(0);
       }
       if a == 1 {
           // Just the sum of the coefficients
            let mut result: i32 = 0;
           for  coefficient in self.coefficients {
               result = GenericGF::add_or_subtract(result, coefficient);
           }
           return result;
       }
        let mut result: i32 = self.coefficients[0];
        let size: i32 = self.coefficients.len();
        {
            let mut i: i32 = 1;
           while i < size {
               {
                   result = GenericGF::add_or_subtract(&self.field.multiply(a, result), self.coefficients[i]);
               }
               i += 1;
            }
        }

       return result;
   }

   fn  add_or_subtract(&self,  other: &GenericGFPoly) -> Result<GenericGFPoly,IllegalArgumentException>  {
       if !self.field.equals(other.field) {
           return Err( IllegalArgumentException::new("GenericGFPolys do not have same GenericGF field") );
       }
       if self.is_zero() {
           return other;
       }
       if other.is_zero() {
           return self;
       }
        let smaller_coefficients: Vec<i32> = self.coefficients;
        let larger_coefficients: Vec<i32> = other.coefficients;
       if smaller_coefficients.len() > larger_coefficients.len() {
            let temp: Vec<i32> = smaller_coefficients;
           smaller_coefficients = larger_coefficients;
           larger_coefficients = temp;
       }
        let sum_diff: [i32; larger_coefficients.len()] = [0; larger_coefficients.len()];
        let length_diff: i32 = larger_coefficients.len() - smaller_coefficients.len();
       // Copy high-order terms only found in higher-degree polynomial's coefficients
       System::arraycopy(&larger_coefficients, 0, &sum_diff, 0, length_diff);
        {
            let mut i: i32 = length_diff;
           while i < larger_coefficients.len() {
               {
                   sum_diff[i] = GenericGF::add_or_subtract(smaller_coefficients[i - length_diff], larger_coefficients[i]);
               }
               i += 1;
            }
        }

       return GenericGFPoly::new(&self.field, &sum_diff);
   }

   fn  multiply(&self,  other: &GenericGFPoly) -> Result<GenericGFPoly,IllegalArgumentException>  {
       if !self.field.equals(other.field) {
           return Err(IllegalArgumentException::new("GenericGFPolys do not have same GenericGF field"));
       }
       if self.is_zero() || other.is_zero() {
           return Ok(self.field.get_zero());
       }
        let a_coefficients: Vec<i32> = self.coefficients;
        let a_length: i32 = a_coefficients.len();
        let b_coefficients: Vec<i32> = other.coefficients;
        let b_length: i32 = b_coefficients.len();
        let mut product: [i32; a_length + b_length - 1] = [0; a_length + b_length - 1];
        {
            let mut i: i32 = 0;
           while i < a_length {
               {
                    let a_coeff: i32 = a_coefficients[i];
                    {
                        let mut j: i32 = 0;
                       while j < b_length {
                           {
                               product[i + j] = GenericGF::add_or_subtract(product[i + j], &self.field.multiply(a_coeff, b_coefficients[j]));
                           }
                           j += 1;
                        }
                    }

               }
               i += 1;
            }
        }

       return GenericGFPoly::new(&self.field, &product);
   }

   fn  multiply(&self,  scalar: i32) -> GenericGFPoly  {
       if scalar == 0 {
           return self.field.get_zero();
       }
       if scalar == 1 {
           return self;
       }
        let size: i32 = self.coefficients.len();
        let mut product: [i32; size] = [0; size];
        {
            let mut i: i32 = 0;
           while i < size {
               {
                   product[i] = self.field.multiply(self.coefficients[i], scalar);
               }
               i += 1;
            }
        }

       return GenericGFPoly::new(&self.field, &product);
   }

   fn  multiply_by_monomial(&self,  degree: i32,  coefficient: i32) -> Result<GenericGFPoly,IllegalArgumentException>  {
       if degree < 0 {
           return Err( IllegalArgumentException::new());
       }
       if coefficient == 0 {
           return Ok(self.field.get_zero());
       }
        let size: i32 = self.coefficients.len();
        let mut product: [i32; size + degree] = [0; size + degree];
        {
            let mut i: i32 = 0;
           while i < size {
               {
                   product[i] = self.field.multiply(self.coefficients[i], coefficient);
               }
               i += 1;
            }
        }

       return GenericGFPoly::new(&self.field, &product);
   }

   fn  divide(&self,  other: &GenericGFPoly) -> Result<Vec<GenericGFPoly>,IllegalArgumentException>  {
       if !self.field.equals(other.field) {
           return Err( IllegalArgumentException::new("GenericGFPolys do not have same GenericGF field"));
       }
       if other.is_zero() {
           return Err( IllegalArgumentException::new("Divide by 0"));
       }
        let mut quotient: GenericGFPoly = self.field.get_zero();
        let mut remainder: GenericGFPoly = self;
        let denominator_leading_term: i32 = other.get_coefficient(&other.get_degree());
        let inverse_denominator_leading_term: i32 = self.field.inverse(denominator_leading_term);
       while remainder.get_degree() >= other.get_degree() && !remainder.is_zero() {
            let degree_difference: i32 = remainder.get_degree() - other.get_degree();
            let scale: i32 = self.field.multiply(&remainder.get_coefficient(&remainder.get_degree()), inverse_denominator_leading_term);
            let term: GenericGFPoly = other.multiply_by_monomial(degree_difference, scale);
            let iteration_quotient: GenericGFPoly = self.field.build_monomial(degree_difference, scale);
           quotient = quotient.add_or_subtract(&iteration_quotient);
           remainder = remainder.add_or_subtract(&term);
       }
       return  Ok(vec![quotient, remainder, ]);
   }

   pub fn  to_string(&self) -> String  {
       if self.is_zero() {
           return "0".to_owned();
       }
        let result: StringBuilder = StringBuilder::new(8 * self.get_degree());
        {
            let mut degree: i32 = self.get_degree();
           while degree >= 0 {
               {
                    let mut coefficient: i32 = self.get_coefficient(degree);
                   if coefficient != 0 {
                       if coefficient < 0 {
                           if degree == self.get_degree() {
                               result.append("-");
                           } else {
                               result.append(" - ");
                           }
                           coefficient = -coefficient;
                       } else {
                           if result.length() > 0 {
                               result.append(" + ");
                           }
                       }
                       if degree == 0 || coefficient != 1 {
                            let alpha_power: i32 = self.field.log(coefficient);
                           if alpha_power == 0 {
                               result.append('1');
                           } else if alpha_power == 1 {
                               result.append('a');
                           } else {
                               result.append("a^");
                               result.append(alpha_power);
                           }
                       }
                       if degree != 0 {
                           if degree == 1 {
                               result.append('x');
                           } else {
                               result.append("x^");
                               result.append(degree);
                           }
                       }
                   }
               }
               degree -= 1;
            }
        }

       return result.to_string();
   }
}

// GenericGF.java
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

// x^12 + x^6 + x^5 + x^3 + 1
const AZTEC_DATA_12: GenericGF = GenericGF::new(0x1069, 4096, 1);

// x^10 + x^3 + 1
 const AZTEC_DATA_10: GenericGF = GenericGF::new(0x409, 1024, 1);

// x^6 + x + 1
 const AZTEC_DATA_6: GenericGF = GenericGF::new(0x43, 64, 1);

// x^4 + x + 1
 const AZTEC_PARAM: GenericGF = GenericGF::new(0x13, 16, 1);

// x^8 + x^4 + x^3 + x^2 + 1
 const QR_CODE_FIELD_256: GenericGF = GenericGF::new(0x011D, 256, 0);

// x^8 + x^5 + x^3 + x^2 + 1
 const DATA_MATRIX_FIELD_256: GenericGF = GenericGF::new(0x012D, 256, 1);

 const AZTEC_DATA_8: GenericGF = DATA_MATRIX_FIELD_256;

 const MAXICODE_FIELD_64: GenericGF = AZTEC_DATA_6;

pub struct GenericGF {

      exp_table: Vec<i32>,

      log_table: Vec<i32>,

       zero: GenericGFPoly,

      one: GenericGFPoly,

      size: i32,

      primitive: i32,

      generator_base: i32
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
    pub fn new( primitive: i32,  size: i32,  b: i32) -> Self {
        let mut new_generic_gf : GenericGF;
        new_generic_gf .primitive = primitive;
        new_generic_gf .size = size;
        new_generic_gf .generatorBase = b;
        exp_table   = [0; size];
        log_table   = [0; size];
         let mut x: i32 = 1;
         {
             let mut i: i32 = 0;
            while i < size {
                {
                    exp_table[i] = x;
                    // we're assuming the generator alpha is 2
                    x *= 2;
                    if x >= size {
                        x ^= primitive;
                        x &= size - 1;
                    }
                }
                i += 1;
             }
         }

         {
             let mut i: i32 = 0;
            while i < size - 1 {
                {
                    log_table[exp_table[i]] = i;
                }
                i += 1;
             }
         }

        // logTable[0] == 0 but this should never be used
        new_generic_gf.zero = GenericGFPoly::new( 0, &vec![0, ]);
        new_generic_gf.one = GenericGFPoly::new( 0, &vec![1, ]);

        new_generic_gf
    }

    fn  get_zero(&self) -> GenericGFPoly  {
        return self.zero;
    }

    fn  get_one(&self) -> GenericGFPoly  {
        return self.one;
    }

    /**
   * @return the monomial representing coefficient * x^degree
   */
    fn  build_monomial(&self,  degree: i32,  coefficient: i32) -> Result<GenericGFPoly,IllegalArgumentException>  {
        if degree < 0 {
            return Err( IllegalArgumentException::new());
        }
        if coefficient == 0 {
            return Ok(self.zero);
        }
         let mut coefficients: [i32; degree + 1] = [0; degree + 1];
        coefficients[0] = coefficient;
        return GenericGFPoly::new(self, &coefficients);
    }

    /**
   * Implements both addition and subtraction -- they are the same in GF(size).
   *
   * @return sum/difference of a and b
   */
    fn  add_or_subtract( a: i32,  b: i32) -> i32  {
        return a ^ b;
    }

    /**
   * @return 2 to the power of a in GF(size)
   */
    fn  exp(&self,  a: i32) -> i32  {
        return self.exp_table[a];
    }

    /**
   * @return base 2 log of a in GF(size)
   */
    fn  log(&self,  a: i32) -> Result<i32,IllegalArgumentException>  {
        if a == 0 {
            return Err( IllegalArgumentException::new());
        }
        return self.log_table[a];
    }

    /**
   * @return multiplicative inverse of a
   */
    fn  inverse(&self,  a: i32) -> Result<i32,ArithmeticException>  {
        if a == 0 {
            return Err( ArithmeticException::new());
        }
        return self.exp_table[self.size - self.log_table[a] - 1];
    }

    /**
   * @return product of a and b in GF(size)
   */
    fn  multiply(&self,  a: i32,  b: i32) -> i32  {
        if a == 0 || b == 0 {
            return 0;
        }
        return self.exp_table[(self.log_table[a] + self.log_table[b]) % (self.size - 1)];
    }

    pub fn  get_size(&self) -> i32  {
        return self.size;
    }

    pub fn  get_generator_base(&self) -> i32  {
        return self.generator_base;
    }

    pub fn  to_string(&self) -> String  {
        return format!("GF(0x{},{})", Integer::to_hex_string(self.primitive), self.size);
    }
}

// ReedSolomonDecoder.java
/**
 * <p>Implements Reed-Solomon decoding, as the name implies.</p>
 *
 * <p>The algorithm will not be explained here, but the following references were helpful
 * in creating this implementation:</p>
 *
 * <ul>
 * <li>Bruce Maggs.
 * <a href="http://www.cs.cmu.edu/afs/cs.cmu.edu/project/pscico-guyb/realworld/www/rs_decode.ps">
 * "Decoding Reed-Solomon Codes"</a> (see discussion of Forney's Formula)</li>
 * <li>J.I. Hall. <a href="www.mth.msu.edu/~jhall/classes/codenotes/GRS.pdf">
 * "Chapter 5. Generalized Reed-Solomon Codes"</a>
 * (see discussion of Euclidean algorithm)</li>
 * </ul>
 *
 * <p>Much credit is due to William Rucklidge since portions of this code are an indirect
 * port of his C++ Reed-Solomon implementation.</p>
 *
 * @author Sean Owen
 * @author William Rucklidge
 * @author sanfordsquires
 */
pub struct ReedSolomonDecoder {

    field: GenericGF
}

impl ReedSolomonDecoder {

   pub fn new( field: &GenericGF) -> Self {
       Self{ field }
   }

   /**
  * <p>Decodes given set of received codewords, which include both data and error-correction
  * codewords. Really, this means it uses Reed-Solomon to detect and correct errors, in-place,
  * in the input.</p>
  *
  * @param received data and error-correction codewords
  * @param twoS number of error-correction codewords available
  * @throws ReedSolomonException if decoding fails for any reason
  */
   pub fn  decode(&self,  received: &Vec<i32>,  two_s: i32)  -> Result<(), ReedSolomonException>   {
        let poly: GenericGFPoly = GenericGFPoly::new(&self.field, &received);
        let syndrome_coefficients: [i32; two_s] = [0; two_s];
        let no_error: bool = true;
        {
            let mut i: i32 = 0;
           while i < two_s {
               {
                    let eval: i32 = poly.evaluate_at(&self.field.exp(i + self.field.get_generator_base()));
                   syndrome_coefficients[syndrome_coefficients.len() - 1 - i] = eval;
                   if eval != 0 {
                       no_error = false;
                   }
               }
               i += 1;
            }
        }

       if no_error {
           return;
       }
        let syndrome: GenericGFPoly = GenericGFPoly::new(&self.field, &syndrome_coefficients);
        let sigma_omega: Vec<GenericGFPoly> = self.run_euclidean_algorithm(&self.field.build_monomial(two_s, 1), &syndrome, two_s);
        let sigma: GenericGFPoly = sigma_omega[0];
        let omega: GenericGFPoly = sigma_omega[1];
        let error_locations: Vec<i32> = self.find_error_locations(&sigma);
        let error_magnitudes: Vec<i32> = self.find_error_magnitudes(&omega, &error_locations);
        {
            let mut i: i32 = 0;
           while i < error_locations.len() {
               {
                    let mut position: i32 = received.len() - 1 - self.field.log(error_locations[i]);
                   if position < 0 {
                       return Err( ReedSolomonException::new("Bad error location"));
                   }
                   received[position] = GenericGF::add_or_subtract(received[position], error_magnitudes[i]);
               }
               i += 1;
            }
        }
        Ok(())

   }

   fn  run_euclidean_algorithm(&self,  a: &GenericGFPoly,  b: &GenericGFPoly,  R: i32) -> Result<Vec<GenericGFPoly>, ReedSolomonException+IllegalStateException>   {
       // Assume a's degree is >= b's
       if a.get_degree() < b.get_degree() {
            let temp: GenericGFPoly = a;
           a = b;
           b = &temp;
       }
        let r_last: GenericGFPoly = a;
        let mut r: GenericGFPoly = b;
        let t_last: GenericGFPoly = self.field.get_zero();
        let mut t: GenericGFPoly = self.field.get_one();
       // Run Euclidean algorithm until r's degree is less than R/2
       while 2 * r.get_degree() >= R {
            let r_last_last: GenericGFPoly = r_last;
            let t_last_last: GenericGFPoly = t_last;
           r_last = r;
           t_last = t;
           // Divide rLastLast by rLast, with quotient in q and remainder in r
           if r_last.is_zero() {
               // Oops, Euclidean algorithm already terminated?
               return Err( ReedSolomonException::new("r_{i-1} was zero"));
           }
           r = r_last_last;
            let mut q: GenericGFPoly = self.field.get_zero();
            let denominator_leading_term: i32 = r_last.get_coefficient(&r_last.get_degree());
            let dlt_inverse: i32 = self.field.inverse(denominator_leading_term);
           while r.get_degree() >= r_last.get_degree() && !r.is_zero() {
                let degree_diff: i32 = r.get_degree() - r_last.get_degree();
                let scale: i32 = self.field.multiply(&r.get_coefficient(&r.get_degree()), dlt_inverse);
               q = q.add_or_subtract(&self.field.build_monomial(degree_diff, scale));
               r = r.add_or_subtract(&r_last.multiply_by_monomial(degree_diff, scale));
           }
           t = q.multiply(&t_last).add_or_subtract(t_last_last);
           if r.get_degree() >= r_last.get_degree() {
               return Err( IllegalStateException::new(format!("Division algorithm failed to reduce polynomial? r: {}, rLast: {}", r, r_last)));
           }
       }
        let sigma_tilde_at_zero: i32 = t.get_coefficient(0);
       if sigma_tilde_at_zero == 0 {
           return Err( ReedSolomonException::new("sigmaTilde(0) was zero"));
       }
        let inverse: i32 = self.field.inverse(sigma_tilde_at_zero);
        let sigma: GenericGFPoly = t.multiply(inverse);
        let omega: GenericGFPoly = r.multiply(inverse);
       return Ok(  vec![sigma, omega, ]);
   }

   fn  find_error_locations(&self,  error_locator: &GenericGFPoly) -> Result<Vec<i32>, ReedSolomonException>   {
       // This is a direct application of Chien's search
        let num_errors: i32 = error_locator.get_degree();
       if num_errors == 1 {
           // shortcut
           return Ok(  vec![error_locator.get_coefficient(1), ]);
       }
        let mut result: [i32; num_errors] = [0; num_errors];
        let mut e: i32 = 0;
        {
            let mut i: i32 = 1;
           while i < self.field.get_size() && e < num_errors {
               {
                   if error_locator.evaluate_at(i) == 0 {
                       result[e] = self.field.inverse(i);
                       e += 1;
                   }
               }
               i += 1;
            }
        }

       if e != num_errors {
           return Err( ReedSolomonException::new("Error locator degree does not match number of roots"));
       }
       return Ok(result);
   }

   fn  find_error_magnitudes(&self,  error_evaluator: &GenericGFPoly,  error_locations: &Vec<i32>) -> Vec<i32>  {
       // This is directly applying Forney's Formula
        let s: i32 = error_locations.len();
        let mut result: [i32; s] = [0; s];
        {
            let mut i: i32 = 0;
           while i < s {
               {
                    let xi_inverse: i32 = self.field.inverse(error_locations[i]);
                    let mut denominator: i32 = 1;
                    {
                        let mut j: i32 = 0;
                       while j < s {
                           {
                               if i != j {
                                   //denominator = field.multiply(denominator,
                                   //    GenericGF.addOrSubtract(1, field.multiply(errorLocations[j], xiInverse)));
                                   // Above should work but fails on some Apple and Linux JDKs due to a Hotspot bug.
                                   // Below is a funny-looking workaround from Steven Parkes
                                    let term: i32 = self.field.multiply(error_locations[j], xi_inverse);
                                    let term_plus1: i32 =  if (term & 0x1) == 0 { term | 1 } else { term & 1 };
                                   denominator = self.field.multiply(denominator, term_plus1);
                               }
                           }
                           j += 1;
                        }
                    }

                   result[i] = self.field.multiply(&error_evaluator.evaluate_at(xi_inverse), &self.field.inverse(denominator));
                   if self.field.get_generator_base() != 0 {
                       result[i] = self.field.multiply(result[i], xi_inverse);
                   }
               }
               i += 1;
            }
        }

       return result;
   }
}

// ReedSolomonEncoder.java
/**
 * <p>Implements Reed-Solomon encoding, as the name implies.</p>
 *
 * @author Sean Owen
 * @author William Rucklidge
 */
pub struct ReedSolomonEncoder {

    field: GenericGF,

     cached_generators: Vector<GenericGFPoly>
}

impl ReedSolomonEncoder {

   pub fn new( field: &GenericGF) -> Self {
    let mut new_rse;
    new_rse .field = field;
    new_rse .cachedGenerators = Vector::new();
       cached_generators.add(GenericGFPoly::new(field, &vec![1, ]));
       new_rse
   }

   fn  build_generator(&self,  degree: i32) -> GenericGFPoly  {
       if degree >= self.cached_generators.size() {
            let last_generator: GenericGFPoly = self.cached_generators.get(self.cached_generators.size() - 1);
            {
                let mut d: i32 = self.cached_generators.size();
               while d <= degree {
                   {
                        let next_generator: GenericGFPoly = last_generator.multiply(GenericGFPoly::new(&self.field,   &vec![1, self.field.exp(d - 1 + self.field.get_generator_base()), ]));
                       self.cached_generators.add(next_generator);
                       last_generator = next_generator;
                   }
                   d += 1;
                }
            }

       }
       return self.cached_generators.get(degree);
   }

   pub fn  encode(&self,  to_encode: &Vec<i32>,  ec_bytes: i32) -> Result<(),IllegalArgumentException>  {
       if ec_bytes == 0 {
           return Err( IllegalArgumentException::new("No error correction bytes"));
       }
        let data_bytes: i32 = to_encode.len() - ec_bytes;
       if data_bytes <= 0 {
           return Err( IllegalArgumentException::new("No data bytes provided"));
       }
        let generator: GenericGFPoly = self.build_generator(ec_bytes);
        let info_coefficients: [i32; data_bytes] = [0; data_bytes];
       System::arraycopy(&to_encode, 0, &info_coefficients, 0, data_bytes);
        let mut info: GenericGFPoly = GenericGFPoly::new(&self.field, &info_coefficients);
       info = info.multiply_by_monomial(ec_bytes, 1);
        let remainder: GenericGFPoly = info.divide(&generator)[1];
        let coefficients: Vec<i32> = remainder.get_coefficients();
        let num_zero_coefficients: i32 = ec_bytes - coefficients.len();
        {
            let mut i: i32 = 0;
           while i < num_zero_coefficients {
               {
                   to_encode[data_bytes + i] = 0;
               }
               i += 1;
            }
        }

       System::arraycopy(&coefficients, 0, &to_encode, data_bytes + num_zero_coefficients, coefficients.len());
       Ok(())
   }
}

// ReedSolomonException.java
/**
 * <p>Thrown when an exception occurs during Reed-Solomon decoding, such as when
 * there are too many errors to correct.</p>
 *
 * @author Sean Owen
 */
pub struct ReedSolomonException {
    message: String
}

impl ReedSolomonException {

    pub fn new( message: &String) -> Self {
        ReedSolomonException{message}
    }
}