/*
 * Copyright 2014 ZXing authors
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
 * Represents a parsed result that encodes a Vehicle Identification Number (VIN).
 */
pub struct VINParsedResult {
    super: ParsedResult;

     let vin: String;

     let world_manufacturer_i_d: String;

     let vehicle_descriptor_section: String;

     let vehicle_identifier_section: String;

     let country_code: String;

     let vehicle_attributes: String;

     let model_year: i32;

     let plant_code: char;

     let sequential_number: String;
}

impl VINParsedResult {

    pub fn new( vin: &String,  world_manufacturer_i_d: &String,  vehicle_descriptor_section: &String,  vehicle_identifier_section: &String,  country_code: &String,  vehicle_attributes: &String,  model_year: i32,  plant_code: char,  sequential_number: &String) -> VINParsedResult {
        super(ParsedResultType::VIN);
        let .vin = vin;
        let .worldManufacturerID = world_manufacturer_i_d;
        let .vehicleDescriptorSection = vehicle_descriptor_section;
        let .vehicleIdentifierSection = vehicle_identifier_section;
        let .countryCode = country_code;
        let .vehicleAttributes = vehicle_attributes;
        let .modelYear = model_year;
        let .plantCode = plant_code;
        let .sequentialNumber = sequential_number;
    }

    pub fn  get_v_i_n(&self) -> String  {
        return self.vin;
    }

    pub fn  get_world_manufacturer_i_d(&self) -> String  {
        return self.world_manufacturer_i_d;
    }

    pub fn  get_vehicle_descriptor_section(&self) -> String  {
        return self.vehicle_descriptor_section;
    }

    pub fn  get_vehicle_identifier_section(&self) -> String  {
        return self.vehicle_identifier_section;
    }

    pub fn  get_country_code(&self) -> String  {
        return self.country_code;
    }

    pub fn  get_vehicle_attributes(&self) -> String  {
        return self.vehicle_attributes;
    }

    pub fn  get_model_year(&self) -> i32  {
        return self.model_year;
    }

    pub fn  get_plant_code(&self) -> char  {
        return self.plant_code;
    }

    pub fn  get_sequential_number(&self) -> String  {
        return self.sequential_number;
    }

    pub fn  get_display_result(&self) -> String  {
         let result: StringBuilder = StringBuilder::new(50);
        result.append(&self.world_manufacturer_i_d).append(' ');
        result.append(&self.vehicle_descriptor_section).append(' ');
        result.append(&self.vehicle_identifier_section).append('\n');
        if self.country_code != null {
            result.append(&self.country_code).append(' ');
        }
        result.append(self.model_year).append(' ');
        result.append(self.plant_code).append(' ');
        result.append(&self.sequential_number).append('\n');
        return result.to_string();
    }
}

