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
 */
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{common::BitArray, Exceptions};

static ONE: Lazy<Regex> = Lazy::new(|| Regex::new("1").unwrap());
static ZERO: Lazy<Regex> = Lazy::new(|| Regex::new("0").unwrap());
static SPACE: Lazy<Regex> = Lazy::new(|| Regex::new(" ").unwrap());

/*
 * Constructs a BitArray from a String like the one returned from BitArray.toString()
 */
pub fn buildBitArrayFromString(data: &str) -> Result<BitArray, Exceptions> {
    let dotsAndXs = ZERO
        .replace_all(&ONE.replace_all(data, "X"), ".")
        .to_string();
    let mut binary = BitArray::with_size(SPACE.replace_all(&dotsAndXs, "").chars().count());
    let mut counter = 0;

    for i in 0..dotsAndXs.chars().count() {
        // for (int i = 0; i < dotsAndXs.length(); ++i) {
        if i % 9 == 0 {
            // spaces
            if dotsAndXs.chars().nth(i).ok_or(Exceptions::parseEmpty())? != ' ' {
                return Err(Exceptions::illegalState("space expected".to_owned()));
            }
            continue;
        }

        let currentChar = dotsAndXs.chars().nth(i).ok_or(Exceptions::parseEmpty())?;
        if currentChar == 'X' || currentChar == 'x' {
            binary.set(counter);
        }
        counter += 1;
    }
    Ok(binary)
}

pub fn buildBitArrayFromStringWithoutSpaces(data: &str) -> Result<BitArray, Exceptions> {
    let mut sb = String::new();

    // let dotsAndXs = ZERO.matcher(ONE.matcher(data).replaceAll("X")).replaceAll(".");
    let dotsAndXs = ZERO
        .replace_all(&ONE.replace_all(data, "X"), ".")
        .to_string();
    let mut current = 0;
    let dotsAndXs_length = dotsAndXs.chars().count();
    while current < dotsAndXs_length {
        sb.push(' ');
        let mut i = 0;
        while i < 8 && current < dotsAndXs_length {
            sb.push(
                dotsAndXs
                    .chars()
                    .nth(current)
                    .ok_or(Exceptions::parseEmpty())?,
            );
            current += 1;

            i += 1;
        }
    }

    buildBitArrayFromString(&sb)
}

/**
 * @author Pablo Orduña, University of Deusto (pablo.orduna@deusto.es)
 */
#[cfg(test)]
mod BinaryUtilTest {

    #[test]
    fn testBuildBitArrayFromString() {
        let data = " ..X..X.. ..XXX... XXXXXXXX ........";
        check(data);

        let data = " XXX..X..";
        check(data);

        let data = " XX";
        check(data);

        let data = " ....XX.. ..XX";
        check(data);

        let data = " ....XX.. ..XX..XX ....X.X. ........";
        check(data);
    }

    fn check(data: &str) {
        let binary = super::buildBitArrayFromString(data).expect("check");
        assert_eq!(data, binary.to_string());
    }

    #[test]
    fn testBuildBitArrayFromStringWithoutSpaces() {
        let data = " ..X..X.. ..XXX... XXXXXXXX ........";
        checkWithoutSpaces(data);

        let data = " XXX..X..";
        checkWithoutSpaces(data);

        let data = " XX";
        checkWithoutSpaces(data);

        let data = " ....XX.. ..XX";
        checkWithoutSpaces(data);

        let data = " ....XX.. ..XX..XX ....X.X. ........";
        checkWithoutSpaces(data);
    }

    fn checkWithoutSpaces(data: &str) {
        let dataWithoutSpaces = super::SPACE.replace_all(data, "");
        let binary =
            super::buildBitArrayFromStringWithoutSpaces(&dataWithoutSpaces).expect("success");
        assert_eq!(data, binary.to_string());
    }
}
