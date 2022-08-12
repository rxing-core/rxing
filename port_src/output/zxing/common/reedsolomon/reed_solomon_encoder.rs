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
// package com::google::zxing::common::reedsolomon;

/**
 * <p>Implements Reed-Solomon encoding, as the name implies.</p>
 *
 * @author Sean Owen
 * @author William Rucklidge
 */
pub struct ReedSolomonEncoder {

     let field: GenericGF;

     let cached_generators: List<GenericGFPoly>;
}

impl ReedSolomonEncoder {

    pub fn new( field: &GenericGF) -> ReedSolomonEncoder {
        let .field = field;
        let .cachedGenerators = ArrayList<>::new();
        cached_generators.add(GenericGFPoly::new(field,  : vec![i32; 1] = vec![1, ]
        ));
    }

    fn  build_generator(&self,  degree: i32) -> GenericGFPoly  {
        if degree >= self.cached_generators.size() {
             let last_generator: GenericGFPoly = self.cached_generators.get(self.cached_generators.size() - 1);
             {
                 let mut d: i32 = self.cached_generators.size();
                while d <= degree {
                    {
                         let next_generator: GenericGFPoly = last_generator.multiply(GenericGFPoly::new(self.field,  : vec![i32; 2] = vec![1, self.field.exp(d - 1 + self.field.get_generator_base()), ]
                        ));
                        self.cached_generators.add(next_generator);
                        last_generator = next_generator;
                    }
                    d += 1;
                 }
             }

        }
        return self.cached_generators.get(degree);
    }

    pub fn  encode(&self,  to_encode: &Vec<i32>,  ec_bytes: i32)   {
        if ec_bytes == 0 {
            throw IllegalArgumentException::new("No error correction bytes");
        }
         let data_bytes: i32 = to_encode.len() - ec_bytes;
        if data_bytes <= 0 {
            throw IllegalArgumentException::new("No data bytes provided");
        }
         let generator: GenericGFPoly = self.build_generator(ec_bytes);
         let info_coefficients: [i32; data_bytes] = [0; data_bytes];
        System::arraycopy(&to_encode, 0, &info_coefficients, 0, data_bytes);
         let mut info: GenericGFPoly = GenericGFPoly::new(self.field, &info_coefficients);
        info = info.multiply_by_monomial(ec_bytes, 1);
         let remainder: GenericGFPoly = info.divide(generator)[1];
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
    }
}

