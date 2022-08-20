/*
 * Copyright 2007 ZXing authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//package com.google.zxing.common.reedsolomon;

/**
 * <p>Thrown when an exception occurs during Reed-Solomon decoding, such as when
 * there are too many errors to correct.</p>
 *
 * @author Sean Owen
 */
pub struct ReedSolomonException {
    message: String,
}

impl ReedSolomonException {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
        }
    }
}

/*
 * Copyright 2007 ZXing authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package com.google.zxing.common.reedsolomon;

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
public final class GenericGF {

  public static final GenericGF AZTEC_DATA_12 = new GenericGF(0x1069, 4096, 1); // x^12 + x^6 + x^5 + x^3 + 1
  public static final GenericGF AZTEC_DATA_10 = new GenericGF(0x409, 1024, 1); // x^10 + x^3 + 1
  public static final GenericGF AZTEC_DATA_6 = new GenericGF(0x43, 64, 1); // x^6 + x + 1
  public static final GenericGF AZTEC_PARAM = new GenericGF(0x13, 16, 1); // x^4 + x + 1
  public static final GenericGF QR_CODE_FIELD_256 = new GenericGF(0x011D, 256, 0); // x^8 + x^4 + x^3 + x^2 + 1
  public static final GenericGF DATA_MATRIX_FIELD_256 = new GenericGF(0x012D, 256, 1); // x^8 + x^5 + x^3 + x^2 + 1
  public static final GenericGF AZTEC_DATA_8 = DATA_MATRIX_FIELD_256;
  public static final GenericGF MAXICODE_FIELD_64 = AZTEC_DATA_6;

  private final int[] expTable;
  private final int[] logTable;
  private final GenericGFPoly zero;
  private final GenericGFPoly one;
  private final int size;
  private final int primitive;
  private final int generatorBase;

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
  public GenericGF(int primitive, int size, int b) {
    this.primitive = primitive;
    this.size = size;
    this.generatorBase = b;

    expTable = new int[size];
    logTable = new int[size];
    int x = 1;
    for (int i = 0; i < size; i++) {
      expTable[i] = x;
      x *= 2; // we're assuming the generator alpha is 2
      if (x >= size) {
        x ^= primitive;
        x &= size - 1;
      }
    }
    for (int i = 0; i < size - 1; i++) {
      logTable[expTable[i]] = i;
    }
    // logTable[0] == 0 but this should never be used
    zero = new GenericGFPoly(this, new int[]{0});
    one = new GenericGFPoly(this, new int[]{1});
  }

  GenericGFPoly getZero() {
    return zero;
  }

  GenericGFPoly getOne() {
    return one;
  }

  /**
   * @return the monomial representing coefficient * x^degree
   */
  GenericGFPoly buildMonomial(int degree, int coefficient) {
    if (degree < 0) {
      throw new IllegalArgumentException();
    }
    if (coefficient == 0) {
      return zero;
    }
    int[] coefficients = new int[degree + 1];
    coefficients[0] = coefficient;
    return new GenericGFPoly(this, coefficients);
  }

  /**
   * Implements both addition and subtraction -- they are the same in GF(size).
   *
   * @return sum/difference of a and b
   */
  static int addOrSubtract(int a, int b) {
    return a ^ b;
  }

  /**
   * @return 2 to the power of a in GF(size)
   */
  int exp(int a) {
    return expTable[a];
  }

  /**
   * @return base 2 log of a in GF(size)
   */
  int log(int a) {
    if (a == 0) {
      throw new IllegalArgumentException();
    }
    return logTable[a];
  }

  /**
   * @return multiplicative inverse of a
   */
  int inverse(int a) {
    if (a == 0) {
      throw new ArithmeticException();
    }
    return expTable[size - logTable[a] - 1];
  }

  /**
   * @return product of a and b in GF(size)
   */
  int multiply(int a, int b) {
    if (a == 0 || b == 0) {
      return 0;
    }
    return expTable[(logTable[a] + logTable[b]) % (size - 1)];
  }

  public int getSize() {
    return size;
  }

  public int getGeneratorBase() {
    return generatorBase;
  }

  @Override
  public String toString() {
    return "GF(0x" + Integer.toHexString(primitive) + ',' + size + ')';
  }

}


/*
 * Copyright 2007 ZXing authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package com.google.zxing.common.reedsolomon;

/**
 * <p>Represents a polynomial whose coefficients are elements of a GF.
 * Instances of this class are immutable.</p>
 *
 * <p>Much credit is due to William Rucklidge since portions of this code are an indirect
 * port of his C++ Reed-Solomon implementation.</p>
 *
 * @author Sean Owen
 */
final class GenericGFPoly {

  private final GenericGF field;
  private final int[] coefficients;

  /**
   * @param field the {@link GenericGF} instance representing the field to use
   * to perform computations
   * @param coefficients coefficients as ints representing elements of GF(size), arranged
   * from most significant (highest-power term) coefficient to least significant
   * @throws IllegalArgumentException if argument is null or empty,
   * or if leading coefficient is 0 and this is not a
   * constant polynomial (that is, it is not the monomial "0")
   */
  GenericGFPoly(GenericGF field, int[] coefficients) {
    if (coefficients.length == 0) {
      throw new IllegalArgumentException();
    }
    this.field = field;
    int coefficientsLength = coefficients.length;
    if (coefficientsLength > 1 && coefficients[0] == 0) {
      // Leading term must be non-zero for anything except the constant polynomial "0"
      int firstNonZero = 1;
      while (firstNonZero < coefficientsLength && coefficients[firstNonZero] == 0) {
        firstNonZero++;
      }
      if (firstNonZero == coefficientsLength) {
        this.coefficients = new int[]{0};
      } else {
        this.coefficients = new int[coefficientsLength - firstNonZero];
        System.arraycopy(coefficients,
            firstNonZero,
            this.coefficients,
            0,
            this.coefficients.length);
      }
    } else {
      this.coefficients = coefficients;
    }
  }

  int[] getCoefficients() {
    return coefficients;
  }

  /**
   * @return degree of this polynomial
   */
  int getDegree() {
    return coefficients.length - 1;
  }

  /**
   * @return true iff this polynomial is the monomial "0"
   */
  boolean isZero() {
    return coefficients[0] == 0;
  }

  /**
   * @return coefficient of x^degree term in this polynomial
   */
  int getCoefficient(int degree) {
    return coefficients[coefficients.length - 1 - degree];
  }

  /**
   * @return evaluation of this polynomial at a given point
   */
  int evaluateAt(int a) {
    if (a == 0) {
      // Just return the x^0 coefficient
      return getCoefficient(0);
    }
    if (a == 1) {
      // Just the sum of the coefficients
      int result = 0;
      for (int coefficient : coefficients) {
        result = GenericGF.addOrSubtract(result, coefficient);
      }
      return result;
    }
    int result = coefficients[0];
    int size = coefficients.length;
    for (int i = 1; i < size; i++) {
      result = GenericGF.addOrSubtract(field.multiply(a, result), coefficients[i]);
    }
    return result;
  }

  GenericGFPoly addOrSubtract(GenericGFPoly other) {
    if (!field.equals(other.field)) {
      throw new IllegalArgumentException("GenericGFPolys do not have same GenericGF field");
    }
    if (isZero()) {
      return other;
    }
    if (other.isZero()) {
      return this;
    }

    int[] smallerCoefficients = this.coefficients;
    int[] largerCoefficients = other.coefficients;
    if (smallerCoefficients.length > largerCoefficients.length) {
      int[] temp = smallerCoefficients;
      smallerCoefficients = largerCoefficients;
      largerCoefficients = temp;
    }
    int[] sumDiff = new int[largerCoefficients.length];
    int lengthDiff = largerCoefficients.length - smallerCoefficients.length;
    // Copy high-order terms only found in higher-degree polynomial's coefficients
    System.arraycopy(largerCoefficients, 0, sumDiff, 0, lengthDiff);

    for (int i = lengthDiff; i < largerCoefficients.length; i++) {
      sumDiff[i] = GenericGF.addOrSubtract(smallerCoefficients[i - lengthDiff], largerCoefficients[i]);
    }

    return new GenericGFPoly(field, sumDiff);
  }

  GenericGFPoly multiply(GenericGFPoly other) {
    if (!field.equals(other.field)) {
      throw new IllegalArgumentException("GenericGFPolys do not have same GenericGF field");
    }
    if (isZero() || other.isZero()) {
      return field.getZero();
    }
    int[] aCoefficients = this.coefficients;
    int aLength = aCoefficients.length;
    int[] bCoefficients = other.coefficients;
    int bLength = bCoefficients.length;
    int[] product = new int[aLength + bLength - 1];
    for (int i = 0; i < aLength; i++) {
      int aCoeff = aCoefficients[i];
      for (int j = 0; j < bLength; j++) {
        product[i + j] = GenericGF.addOrSubtract(product[i + j],
            field.multiply(aCoeff, bCoefficients[j]));
      }
    }
    return new GenericGFPoly(field, product);
  }

  GenericGFPoly multiply(int scalar) {
    if (scalar == 0) {
      return field.getZero();
    }
    if (scalar == 1) {
      return this;
    }
    int size = coefficients.length;
    int[] product = new int[size];
    for (int i = 0; i < size; i++) {
      product[i] = field.multiply(coefficients[i], scalar);
    }
    return new GenericGFPoly(field, product);
  }

  GenericGFPoly multiplyByMonomial(int degree, int coefficient) {
    if (degree < 0) {
      throw new IllegalArgumentException();
    }
    if (coefficient == 0) {
      return field.getZero();
    }
    int size = coefficients.length;
    int[] product = new int[size + degree];
    for (int i = 0; i < size; i++) {
      product[i] = field.multiply(coefficients[i], coefficient);
    }
    return new GenericGFPoly(field, product);
  }

  GenericGFPoly[] divide(GenericGFPoly other) {
    if (!field.equals(other.field)) {
      throw new IllegalArgumentException("GenericGFPolys do not have same GenericGF field");
    }
    if (other.isZero()) {
      throw new IllegalArgumentException("Divide by 0");
    }

    GenericGFPoly quotient = field.getZero();
    GenericGFPoly remainder = this;

    int denominatorLeadingTerm = other.getCoefficient(other.getDegree());
    int inverseDenominatorLeadingTerm = field.inverse(denominatorLeadingTerm);

    while (remainder.getDegree() >= other.getDegree() && !remainder.isZero()) {
      int degreeDifference = remainder.getDegree() - other.getDegree();
      int scale = field.multiply(remainder.getCoefficient(remainder.getDegree()), inverseDenominatorLeadingTerm);
      GenericGFPoly term = other.multiplyByMonomial(degreeDifference, scale);
      GenericGFPoly iterationQuotient = field.buildMonomial(degreeDifference, scale);
      quotient = quotient.addOrSubtract(iterationQuotient);
      remainder = remainder.addOrSubtract(term);
    }

    return new GenericGFPoly[] { quotient, remainder };
  }

  @Override
  public String toString() {
    if (isZero()) {
      return "0";
    }
    StringBuilder result = new StringBuilder(8 * getDegree());
    for (int degree = getDegree(); degree >= 0; degree--) {
      int coefficient = getCoefficient(degree);
      if (coefficient != 0) {
        if (coefficient < 0) {
          if (degree == getDegree()) {
            result.append("-");
          } else {
            result.append(" - ");
          }
          coefficient = -coefficient;
        } else {
          if (result.length() > 0) {
            result.append(" + ");
          }
        }
        if (degree == 0 || coefficient != 1) {
          int alphaPower = field.log(coefficient);
          if (alphaPower == 0) {
            result.append('1');
          } else if (alphaPower == 1) {
            result.append('a');
          } else {
            result.append("a^");
            result.append(alphaPower);
          }
        }
        if (degree != 0) {
          if (degree == 1) {
            result.append('x');
          } else {
            result.append("x^");
            result.append(degree);
          }
        }
      }
    }
    return result.toString();
  }

}


/*
 * Copyright 2007 ZXing authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package com.google.zxing.common.reedsolomon;

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
public final class ReedSolomonDecoder {

  private final GenericGF field;

  public ReedSolomonDecoder(GenericGF field) {
    this.field = field;
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
  public void decode(int[] received, int twoS) throws ReedSolomonException {
    GenericGFPoly poly = new GenericGFPoly(field, received);
    int[] syndromeCoefficients = new int[twoS];
    boolean noError = true;
    for (int i = 0; i < twoS; i++) {
      int eval = poly.evaluateAt(field.exp(i + field.getGeneratorBase()));
      syndromeCoefficients[syndromeCoefficients.length - 1 - i] = eval;
      if (eval != 0) {
        noError = false;
      }
    }
    if (noError) {
      return;
    }
    GenericGFPoly syndrome = new GenericGFPoly(field, syndromeCoefficients);
    GenericGFPoly[] sigmaOmega =
        runEuclideanAlgorithm(field.buildMonomial(twoS, 1), syndrome, twoS);
    GenericGFPoly sigma = sigmaOmega[0];
    GenericGFPoly omega = sigmaOmega[1];
    int[] errorLocations = findErrorLocations(sigma);
    int[] errorMagnitudes = findErrorMagnitudes(omega, errorLocations);
    for (int i = 0; i < errorLocations.length; i++) {
      int position = received.length - 1 - field.log(errorLocations[i]);
      if (position < 0) {
        throw new ReedSolomonException("Bad error location");
      }
      received[position] = GenericGF.addOrSubtract(received[position], errorMagnitudes[i]);
    }
  }

  private GenericGFPoly[] runEuclideanAlgorithm(GenericGFPoly a, GenericGFPoly b, int R)
      throws ReedSolomonException {
    // Assume a's degree is >= b's
    if (a.getDegree() < b.getDegree()) {
      GenericGFPoly temp = a;
      a = b;
      b = temp;
    }

    GenericGFPoly rLast = a;
    GenericGFPoly r = b;
    GenericGFPoly tLast = field.getZero();
    GenericGFPoly t = field.getOne();

    // Run Euclidean algorithm until r's degree is less than R/2
    while (2 * r.getDegree() >= R) {
      GenericGFPoly rLastLast = rLast;
      GenericGFPoly tLastLast = tLast;
      rLast = r;
      tLast = t;

      // Divide rLastLast by rLast, with quotient in q and remainder in r
      if (rLast.isZero()) {
        // Oops, Euclidean algorithm already terminated?
        throw new ReedSolomonException("r_{i-1} was zero");
      }
      r = rLastLast;
      GenericGFPoly q = field.getZero();
      int denominatorLeadingTerm = rLast.getCoefficient(rLast.getDegree());
      int dltInverse = field.inverse(denominatorLeadingTerm);
      while (r.getDegree() >= rLast.getDegree() && !r.isZero()) {
        int degreeDiff = r.getDegree() - rLast.getDegree();
        int scale = field.multiply(r.getCoefficient(r.getDegree()), dltInverse);
        q = q.addOrSubtract(field.buildMonomial(degreeDiff, scale));
        r = r.addOrSubtract(rLast.multiplyByMonomial(degreeDiff, scale));
      }

      t = q.multiply(tLast).addOrSubtract(tLastLast);

      if (r.getDegree() >= rLast.getDegree()) {
        throw new IllegalStateException("Division algorithm failed to reduce polynomial? " +
          "r: " + r + ", rLast: " + rLast);
      }
    }

    int sigmaTildeAtZero = t.getCoefficient(0);
    if (sigmaTildeAtZero == 0) {
      throw new ReedSolomonException("sigmaTilde(0) was zero");
    }

    int inverse = field.inverse(sigmaTildeAtZero);
    GenericGFPoly sigma = t.multiply(inverse);
    GenericGFPoly omega = r.multiply(inverse);
    return new GenericGFPoly[]{sigma, omega};
  }

  private int[] findErrorLocations(GenericGFPoly errorLocator) throws ReedSolomonException {
    // This is a direct application of Chien's search
    int numErrors = errorLocator.getDegree();
    if (numErrors == 1) { // shortcut
      return new int[] { errorLocator.getCoefficient(1) };
    }
    int[] result = new int[numErrors];
    int e = 0;
    for (int i = 1; i < field.getSize() && e < numErrors; i++) {
      if (errorLocator.evaluateAt(i) == 0) {
        result[e] = field.inverse(i);
        e++;
      }
    }
    if (e != numErrors) {
      throw new ReedSolomonException("Error locator degree does not match number of roots");
    }
    return result;
  }

  private int[] findErrorMagnitudes(GenericGFPoly errorEvaluator, int[] errorLocations) {
    // This is directly applying Forney's Formula
    int s = errorLocations.length;
    int[] result = new int[s];
    for (int i = 0; i < s; i++) {
      int xiInverse = field.inverse(errorLocations[i]);
      int denominator = 1;
      for (int j = 0; j < s; j++) {
        if (i != j) {
          //denominator = field.multiply(denominator,
          //    GenericGF.addOrSubtract(1, field.multiply(errorLocations[j], xiInverse)));
          // Above should work but fails on some Apple and Linux JDKs due to a Hotspot bug.
          // Below is a funny-looking workaround from Steven Parkes
          int term = field.multiply(errorLocations[j], xiInverse);
          int termPlus1 = (term & 0x1) == 0 ? term | 1 : term & ~1;
          denominator = field.multiply(denominator, termPlus1);
        }
      }
      result[i] = field.multiply(errorEvaluator.evaluateAt(xiInverse),
          field.inverse(denominator));
      if (field.getGeneratorBase() != 0) {
        result[i] = field.multiply(result[i], xiInverse);
      }
    }
    return result;
  }

}


/*
 * Copyright 2008 ZXing authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package com.google.zxing.common.reedsolomon;

import java.util.ArrayList;
import java.util.List;

/**
 * <p>Implements Reed-Solomon encoding, as the name implies.</p>
 *
 * @author Sean Owen
 * @author William Rucklidge
 */
public final class ReedSolomonEncoder {

  private final GenericGF field;
  private final List<GenericGFPoly> cachedGenerators;

  public ReedSolomonEncoder(GenericGF field) {
    this.field = field;
    this.cachedGenerators = new ArrayList<>();
    cachedGenerators.add(new GenericGFPoly(field, new int[]{1}));
  }

  private GenericGFPoly buildGenerator(int degree) {
    if (degree >= cachedGenerators.size()) {
      GenericGFPoly lastGenerator = cachedGenerators.get(cachedGenerators.size() - 1);
      for (int d = cachedGenerators.size(); d <= degree; d++) {
        GenericGFPoly nextGenerator = lastGenerator.multiply(
            new GenericGFPoly(field, new int[] { 1, field.exp(d - 1 + field.getGeneratorBase()) }));
        cachedGenerators.add(nextGenerator);
        lastGenerator = nextGenerator;
      }
    }
    return cachedGenerators.get(degree);
  }

  public void encode(int[] toEncode, int ecBytes) {
    if (ecBytes == 0) {
      throw new IllegalArgumentException("No error correction bytes");
    }
    int dataBytes = toEncode.length - ecBytes;
    if (dataBytes <= 0) {
      throw new IllegalArgumentException("No data bytes provided");
    }
    GenericGFPoly generator = buildGenerator(ecBytes);
    int[] infoCoefficients = new int[dataBytes];
    System.arraycopy(toEncode, 0, infoCoefficients, 0, dataBytes);
    GenericGFPoly info = new GenericGFPoly(field, infoCoefficients);
    info = info.multiplyByMonomial(ecBytes, 1);
    GenericGFPoly remainder = info.divide(generator)[1];
    int[] coefficients = remainder.getCoefficients();
    int numZeroCoefficients = ecBytes - coefficients.length;
    for (int i = 0; i < numZeroCoefficients; i++) {
      toEncode[dataBytes + i] = 0;
    }
    System.arraycopy(coefficients, 0, toEncode, dataBytes + numZeroCoefficients, coefficients.length);
  }

}
