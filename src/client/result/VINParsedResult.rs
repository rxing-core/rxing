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

// package com.google.zxing.client.result;

use super::{ParsedRXingResult, ParsedRXingResultType};

/**
 * Represents a parsed result that encodes a Vehicle Identification Number (VIN).
 */
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct VINParsedRXingResult {
    vin: String,
    world_manufacturer_id: String,
    vehicle_descriptor_section: String,
    vehicle_identifier_section: String,
    country_code: String,
    vehicle_attributes: String,
    model_year: u32,
    plant_code: char,
    sequential_number: String,
}

impl ParsedRXingResult for VINParsedRXingResult {
    fn getType(&self) -> super::ParsedRXingResultType {
        ParsedRXingResultType::Vin
    }

    fn getDisplayRXingResult(&self) -> String {
        let mut result = String::with_capacity(50);
        result.push_str(&self.world_manufacturer_id);
        result.push(' ');
        result.push_str(&self.vehicle_descriptor_section);
        result.push(' ');
        result.push_str(&self.vehicle_identifier_section);
        result.push('\n');
        if !self.country_code.is_empty() {
            result.push_str(&self.country_code);
            result.push(' ');
        }
        result.push_str(&self.model_year.to_string());
        result.push(' ');
        result.push(self.plant_code);
        result.push(' ');
        result.push_str(&self.sequential_number);
        result.push('\n');

        result
    }
}
impl VINParsedRXingResult {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        vin: String,
        world_manufacturer_id: String,
        vehicle_descriptor_section: String,
        vehicle_identifier_section: String,
        country_code: String,
        vehicle_attributes: String,
        model_year: u32,
        plant_code: char,
        sequential_number: String,
    ) -> Self {
        Self {
            vin,
            world_manufacturer_id,
            vehicle_descriptor_section,
            vehicle_identifier_section,
            country_code,
            vehicle_attributes,
            model_year,
            plant_code,
            sequential_number,
        }
    }

    pub fn getVIN(&self) -> &str {
        &self.vin
    }

    pub fn getWorldManufacturerID(&self) -> &str {
        &self.world_manufacturer_id
    }

    pub fn getVehicleDescriptorSection(&self) -> &str {
        &self.vehicle_descriptor_section
    }

    pub fn getVehicleIdentifierSection(&self) -> &str {
        &self.vehicle_identifier_section
    }

    pub fn getCountryCode(&self) -> &str {
        &self.country_code
    }

    pub fn getVehicleAttributes(&self) -> &str {
        &self.vehicle_attributes
    }

    pub fn getModelYear(&self) -> u32 {
        self.model_year
    }

    pub fn getPlantCode(&self) -> char {
        self.plant_code
    }

    pub fn getSequentialNumber(&self) -> &str {
        &self.sequential_number
    }
}
