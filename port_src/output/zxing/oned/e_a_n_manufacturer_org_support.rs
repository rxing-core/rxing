/*
 * Copyright (C) 2010 ZXing authors
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
// package com::google::zxing::oned;

/**
 * Records EAN prefix to GS1 Member Organization, where the member organization
 * correlates strongly with a country. This is an imperfect means of identifying
 * a country of origin by EAN-13 barcode value. See
 * <a href="http://en.wikipedia.org/wiki/List_of_GS1_country_codes">
 * http://en.wikipedia.org/wiki/List_of_GS1_country_codes</a>.
 *
 * @author Sean Owen
 */
struct EANManufacturerOrgSupport {

     let ranges: List<Vec<i32>> = ArrayList<>::new();

     let country_identifiers: List<String> = ArrayList<>::new();
}

impl EANManufacturerOrgSupport {

    fn  lookup_country_identifier(&self,  product_code: &String) -> String  {
        self.init_if_needed();
         let prefix: i32 = Integer::parse_int(&product_code.substring(0, 3));
         let max: i32 = self.ranges.size();
         {
             let mut i: i32 = 0;
            while i < max {
                {
                     let range: Vec<i32> = self.ranges.get(i);
                     let start: i32 = range[0];
                    if prefix < start {
                        return null;
                    }
                     let end: i32 =  if range.len() == 1 { start } else { range[1] };
                    if prefix <= end {
                        return self.country_identifiers.get(i);
                    }
                }
                i += 1;
             }
         }

        return null;
    }

    fn  add(&self,  range: &Vec<i32>,  id: &String)   {
        self.ranges.add(&range);
        self.country_identifiers.add(&id);
    }

    fn  init_if_needed(&self)   {
        if !self.ranges.is_empty() {
            return;
        }
        self.add( : vec![i32; 2] = vec![0, 19, ]
        , "US/CA");
        self.add( : vec![i32; 2] = vec![30, 39, ]
        , "US");
        self.add( : vec![i32; 2] = vec![60, 139, ]
        , "US/CA");
        self.add( : vec![i32; 2] = vec![300, 379, ]
        , "FR");
        self.add( : vec![i32; 1] = vec![380, ]
        , "BG");
        self.add( : vec![i32; 1] = vec![383, ]
        , "SI");
        self.add( : vec![i32; 1] = vec![385, ]
        , "HR");
        self.add( : vec![i32; 1] = vec![387, ]
        , "BA");
        self.add( : vec![i32; 2] = vec![400, 440, ]
        , "DE");
        self.add( : vec![i32; 2] = vec![450, 459, ]
        , "JP");
        self.add( : vec![i32; 2] = vec![460, 469, ]
        , "RU");
        self.add( : vec![i32; 1] = vec![471, ]
        , "TW");
        self.add( : vec![i32; 1] = vec![474, ]
        , "EE");
        self.add( : vec![i32; 1] = vec![475, ]
        , "LV");
        self.add( : vec![i32; 1] = vec![476, ]
        , "AZ");
        self.add( : vec![i32; 1] = vec![477, ]
        , "LT");
        self.add( : vec![i32; 1] = vec![478, ]
        , "UZ");
        self.add( : vec![i32; 1] = vec![479, ]
        , "LK");
        self.add( : vec![i32; 1] = vec![480, ]
        , "PH");
        self.add( : vec![i32; 1] = vec![481, ]
        , "BY");
        self.add( : vec![i32; 1] = vec![482, ]
        , "UA");
        self.add( : vec![i32; 1] = vec![484, ]
        , "MD");
        self.add( : vec![i32; 1] = vec![485, ]
        , "AM");
        self.add( : vec![i32; 1] = vec![486, ]
        , "GE");
        self.add( : vec![i32; 1] = vec![487, ]
        , "KZ");
        self.add( : vec![i32; 1] = vec![489, ]
        , "HK");
        self.add( : vec![i32; 2] = vec![490, 499, ]
        , "JP");
        self.add( : vec![i32; 2] = vec![500, 509, ]
        , "GB");
        self.add( : vec![i32; 1] = vec![520, ]
        , "GR");
        self.add( : vec![i32; 1] = vec![528, ]
        , "LB");
        self.add( : vec![i32; 1] = vec![529, ]
        , "CY");
        self.add( : vec![i32; 1] = vec![531, ]
        , "MK");
        self.add( : vec![i32; 1] = vec![535, ]
        , "MT");
        self.add( : vec![i32; 1] = vec![539, ]
        , "IE");
        self.add( : vec![i32; 2] = vec![540, 549, ]
        , "BE/LU");
        self.add( : vec![i32; 1] = vec![560, ]
        , "PT");
        self.add( : vec![i32; 1] = vec![569, ]
        , "IS");
        self.add( : vec![i32; 2] = vec![570, 579, ]
        , "DK");
        self.add( : vec![i32; 1] = vec![590, ]
        , "PL");
        self.add( : vec![i32; 1] = vec![594, ]
        , "RO");
        self.add( : vec![i32; 1] = vec![599, ]
        , "HU");
        self.add( : vec![i32; 2] = vec![600, 601, ]
        , "ZA");
        self.add( : vec![i32; 1] = vec![603, ]
        , "GH");
        self.add( : vec![i32; 1] = vec![608, ]
        , "BH");
        self.add( : vec![i32; 1] = vec![609, ]
        , "MU");
        self.add( : vec![i32; 1] = vec![611, ]
        , "MA");
        self.add( : vec![i32; 1] = vec![613, ]
        , "DZ");
        self.add( : vec![i32; 1] = vec![616, ]
        , "KE");
        self.add( : vec![i32; 1] = vec![618, ]
        , "CI");
        self.add( : vec![i32; 1] = vec![619, ]
        , "TN");
        self.add( : vec![i32; 1] = vec![621, ]
        , "SY");
        self.add( : vec![i32; 1] = vec![622, ]
        , "EG");
        self.add( : vec![i32; 1] = vec![624, ]
        , "LY");
        self.add( : vec![i32; 1] = vec![625, ]
        , "JO");
        self.add( : vec![i32; 1] = vec![626, ]
        , "IR");
        self.add( : vec![i32; 1] = vec![627, ]
        , "KW");
        self.add( : vec![i32; 1] = vec![628, ]
        , "SA");
        self.add( : vec![i32; 1] = vec![629, ]
        , "AE");
        self.add( : vec![i32; 2] = vec![640, 649, ]
        , "FI");
        self.add( : vec![i32; 2] = vec![690, 695, ]
        , "CN");
        self.add( : vec![i32; 2] = vec![700, 709, ]
        , "NO");
        self.add( : vec![i32; 1] = vec![729, ]
        , "IL");
        self.add( : vec![i32; 2] = vec![730, 739, ]
        , "SE");
        self.add( : vec![i32; 1] = vec![740, ]
        , "GT");
        self.add( : vec![i32; 1] = vec![741, ]
        , "SV");
        self.add( : vec![i32; 1] = vec![742, ]
        , "HN");
        self.add( : vec![i32; 1] = vec![743, ]
        , "NI");
        self.add( : vec![i32; 1] = vec![744, ]
        , "CR");
        self.add( : vec![i32; 1] = vec![745, ]
        , "PA");
        self.add( : vec![i32; 1] = vec![746, ]
        , "DO");
        self.add( : vec![i32; 1] = vec![750, ]
        , "MX");
        self.add( : vec![i32; 2] = vec![754, 755, ]
        , "CA");
        self.add( : vec![i32; 1] = vec![759, ]
        , "VE");
        self.add( : vec![i32; 2] = vec![760, 769, ]
        , "CH");
        self.add( : vec![i32; 1] = vec![770, ]
        , "CO");
        self.add( : vec![i32; 1] = vec![773, ]
        , "UY");
        self.add( : vec![i32; 1] = vec![775, ]
        , "PE");
        self.add( : vec![i32; 1] = vec![777, ]
        , "BO");
        self.add( : vec![i32; 1] = vec![779, ]
        , "AR");
        self.add( : vec![i32; 1] = vec![780, ]
        , "CL");
        self.add( : vec![i32; 1] = vec![784, ]
        , "PY");
        self.add( : vec![i32; 1] = vec![785, ]
        , "PE");
        self.add( : vec![i32; 1] = vec![786, ]
        , "EC");
        self.add( : vec![i32; 2] = vec![789, 790, ]
        , "BR");
        self.add( : vec![i32; 2] = vec![800, 839, ]
        , "IT");
        self.add( : vec![i32; 2] = vec![840, 849, ]
        , "ES");
        self.add( : vec![i32; 1] = vec![850, ]
        , "CU");
        self.add( : vec![i32; 1] = vec![858, ]
        , "SK");
        self.add( : vec![i32; 1] = vec![859, ]
        , "CZ");
        self.add( : vec![i32; 1] = vec![860, ]
        , "YU");
        self.add( : vec![i32; 1] = vec![865, ]
        , "MN");
        self.add( : vec![i32; 1] = vec![867, ]
        , "KP");
        self.add( : vec![i32; 2] = vec![868, 869, ]
        , "TR");
        self.add( : vec![i32; 2] = vec![870, 879, ]
        , "NL");
        self.add( : vec![i32; 1] = vec![880, ]
        , "KR");
        self.add( : vec![i32; 1] = vec![885, ]
        , "TH");
        self.add( : vec![i32; 1] = vec![888, ]
        , "SG");
        self.add( : vec![i32; 1] = vec![890, ]
        , "IN");
        self.add( : vec![i32; 1] = vec![893, ]
        , "VN");
        self.add( : vec![i32; 1] = vec![896, ]
        , "PK");
        self.add( : vec![i32; 1] = vec![899, ]
        , "ID");
        self.add( : vec![i32; 2] = vec![900, 919, ]
        , "AT");
        self.add( : vec![i32; 2] = vec![930, 939, ]
        , "AU");
        self.add( : vec![i32; 2] = vec![940, 949, ]
        , "AZ");
        self.add( : vec![i32; 1] = vec![955, ]
        , "MY");
        self.add( : vec![i32; 1] = vec![958, ]
        , "MO");
    }
}

