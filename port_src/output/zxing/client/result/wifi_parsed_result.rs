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
// package com::google::zxing::client::result;

/**
 * Represents a parsed result that encodes wifi network information, like SSID and password.
 *
 * @author Vikram Aggarwal
 */
pub struct WifiParsedResult {
    super: ParsedResult;

     let ssid: String;

     let network_encryption: String;

     let password: String;

     let hidden: bool;

     let identity: String;

     let anonymous_identity: String;

     let eap_method: String;

     let phase2_method: String;
}

impl WifiParsedResult {

    pub fn new( network_encryption: &String,  ssid: &String,  password: &String) -> WifiParsedResult {
        this(&network_encryption, &ssid, &password, false);
    }

    pub fn new( network_encryption: &String,  ssid: &String,  password: &String,  hidden: bool) -> WifiParsedResult {
        this(&network_encryption, &ssid, &password, hidden, null, null, null, null);
    }

    pub fn new( network_encryption: &String,  ssid: &String,  password: &String,  hidden: bool,  identity: &String,  anonymous_identity: &String,  eap_method: &String,  phase2_method: &String) -> WifiParsedResult {
        super(ParsedResultType::WIFI);
        let .ssid = ssid;
        let .networkEncryption = network_encryption;
        let .password = password;
        let .hidden = hidden;
        let .identity = identity;
        let .anonymousIdentity = anonymous_identity;
        let .eapMethod = eap_method;
        let .phase2Method = phase2_method;
    }

    pub fn  get_ssid(&self) -> String  {
        return self.ssid;
    }

    pub fn  get_network_encryption(&self) -> String  {
        return self.network_encryption;
    }

    pub fn  get_password(&self) -> String  {
        return self.password;
    }

    pub fn  is_hidden(&self) -> bool  {
        return self.hidden;
    }

    pub fn  get_identity(&self) -> String  {
        return self.identity;
    }

    pub fn  get_anonymous_identity(&self) -> String  {
        return self.anonymous_identity;
    }

    pub fn  get_eap_method(&self) -> String  {
        return self.eap_method;
    }

    pub fn  get_phase2_method(&self) -> String  {
        return self.phase2_method;
    }

    pub fn  get_display_result(&self) -> String  {
         let result: StringBuilder = StringBuilder::new(80);
        maybe_append(&self.ssid, &result);
        maybe_append(&self.network_encryption, &result);
        maybe_append(&self.password, &result);
        maybe_append(&Boolean::to_string(self.hidden), &result);
        return result.to_string();
    }
}

