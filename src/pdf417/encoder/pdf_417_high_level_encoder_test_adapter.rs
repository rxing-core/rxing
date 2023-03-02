/*
 * Copyright 2022 ZXing authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::common::{CharacterSetECI, Result};

use super::{pdf_417_high_level_encoder, Compaction};

/**
 * Test adapter for PDF417HighLevelEncoder to be called solely from unit tests.
 */

pub fn encodeHighLevel(
    msg: &str,
    compaction: Compaction,
    encoding: Option<CharacterSetECI>,
    autoECI: bool,
) -> Result<String> {
    pdf_417_high_level_encoder::encodeHighLevel(msg, compaction, encoding, autoECI)
}
