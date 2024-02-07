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

/**
 * Records EAN prefix to GS1 Member Organization, where the member organization
 * correlates strongly with a country. This is an imperfect means of identifying
 * a country of origin by EAN-13 barcode value. See
 * <a href="http://en.wikipedia.org/wiki/List_of_GS1_country_codes">
 * http://en.wikipedia.org/wiki/List_of_GS1_country_codes</a>.
 *
 * @author Sean Owen
 */

pub struct EANManufacturerOrgSupport {
    ranges: Vec<[u32; 2]>,                 //= new ArrayList<>();
    countryIdentifiers: Vec<&'static str>, // = new ArrayList<>();
}

impl Default for EANManufacturerOrgSupport {
    fn default() -> Self {
        let mut slf = Self {
            ranges: Vec::with_capacity(150),
            countryIdentifiers: Vec::with_capacity(150),
        };
        slf.initIfNeeded();
        slf
    }
}

impl EANManufacturerOrgSupport {
    pub fn lookupCountryIdentifier(&self, productCode: &str) -> Option<&str> {
        let prefix = productCode[0..3].parse::<u32>().ok()?;
        let max = self.ranges.len();
        for (i, range) in self.ranges.iter().enumerate().take(max) {
            let start = range[0];
            if prefix < start {
                return None;
            }
            let end = if range[1] == 0 { start } else { range[1] };
            if prefix <= end {
                return Some(self.countryIdentifiers.get(i)?);
            }
        }

        None
    }

    fn add(&mut self, range: [u32; 2], id: &'static str) {
        self.ranges.push(range);
        self.countryIdentifiers.push(id);
    }

    fn initIfNeeded(&mut self) {
        if !self.ranges.is_empty() {
            return;
        }
        self.add([0, 19], "US/CA");
        self.add([30, 39], "US");
        self.add([60, 139], "US/CA");
        self.add([300, 379], "FR");
        self.add([380, 0], "BG");
        self.add([383, 0], "SI");
        self.add([385, 0], "HR");
        self.add([387, 0], "BA");
        self.add([400, 440], "DE");
        self.add([450, 459], "JP");
        self.add([460, 469], "RU");
        self.add([471, 0], "TW");
        self.add([474, 0], "EE");
        self.add([475, 0], "LV");
        self.add([476, 0], "AZ");
        self.add([477, 0], "LT");
        self.add([478, 0], "UZ");
        self.add([479, 0], "LK");
        self.add([480, 0], "PH");
        self.add([481, 0], "BY");
        self.add([482, 0], "UA");
        self.add([484, 0], "MD");
        self.add([485, 0], "AM");
        self.add([486, 0], "GE");
        self.add([487, 0], "KZ");
        self.add([489, 0], "HK");
        self.add([490, 499], "JP");
        self.add([500, 509], "GB");
        self.add([520, 0], "GR");
        self.add([528, 0], "LB");
        self.add([529, 0], "CY");
        self.add([531, 0], "MK");
        self.add([535, 0], "MT");
        self.add([539, 0], "IE");
        self.add([540, 549], "BE/LU");
        self.add([560, 0], "PT");
        self.add([569, 0], "IS");
        self.add([570, 579], "DK");
        self.add([590, 0], "PL");
        self.add([594, 0], "RO");
        self.add([599, 0], "HU");
        self.add([600, 601], "ZA");
        self.add([603, 0], "GH");
        self.add([608, 0], "BH");
        self.add([609, 0], "MU");
        self.add([611, 0], "MA");
        self.add([613, 0], "DZ");
        self.add([616, 0], "KE");
        self.add([618, 0], "CI");
        self.add([619, 0], "TN");
        self.add([621, 0], "SY");
        self.add([622, 0], "EG");
        self.add([624, 0], "LY");
        self.add([625, 0], "JO");
        self.add([626, 0], "IR");
        self.add([627, 0], "KW");
        self.add([628, 0], "SA");
        self.add([629, 0], "AE");
        self.add([640, 649], "FI");
        self.add([690, 695], "CN");
        self.add([700, 709], "NO");
        self.add([729, 0], "IL");
        self.add([730, 739], "SE");
        self.add([740, 0], "GT");
        self.add([741, 0], "SV");
        self.add([742, 0], "HN");
        self.add([743, 0], "NI");
        self.add([744, 0], "CR");
        self.add([745, 0], "PA");
        self.add([746, 0], "DO");
        self.add([750, 0], "MX");
        self.add([754, 755], "CA");
        self.add([759, 0], "VE");
        self.add([760, 769], "CH");
        self.add([770, 0], "CO");
        self.add([773, 0], "UY");
        self.add([775, 0], "PE");
        self.add([777, 0], "BO");
        self.add([779, 0], "AR");
        self.add([780, 0], "CL");
        self.add([784, 0], "PY");
        self.add([785, 0], "PE");
        self.add([786, 0], "EC");
        self.add([789, 790], "BR");
        self.add([800, 839], "IT");
        self.add([840, 849], "ES");
        self.add([850, 0], "CU");
        self.add([858, 0], "SK");
        self.add([859, 0], "CZ");
        self.add([860, 0], "YU");
        self.add([865, 0], "MN");
        self.add([867, 0], "KP");
        self.add([868, 869], "TR");
        self.add([870, 879], "NL");
        self.add([880, 0], "KR");
        self.add([885, 0], "TH");
        self.add([888, 0], "SG");
        self.add([890, 0], "IN");
        self.add([893, 0], "VN");
        self.add([896, 0], "PK");
        self.add([899, 0], "ID");
        self.add([900, 919], "AT");
        self.add([930, 939], "AU");
        self.add([940, 949], "AZ");
        self.add([955, 0], "MY");
        self.add([958, 0], "MO");
    }
}

/**
 * Tests {@link EANManufacturerOrgSupport}.
 *
 * @author Sean Owen
 */
#[cfg(test)]
mod EANManufacturerOrgSupportTest {
    use crate::oned::EANManufacturerOrgSupport;

    #[test]
    fn testLookup() {
        let support = EANManufacturerOrgSupport::default();
        assert!(support.lookupCountryIdentifier("472000").is_none());
        assert_eq!(
            "US/CA",
            support.lookupCountryIdentifier("000000").expect("msg")
        );
        assert_eq!(
            "MO",
            support.lookupCountryIdentifier("958000").expect("msg")
        );
        assert_eq!(
            "GB",
            support.lookupCountryIdentifier("500000").expect("msg")
        );
        assert_eq!(
            "GB",
            support.lookupCountryIdentifier("509000").expect("msg")
        );
    }
}
