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

// package com.google.zxing.client.result;

use super::{ParsedRXingResult, ParsedRXingResultType, ResultParser};

/**
 * Represents a parsed result that encodes wifi network information, like SSID and password.
 *
 * @author Vikram Aggarwal
 */
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct WifiParsedRXingResult {
    ssid: String,
    networkEncryption: String,
    password: String,
    hidden: bool,
    identity: String,
    anonymousIdentity: String,
    eapMethod: String,
    phase2Method: String,
}

impl ParsedRXingResult for WifiParsedRXingResult {
    fn getType(&self) -> super::ParsedRXingResultType {
        ParsedRXingResultType::WIFI
    }

    fn getDisplayRXingResult(&self) -> String {
        let mut result = String::with_capacity(80);
        ResultParser::maybe_append_string(&self.ssid, &mut result);
        ResultParser::maybe_append_string(&self.networkEncryption, &mut result);
        ResultParser::maybe_append_string(&self.password, &mut result);
        ResultParser::maybe_append_string(&self.hidden.to_string(), &mut result);

        result
    }
}

impl WifiParsedRXingResult {
    pub fn new(networkEncryption: String, ssid: String, password: String) -> Self {
        Self::with_hidden(networkEncryption, ssid, password, false)
    }

    pub fn with_hidden(
        networkEncryption: String,
        ssid: String,
        password: String,
        hidden: bool,
    ) -> Self {
        Self::with_details(
            networkEncryption,
            ssid,
            password,
            hidden,
            String::default(),
            String::default(),
            String::default(),
            String::default(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn with_details(
        networkEncryption: String,
        ssid: String,
        password: String,
        hidden: bool,
        identity: String,
        anonymousIdentity: String,
        eapMethod: String,
        phase2Method: String,
    ) -> Self {
        Self {
            ssid,
            networkEncryption,
            password,
            hidden,
            identity,
            anonymousIdentity,
            eapMethod,
            phase2Method,
        }
    }

    pub fn getSsid(&self) -> &str {
        &self.ssid
    }

    pub fn getNetworkEncryption(&self) -> &str {
        &self.networkEncryption
    }

    pub fn getPassword(&self) -> &str {
        &self.password
    }

    pub fn isHidden(&self) -> bool {
        self.hidden
    }

    pub fn getIdentity(&self) -> &str {
        &self.identity
    }

    pub fn getAnonymousIdentity(&self) -> &str {
        &self.anonymousIdentity
    }

    pub fn getEapMethod(&self) -> &str {
        &self.eapMethod
    }

    pub fn getPhase2Method(&self) -> &str {
        &self.phase2Method
    }
}
