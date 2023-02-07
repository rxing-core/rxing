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
    ranges: Vec<Vec<u32>>,           //= new ArrayList<>();
    countryIdentifiers: Vec<String>, // = new ArrayList<>();
}

impl Default for EANManufacturerOrgSupport {
    fn default() -> Self {
        let mut slf = Self {
            ranges: Default::default(),
            countryIdentifiers: Default::default(),
        };
        slf.initIfNeeded();
        slf
    }
}

impl EANManufacturerOrgSupport {
    pub fn lookupCountryIdentifier(&self, productCode: &str) -> Option<&str> {
        let prefix = productCode[0..3].parse::<u32>().expect("must parse prefix");
        let max = self.ranges.len();
        for (i, range) in self.ranges.iter().enumerate().take(max) {
            let start = range[0];
            if prefix < start {
                return None;
            }
            let end = if range.len() == 1 { start } else { range[1] };
            if prefix <= end {
                return Some(self.countryIdentifiers.get(i)?);
            }
        }

        None
    }

    fn add(&mut self, range: Vec<u32>, id: String) {
        self.ranges.push(range);
        self.countryIdentifiers.push(id);
    }

    fn initIfNeeded(&mut self) {
        if !self.ranges.is_empty() {
            return;
        }
        self.add(vec![0, 19], "US/CA".to_owned());
        self.add(vec![30, 39], "US".to_owned());
        self.add(vec![60, 139], "US/CA".to_owned());
        self.add(vec![300, 379], "FR".to_owned());
        self.add(vec![380], "BG".to_owned());
        self.add(vec![383], "SI".to_owned());
        self.add(vec![385], "HR".to_owned());
        self.add(vec![387], "BA".to_owned());
        self.add(vec![400, 440], "DE".to_owned());
        self.add(vec![450, 459], "JP".to_owned());
        self.add(vec![460, 469], "RU".to_owned());
        self.add(vec![471], "TW".to_owned());
        self.add(vec![474], "EE".to_owned());
        self.add(vec![475], "LV".to_owned());
        self.add(vec![476], "AZ".to_owned());
        self.add(vec![477], "LT".to_owned());
        self.add(vec![478], "UZ".to_owned());
        self.add(vec![479], "LK".to_owned());
        self.add(vec![480], "PH".to_owned());
        self.add(vec![481], "BY".to_owned());
        self.add(vec![482], "UA".to_owned());
        self.add(vec![484], "MD".to_owned());
        self.add(vec![485], "AM".to_owned());
        self.add(vec![486], "GE".to_owned());
        self.add(vec![487], "KZ".to_owned());
        self.add(vec![489], "HK".to_owned());
        self.add(vec![490, 499], "JP".to_owned());
        self.add(vec![500, 509], "GB".to_owned());
        self.add(vec![520], "GR".to_owned());
        self.add(vec![528], "LB".to_owned());
        self.add(vec![529], "CY".to_owned());
        self.add(vec![531], "MK".to_owned());
        self.add(vec![535], "MT".to_owned());
        self.add(vec![539], "IE".to_owned());
        self.add(vec![540, 549], "BE/LU".to_owned());
        self.add(vec![560], "PT".to_owned());
        self.add(vec![569], "IS".to_owned());
        self.add(vec![570, 579], "DK".to_owned());
        self.add(vec![590], "PL".to_owned());
        self.add(vec![594], "RO".to_owned());
        self.add(vec![599], "HU".to_owned());
        self.add(vec![600, 601], "ZA".to_owned());
        self.add(vec![603], "GH".to_owned());
        self.add(vec![608], "BH".to_owned());
        self.add(vec![609], "MU".to_owned());
        self.add(vec![611], "MA".to_owned());
        self.add(vec![613], "DZ".to_owned());
        self.add(vec![616], "KE".to_owned());
        self.add(vec![618], "CI".to_owned());
        self.add(vec![619], "TN".to_owned());
        self.add(vec![621], "SY".to_owned());
        self.add(vec![622], "EG".to_owned());
        self.add(vec![624], "LY".to_owned());
        self.add(vec![625], "JO".to_owned());
        self.add(vec![626], "IR".to_owned());
        self.add(vec![627], "KW".to_owned());
        self.add(vec![628], "SA".to_owned());
        self.add(vec![629], "AE".to_owned());
        self.add(vec![640, 649], "FI".to_owned());
        self.add(vec![690, 695], "CN".to_owned());
        self.add(vec![700, 709], "NO".to_owned());
        self.add(vec![729], "IL".to_owned());
        self.add(vec![730, 739], "SE".to_owned());
        self.add(vec![740], "GT".to_owned());
        self.add(vec![741], "SV".to_owned());
        self.add(vec![742], "HN".to_owned());
        self.add(vec![743], "NI".to_owned());
        self.add(vec![744], "CR".to_owned());
        self.add(vec![745], "PA".to_owned());
        self.add(vec![746], "DO".to_owned());
        self.add(vec![750], "MX".to_owned());
        self.add(vec![754, 755], "CA".to_owned());
        self.add(vec![759], "VE".to_owned());
        self.add(vec![760, 769], "CH".to_owned());
        self.add(vec![770], "CO".to_owned());
        self.add(vec![773], "UY".to_owned());
        self.add(vec![775], "PE".to_owned());
        self.add(vec![777], "BO".to_owned());
        self.add(vec![779], "AR".to_owned());
        self.add(vec![780], "CL".to_owned());
        self.add(vec![784], "PY".to_owned());
        self.add(vec![785], "PE".to_owned());
        self.add(vec![786], "EC".to_owned());
        self.add(vec![789, 790], "BR".to_owned());
        self.add(vec![800, 839], "IT".to_owned());
        self.add(vec![840, 849], "ES".to_owned());
        self.add(vec![850], "CU".to_owned());
        self.add(vec![858], "SK".to_owned());
        self.add(vec![859], "CZ".to_owned());
        self.add(vec![860], "YU".to_owned());
        self.add(vec![865], "MN".to_owned());
        self.add(vec![867], "KP".to_owned());
        self.add(vec![868, 869], "TR".to_owned());
        self.add(vec![870, 879], "NL".to_owned());
        self.add(vec![880], "KR".to_owned());
        self.add(vec![885], "TH".to_owned());
        self.add(vec![888], "SG".to_owned());
        self.add(vec![890], "IN".to_owned());
        self.add(vec![893], "VN".to_owned());
        self.add(vec![896], "PK".to_owned());
        self.add(vec![899], "ID".to_owned());
        self.add(vec![900, 919], "AT".to_owned());
        self.add(vec![930, 939], "AU".to_owned());
        self.add(vec![940, 949], "AZ".to_owned());
        self.add(vec![955], "MY".to_owned());
        self.add(vec![958], "MO".to_owned());
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
