/*
 * Copyright 2007 ZXing authors
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
 * <p>See
 * <a href="http://www.nttdocomo.co.jp/english/service/imode/make/content/barcode/about/s2.html">
 * DoCoMo's documentation</a> about the result types represented by subclasses of this class.</p>
 *
 * <p>Thanks to Jeff Griffin for proposing rewrite of these classes that relies less
 * on exception-based mechanisms during parsing.</p>
 *
 * @author Sean Owen
 */
struct AbstractDoCoMoResultParser {
    super: ResultParser;
}

impl AbstractDoCoMoResultParser {

    fn  match_do_co_mo_prefixed_field( prefix: &String,  raw_text: &String) -> Vec<String>  {
        return match_prefixed_field(&prefix, &raw_text, ';', true);
    }

    fn  match_single_do_co_mo_prefixed_field( prefix: &String,  raw_text: &String,  trim: bool) -> String  {
        return match_single_prefixed_field(&prefix, &raw_text, ';', trim);
    }
}

