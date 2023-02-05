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

// /**
//  * @author Antonio Manuel Benjumea Conde, Servinform, S.A.
//  * @author Agustín Delgado, Servinform, S.A.
//  */
// public final class ExpandedProductParsedRXingResultTestCase extends Assert {

use std::collections::HashMap;

use crate::{client::result::ParsedClientResult, BarcodeFormat, RXingResult};

use super::ExpandedProductResultParser;

#[test]
fn testRSSExpanded() {
    let mut uncommonAIs = HashMap::new();
    uncommonAIs.insert("123", "544654");
    let result = RXingResult::new(
        "(01)66546(13)001205(3932)4455(3102)6544(123)544654",
        Vec::new(),
        Vec::new(),
        BarcodeFormat::RSS_EXPANDED,
    );
    let o = ExpandedProductResultParser::parse(&result);
    if let Some(res) = o {
        if let ParsedClientResult::ExpandedProductResult(epr_res) = res {
            assert_eq!("66546", epr_res.getProductID());
            assert!(epr_res.getSscc().is_empty());
            assert!(epr_res.getLotNumber().is_empty());
            assert!(epr_res.getProductionDate().is_empty());
            assert_eq!("001205", epr_res.getPackagingDate());
            assert!(epr_res.getBestBeforeDate().is_empty());
            assert!(epr_res.getExpirationDate().is_empty());
            assert_eq!("6544", epr_res.getWeight());
            assert_eq!("KG", epr_res.getWeightType());
            assert_eq!("2", epr_res.getWeightIncrement());
            assert_eq!("5", epr_res.getPrice());
            assert_eq!("2", epr_res.getPriceIncrement());
            assert_eq!("445", epr_res.getPriceCurrency());
            assert_eq!(uncommonAIs.len(), epr_res.getUncommonAIs().len());
            for (k, v) in uncommonAIs {
                if epr_res.getUncommonAIs().contains_key(k) {
                    let ev = epr_res.getUncommonAIs().get(k).unwrap();
                    assert_eq!(v, ev);
                } else {
                    panic!("key not found {k}")
                }
            }
            // assert_eq!(&uncommonAIs, epr_res.getUncommonAIs());
        } else {
            panic!("Should have gotten a expanded product");
        }
    } else {
        panic!("Should have found a result");
    }
}
// }
