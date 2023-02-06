/*
 * Copyright 2010 ZXing authors
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

use crate::client::result::{ParsedClientResult, WifiParsedRXingResult};

use super::ResultParser;

/**
 * <p>Parses a WIFI configuration string. Strings will be of the form:</p>
 *
 * <p>{@code WIFI:T:[network type];S:[network SSID];P:[network password];H:[hidden?];;}</p>
 *
 * <p>For WPA2 enterprise (EAP), strings will be of the form:</p>
 *
 * <p>{@code WIFI:T:WPA2-EAP;S:[network SSID];H:[hidden?];E:[EAP method];PH2:[Phase 2 method];A:[anonymous identity];I:[username];P:[password];;}</p>
 *
 * <p>"EAP method" can e.g. be "TTLS" or "PWD" or one of the other fields in <a href="https://developer.android.com/reference/android/net/wifi/WifiEnterpriseConfig.Eap.html">WifiEnterpriseConfig.Eap</a> and "Phase 2 method" can e.g. be "MSCHAPV2" or any of the other fields in <a href="https://developer.android.com/reference/android/net/wifi/WifiEnterpriseConfig.Phase2.html">WifiEnterpriseConfig.Phase2</a></p>
 *
 * <p>The fields can appear in any order. Only "S:" is required.</p>
 *
 * @author Vikram Aggarwal
 * @author Sean Owen
 * @author Steffen Kieß
 */
pub fn parse(theRXingResult: &crate::RXingResult) -> Option<super::ParsedClientResult> {
    const WIFI_TEST: &str = "WIFI:";

    let rawText_unstripped = ResultParser::getMassagedText(theRXingResult);
    if !rawText_unstripped.starts_with(WIFI_TEST) {
        return None;
    }
    let rawText = rawText_unstripped[WIFI_TEST.len()..].to_owned();
    let ssid =
        ResultParser::matchSinglePrefixedField("S:", &rawText, ';', false).unwrap_or_default();

    if ssid.is_empty() {
        return None;
    }

    let pass =
        ResultParser::matchSinglePrefixedField("P:", &rawText, ';', false).unwrap_or_default();

    let n_type = ResultParser::matchSinglePrefixedField("T:", &rawText, ';', false)
        .unwrap_or(String::from("nopass"));

    // Unfortunately, in the past, H: was not just used for boolean 'hidden', but 'phase 2 method'.
    // To try to retain backwards compatibility, we set one or the other based on whether the string
    // is 'true' or 'false':
    let mut hidden = false;
    let mut phase2Method = ResultParser::matchSinglePrefixedField("PH2:", &rawText, ';', false);
    let _hValue = if let Some(hv) =
        ResultParser::matchSinglePrefixedField("H:", &rawText, ';', false)
    {
        // If PH2 was specified separately, or if the value is clearly boolean, interpret it as 'hidden'
        if phase2Method.is_some() || "true" == hv.to_lowercase() || "false" == hv.to_lowercase() {
            hidden = hv.parse().ok()?;
        } else {
            phase2Method = Some(hv);
        }
        
    };

    let identity =
        ResultParser::matchSinglePrefixedField("I:", &rawText, ';', false).unwrap_or_default();
    let anonymousIdentity =
        ResultParser::matchSinglePrefixedField("A:", &rawText, ';', false).unwrap_or_default();
    let eapMethod =
        ResultParser::matchSinglePrefixedField("E:", &rawText, ';', false).unwrap_or_default();

    Some(ParsedClientResult::WiFiResult(
        WifiParsedRXingResult::with_details(
            n_type,
            ssid,
            pass,
            hidden,
            identity,
            anonymousIdentity,
            eapMethod,
            phase2Method.unwrap_or_default(),
        ),
    ))
}
// }
