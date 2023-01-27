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

/*
 * These authors would like to acknowledge the Spanish Ministry of Industry,
 * Tourism and Trade, for the support in the project TSI020301-2008-2
 * "PIRAmIDE: Personalizable Interactions with Resources on AmI-enabled
 * Mobile Dynamic Environments", led by Treelogic
 * ( http://www.treelogic.com/ ):
 *
 *   http://www.piramidepse.com/
 */
/**
 * @author Pablo Orduña, University of Deusto (pablo.orduna@deusto.es)
 * @author Eduardo Castillejo, University of Deusto (eduardo.castillejo@deusto.es)
 */
use std::collections::HashMap;

use crate::Exceptions;

use once_cell::sync::Lazy;

static TWO_DIGIT_DATA_LENGTH: Lazy<HashMap<String, DataLength>> = Lazy::new(|| {
    let mut hm = HashMap::new();
    hm.insert("00".to_owned(), DataLength::fixed(18));
    hm.insert("01".to_owned(), DataLength::fixed(14));
    hm.insert("02".to_owned(), DataLength::fixed(14));
    hm.insert("10".to_owned(), DataLength::variable(20));
    hm.insert("11".to_owned(), DataLength::fixed(6));
    hm.insert("12".to_owned(), DataLength::fixed(6));
    hm.insert("13".to_owned(), DataLength::fixed(6));
    hm.insert("15".to_owned(), DataLength::fixed(6));
    hm.insert("17".to_owned(), DataLength::fixed(6));
    hm.insert("20".to_owned(), DataLength::fixed(2));
    hm.insert("21".to_owned(), DataLength::variable(20));
    hm.insert("22".to_owned(), DataLength::variable(29));
    hm.insert("30".to_owned(), DataLength::variable(8));
    hm.insert("37".to_owned(), DataLength::variable(8));
    //internal company codes
    for i in 90..=99 {
        // for (int i = 90; i <= 99; i++) {
        hm.insert(i.to_string(), DataLength::variable(30));
    }
    hm
});

static THREE_DIGIT_DATA_LENGTH: Lazy<HashMap<String, DataLength>> = Lazy::new(|| {
    let mut hm = HashMap::new();
    hm.insert("240".to_owned(), DataLength::variable(30));
    hm.insert("241".to_owned(), DataLength::variable(30));
    hm.insert("242".to_owned(), DataLength::variable(6));
    hm.insert("250".to_owned(), DataLength::variable(30));
    hm.insert("251".to_owned(), DataLength::variable(30));
    hm.insert("253".to_owned(), DataLength::variable(17));
    hm.insert("254".to_owned(), DataLength::variable(20));
    hm.insert("400".to_owned(), DataLength::variable(30));
    hm.insert("401".to_owned(), DataLength::variable(30));
    hm.insert("402".to_owned(), DataLength::fixed(17));
    hm.insert("403".to_owned(), DataLength::variable(30));
    hm.insert("410".to_owned(), DataLength::fixed(13));
    hm.insert("411".to_owned(), DataLength::fixed(13));
    hm.insert("412".to_owned(), DataLength::fixed(13));
    hm.insert("413".to_owned(), DataLength::fixed(13));
    hm.insert("414".to_owned(), DataLength::fixed(13));
    hm.insert("420".to_owned(), DataLength::variable(20));
    hm.insert("421".to_owned(), DataLength::variable(15));
    hm.insert("422".to_owned(), DataLength::fixed(3));
    hm.insert("423".to_owned(), DataLength::variable(15));
    hm.insert("424".to_owned(), DataLength::fixed(3));
    hm.insert("425".to_owned(), DataLength::fixed(3));
    hm.insert("426".to_owned(), DataLength::fixed(3));

    hm
});

static THREE_DIGIT_PLUS_DIGIT_DATA_LENGTH: Lazy<HashMap<String, DataLength>> = Lazy::new(|| {
    let mut hm = HashMap::new();
    for i in 310..=316 {
        // for (int i = 310; i <= 316; i++) {
        hm.insert(i.to_string(), DataLength::fixed(6));
    }
    for i in 320..=336 {
        // for (int i = 320; i <= 336; i++) {
        hm.insert(i.to_string(), DataLength::fixed(6));
    }
    for i in 340..=357 {
        // for (int i = 340; i <= 357; i++) {
        hm.insert(i.to_string(), DataLength::fixed(6));
    }
    for i in 360..=369 {
        // for (int i = 360; i <= 369; i++) {
        hm.insert(i.to_string(), DataLength::fixed(6));
    }
    hm.insert("390".to_owned(), DataLength::variable(15));
    hm.insert("391".to_owned(), DataLength::variable(18));
    hm.insert("392".to_owned(), DataLength::variable(15));
    hm.insert("393".to_owned(), DataLength::variable(18));
    hm.insert("703".to_owned(), DataLength::variable(30));

    hm
});

static FOUR_DIGIT_DATA_LENGTH: Lazy<HashMap<String, DataLength>> = Lazy::new(|| {
    let mut hm = HashMap::new();
    hm.insert("7001".to_owned(), DataLength::fixed(13));
    hm.insert("7002".to_owned(), DataLength::variable(30));
    hm.insert("7003".to_owned(), DataLength::fixed(10));
    hm.insert("8001".to_owned(), DataLength::fixed(14));
    hm.insert("8002".to_owned(), DataLength::variable(20));
    hm.insert("8003".to_owned(), DataLength::variable(30));
    hm.insert("8004".to_owned(), DataLength::variable(30));
    hm.insert("8005".to_owned(), DataLength::fixed(6));
    hm.insert("8006".to_owned(), DataLength::fixed(18));
    hm.insert("8007".to_owned(), DataLength::variable(30));
    hm.insert("8008".to_owned(), DataLength::variable(12));
    hm.insert("8018".to_owned(), DataLength::fixed(18));
    hm.insert("8020".to_owned(), DataLength::variable(25));
    hm.insert("8100".to_owned(), DataLength::fixed(6));
    hm.insert("8101".to_owned(), DataLength::fixed(10));
    hm.insert("8102".to_owned(), DataLength::fixed(2));
    hm.insert("8110".to_owned(), DataLength::variable(70));
    hm.insert("8200".to_owned(), DataLength::variable(70));

    hm
});

pub fn parseFieldsInGeneralPurpose(rawInformation: &str) -> Result<String, Exceptions> {
    if rawInformation.is_empty() {
        return Ok("".to_owned());
    }

    // Processing 2-digit AIs

    if rawInformation.chars().count() < 2 {
        return Err(Exceptions::NotFoundException(None));
    }

    let lookup: String = rawInformation.chars().take(2).collect();
    let twoDigitDataLength = TWO_DIGIT_DATA_LENGTH.get(&lookup);
    if let Some(tddl) = twoDigitDataLength {
        if tddl.variable {
            return processVariableAI(2, tddl.length, rawInformation);
        }
        return processFixedAI(2, tddl.length, rawInformation);
    }

    if rawInformation.chars().count() < 3 {
        return Err(Exceptions::NotFoundException(None));
    }

    let firstThreeDigits: String = rawInformation.chars().take(3).collect(); //rawInformation.substring(0, 3);
    let threeDigitDataLength = THREE_DIGIT_DATA_LENGTH.get(&firstThreeDigits);
    if let Some(tddl) = threeDigitDataLength {
        if tddl.variable {
            return processVariableAI(3, tddl.length, rawInformation);
        }
        return processFixedAI(3, tddl.length, rawInformation);
    }

    if rawInformation.chars().count() < 4 {
        return Err(Exceptions::NotFoundException(None));
    }

    let threeDigitPlusDigitDataLength = THREE_DIGIT_PLUS_DIGIT_DATA_LENGTH.get(&firstThreeDigits);
    if let Some(tdpddl) = threeDigitPlusDigitDataLength {
        if tdpddl.variable {
            return processVariableAI(4, tdpddl.length, rawInformation);
        }
        return processFixedAI(4, tdpddl.length, rawInformation);
    }

    let lookup: String = rawInformation.chars().take(4).collect();
    let firstFourDigitLength = FOUR_DIGIT_DATA_LENGTH.get(&lookup /*(0, 4)*/);
    if let Some(ffdl) = firstFourDigitLength {
        if ffdl.variable {
            return processVariableAI(4, ffdl.length, rawInformation);
        }
        return processFixedAI(4, ffdl.length, rawInformation);
    }

    Err(Exceptions::NotFoundException(None))
}

fn processFixedAI(
    aiSize: usize,
    fieldSize: usize,
    rawInformation: &str,
) -> Result<String, Exceptions> {
    if rawInformation.chars().count() < aiSize {
        return Err(Exceptions::NotFoundException(None));
    }

    let ai: String = rawInformation.chars().take(aiSize).collect();

    if rawInformation.chars().count() < aiSize + fieldSize {
        return Err(Exceptions::NotFoundException(None));
    }

    let field: String = rawInformation
        .chars()
        .skip(aiSize)
        .take(fieldSize)
        .collect(); //rawInformation.substring(aiSize, aiSize + fieldSize);
    let remaining: String = rawInformation.chars().skip(aiSize + fieldSize).collect(); // rawInformation.substring(aiSize + fieldSize);
    let result = format!("({ai}){field}");
    let parsedAI = parseFieldsInGeneralPurpose(&remaining)?;

    Ok(if parsedAI.is_empty() {
        result
    } else {
        format!("{result}{parsedAI}")
    })
}

fn processVariableAI(
    aiSize: usize,
    variableFieldSize: usize,
    rawInformation: &str,
) -> Result<String, Exceptions> {
    let ai: String = rawInformation.chars().take(aiSize).collect(); //rawInformation.substring(0, aiSize);
    let maxSize = rawInformation
        .chars()
        .count()
        .min(aiSize + variableFieldSize);
    let field: String = rawInformation.chars().skip(aiSize).take(maxSize).collect(); // (aiSize, maxSize);
    let remaining: String = rawInformation.chars().skip(maxSize).collect();
    let result = format!("({ai}){field}"); //'(' + ai + ')' + field;
    let parsedAI = parseFieldsInGeneralPurpose(&remaining)?;

    Ok(if parsedAI.is_empty() {
        result
    } else {
        format!("{result}{parsedAI}")
    })
}

struct DataLength {
    pub variable: bool,
    pub length: usize,
}
impl DataLength {
    // fn new( variable:bool, length:u32) -> Self{
    //   Self(variable,length)
    // }

    pub fn fixed(length: usize) -> Self {
        Self {
            variable: false,
            length,
        }
    }

    pub fn variable(length: usize) -> Self {
        Self {
            variable: true,
            length,
        }
    }
}

/**
 * @author Pablo Orduña, University of Deusto (pablo.orduna@deusto.es)
 * @author Eduardo Castillejo, University of Deusto (eduardo.castillejo@deusto.es)
 */
#[cfg(test)]
mod FieldParserTest {

    fn checkFields(expected: &str) {
        let field = expected.replace(['(', ')'], "");
        let actual = super::parseFieldsInGeneralPurpose(&field).expect("parse");
        assert_eq!(expected, actual);
    }

    #[test]
    fn testParseField() {
        checkFields("(15)991231(3103)001750(10)12A");
    }

    #[test]
    fn testParseField2() {
        checkFields("(15)991231(15)991231(3103)001750(10)12A");
    }
}
