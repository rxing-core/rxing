/*
 * Copyright 2021 ZXing authors
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

use std::{fmt, rc::Rc};

use encoding::EncodingRef;

use crate::{
    common::{BitArray, ECIEncoderSet, Result},
    qrcode::decoder::{ErrorCorrectionLevel, Mode, Version, VersionRef},
    Exceptions,
};

use unicode_segmentation::UnicodeSegmentation;

use super::qrcode_encoder;

pub enum VersionSize {
    SMALL,  //("version 1-9"),
    MEDIUM, //("version 10-26"),
    LARGE,  //("version 27-40");

            // private final String description;

            // VersionSize(String description) {
            //   this.description = description;
            // }

            // public String toString() {
            //   return description;
            // }
}

impl fmt::Display for VersionSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                VersionSize::SMALL => "version 1-9",
                VersionSize::MEDIUM => "version 10-26",
                VersionSize::LARGE => "version 27-40",
            }
        )
    }
}

/**
 * Encoder that encodes minimally
 *
 * Algorithm:
 *
 * The eleventh commandment was "Thou Shalt Compute" or "Thou Shalt Not Compute" - I forget which (Alan Perilis).
 *
 * This implementation computes. As an alternative, the QR-Code specification suggests heuristics like this one:
 *
 * If initial input data is in the exclusive subset of the Alphanumeric character set AND if there are less than
 * [6,7,8] characters followed by data from the remainder of the 8-bit byte character set, THEN select the 8-
 * bit byte mode ELSE select Alphanumeric mode;
 *
 * This is probably right for 99.99% of cases but there is at least this one counter example: The string "AAAAAAa"
 * encodes 2 bits smaller as ALPHANUMERIC(AAAAAA), BYTE(a) than by encoding it as BYTE(AAAAAAa).
 * Perhaps that is the only counter example but without having proof, it remains unclear.
 *
 * ECI switching:
 *
 * In multi language content the algorithm selects the most compact representation using ECI modes.
 * For example the most compact representation of the string "\u0150\u015C" (O-double-acute, S-circumflex) is
 * ECI(UTF-8), BYTE(\u0150\u015C) while prepending one or more times the same leading character as in
 * "\u0150\u0150\u015C", the most compact representation uses two ECIs so that the string is encoded as
 * ECI(ISO-8859-2), BYTE(\u0150\u0150), ECI(ISO-8859-3), BYTE(\u015C).
 *
 * @author Alex Geller
 */
pub struct MinimalEncoder {
    stringToEncode: Vec<String>,
    isGS1: bool,
    encoders: ECIEncoderSet,
    ecLevel: ErrorCorrectionLevel,
}

impl MinimalEncoder {
    /**
     * Creates a MinimalEncoder
     *
     * @param stringToEncode The string to encode
     * @param priorityCharset The preferred {@link Charset}. When the value of the argument is null, the algorithm
     *   chooses charsets that leads to a minimal representation. Otherwise the algorithm will use the priority
     *   charset to encode any character in the input that can be encoded by it if the charset is among the
     *   supported charsets.
     * @param isGS1 {@code true} if a FNC1 is to be prepended; {@code false} otherwise
     * @param ecLevel The error correction level.
     * @see RXingResultList#getVersion
     */
    pub fn new(
        stringToEncode: &str,
        priorityCharset: Option<EncodingRef>,
        isGS1: bool,
        ecLevel: ErrorCorrectionLevel,
    ) -> Self {
        Self {
            stringToEncode: stringToEncode
                .graphemes(true)
                .map(|p| p.to_owned())
                .collect::<Vec<String>>(),
            isGS1,
            encoders: ECIEncoderSet::new(stringToEncode, priorityCharset, None),
            ecLevel,
        }
    }

    /**
     * Encodes the string minimally
     *
     * @param stringToEncode The string to encode
     * @param version The preferred {@link Version}. A minimal version is computed (see
     *   {@link RXingResultList#getVersion method} when the value of the argument is null
     * @param priorityCharset The preferred {@link Charset}. When the value of the argument is null, the algorithm
     *   chooses charsets that leads to a minimal representation. Otherwise the algorithm will use the priority
     *   charset to encode any character in the input that can be encoded by it if the charset is among the
     *   supported charsets.
     * @param isGS1 {@code true} if a FNC1 is to be prepended; {@code false} otherwise
     * @param ecLevel The error correction level.
     * @return An instance of {@code RXingResultList} representing the minimal solution.
     * @see RXingResultList#getBits
     * @see RXingResultList#getVersion
     * @see RXingResultList#getSize
     */
    pub fn encode_with_details(
        stringToEncode: &str,
        version: Option<VersionRef>,
        priorityCharset: Option<EncodingRef>,
        isGS1: bool,
        ecLevel: ErrorCorrectionLevel,
    ) -> Result<RXingResultList> {
        MinimalEncoder::new(stringToEncode, priorityCharset, isGS1, ecLevel).encode(version)
    }

    pub fn encode(&self, version: Option<VersionRef>) -> Result<RXingResultList> {
        if let Some(version) = version {
            // compute minimal encoding for a given version
            let result = self.encodeSpecificVersion(version)?;
            if !qrcode_encoder::willFit(
                result.getSize(),
                Self::getVersion(Self::getVersionSize(result.getVersion()))?,
                &self.ecLevel,
            ) {
                return Err(Exceptions::writerWith(format!(
                    "Data too big for version {version}"
                )));
            }
            Ok(result)
        } else {
            // compute minimal encoding trying the three version sizes.
            let versions = [
                Self::getVersion(VersionSize::SMALL)?,
                Self::getVersion(VersionSize::MEDIUM)?,
                Self::getVersion(VersionSize::LARGE)?,
            ];
            let results = [
                self.encodeSpecificVersion(versions[0])?,
                self.encodeSpecificVersion(versions[1])?,
                self.encodeSpecificVersion(versions[2])?,
            ];
            let mut smallestSize = u32::MAX;
            let mut smallestRXingResult: i32 = -1;
            for i in 0..3 {
                let size = results[i].getSize();
                if qrcode_encoder::willFit(size, versions[i], &self.ecLevel) && size < smallestSize
                {
                    smallestSize = size;
                    smallestRXingResult = i as i32;
                }
            }
            if smallestRXingResult < 0 {
                return Err(Exceptions::writerWith("Data too big for any version"));
            }
            Ok(results[smallestRXingResult as usize].clone())
        }
    }

    pub fn getVersionSize(version: VersionRef) -> VersionSize {
        match version.getVersionNumber() {
            0..=9 => VersionSize::SMALL,
            10..=26 => VersionSize::MEDIUM,
            _ => VersionSize::LARGE,
        }
    }

    pub fn getVersion(versionSize: VersionSize) -> Result<VersionRef> {
        match versionSize {
            VersionSize::SMALL => Version::getVersionForNumber(9),
            VersionSize::MEDIUM => Version::getVersionForNumber(26),
            VersionSize::LARGE => Version::getVersionForNumber(40),
        }
    }

    pub fn isNumeric(c: &str) -> bool {
        if c.len() == 1 {
            if let Some(ch) = c.chars().next() {
                return ('0'..='9').contains(&ch);
            }
        }
        false
    }

    pub fn isDoubleByteKanji(c: &str) -> bool {
        qrcode_encoder::isOnlyDoubleByteKanji(c)
    }

    pub fn isAlphanumeric(c: &str) -> bool {
        if c.len() == 1 {
            if let Some(ch) = c.chars().next() {
                return qrcode_encoder::getAlphanumericCode(ch as u32) != -1;
            }
        }
        false
    }

    pub fn canEncode(&self, mode: &Mode, c: &str) -> bool {
        match mode {
            Mode::NUMERIC => Self::isNumeric(c),
            Mode::ALPHANUMERIC => Self::isAlphanumeric(c),
            Mode::BYTE => true,
            Mode::KANJI => Self::isDoubleByteKanji(c),
            _ => false, // any character can be encoded as byte(s). Up to the caller to manage splitting into
                        // multiple bytes when String.getBytes(Charset) return more than one byte.
        }
    }

    pub fn getCompactedOrdinal(mode: Option<Mode>) -> Result<u32> {
        match mode {
            Some(Mode::NUMERIC) => Ok(2),
            Some(Mode::ALPHANUMERIC) => Ok(1),
            Some(Mode::BYTE) => Ok(3),
            Some(Mode::KANJI) | None => Ok(0),
            _ => Err(Exceptions::illegalArgumentWith(format!(
                "Illegal mode {mode:?}"
            ))),
        }
    }

    pub fn addEdge(
        &self,
        edges: &mut [Vec<Vec<Option<Rc<Edge>>>>],
        position: usize,
        edge: Option<Rc<Edge>>,
    ) -> Result<()> {
        let vertexIndex = position
            + edge
                .as_ref()
                .ok_or(Exceptions::FormatException(None))?
                .characterLength as usize;
        let modeEdges = &mut edges[vertexIndex][edge
            .as_ref()
            .ok_or(Exceptions::FormatException(None))?
            .charsetEncoderIndex];
        let modeOrdinal = Self::getCompactedOrdinal(Some(
            edge.as_ref().ok_or(Exceptions::FormatException(None))?.mode,
        ))? as usize;
        if modeEdges[modeOrdinal].is_none()
            || modeEdges[modeOrdinal]
                .as_ref()
                .ok_or(Exceptions::format)?
                .cachedTotalSize
                > edge.as_ref().ok_or(Exceptions::format)?.cachedTotalSize
        {
            modeEdges[modeOrdinal] = edge;
        }

        Ok(())
    }

    pub fn addEdges(
        &self,
        version: VersionRef,
        edges: &mut [Vec<Vec<Option<Rc<Edge>>>>],
        from: usize,
        previous: Option<Rc<Edge>>,
    ) -> Result<()> {
        let mut start = 0;
        let mut end = self.encoders.len();
        let priorityEncoderIndex = self.encoders.getPriorityEncoderIndex();
        if priorityEncoderIndex.is_some()
            && self
                .encoders
                .canEncode(
                    &self.stringToEncode[from],
                    priorityEncoderIndex.ok_or(Exceptions::format)?,
                )
                .ok_or(Exceptions::format)?
        {
            start = priorityEncoderIndex.ok_or(Exceptions::format)?;
            end = priorityEncoderIndex.ok_or(Exceptions::format)? + 1;
        }

        for i in start..end {
            if self
                .encoders
                .canEncode(
                    self.stringToEncode
                        .get(from)
                        .ok_or(Exceptions::indexOutOfBounds)?,
                    i,
                )
                .ok_or(Exceptions::format)?
            {
                self.addEdge(
                    edges,
                    from,
                    Some(Rc::new(
                        Edge::new(
                            Mode::BYTE,
                            from,
                            i,
                            1,
                            previous.clone(),
                            version,
                            self.encoders.clone(),
                            self.stringToEncode.clone(),
                        )
                        .ok_or(Exceptions::writer)?,
                    )),
                )?;
            }
        }

        if self.canEncode(
            &Mode::KANJI,
            self.stringToEncode.get(from).ok_or(Exceptions::format)?,
        ) {
            self.addEdge(
                edges,
                from,
                Some(Rc::new(
                    Edge::new(
                        Mode::KANJI,
                        from,
                        0,
                        1,
                        previous.clone(),
                        version,
                        self.encoders.clone(),
                        self.stringToEncode.clone(),
                    )
                    .ok_or(Exceptions::writer)?,
                )),
            )?;
        }

        let inputLength = self.stringToEncode.len();
        if self.canEncode(
            &Mode::ALPHANUMERIC,
            self.stringToEncode
                .get(from)
                .ok_or(Exceptions::indexOutOfBounds)?,
        ) {
            self.addEdge(
                edges,
                from,
                Some(Rc::new(
                    Edge::new(
                        Mode::ALPHANUMERIC,
                        from,
                        0,
                        if from + 1 >= inputLength
                            || !self.canEncode(
                                &Mode::ALPHANUMERIC,
                                self.stringToEncode
                                    .get(from + 1)
                                    .ok_or(Exceptions::indexOutOfBounds)?,
                            )
                        {
                            1
                        } else {
                            2
                        },
                        previous.clone(),
                        version,
                        self.encoders.clone(),
                        self.stringToEncode.clone(),
                    )
                    .ok_or(Exceptions::writer)?,
                )),
            )?;
        }

        if self.canEncode(
            &Mode::NUMERIC,
            self.stringToEncode
                .get(from)
                .ok_or(Exceptions::indexOutOfBounds)?,
        ) {
            self.addEdge(
                edges,
                from,
                Some(Rc::new(
                    Edge::new(
                        Mode::NUMERIC,
                        from,
                        0,
                        if from + 1 >= inputLength
                            || !self.canEncode(
                                &Mode::NUMERIC,
                                self.stringToEncode
                                    .get(from + 1)
                                    .ok_or(Exceptions::indexOutOfBounds)?,
                            )
                        {
                            1
                        } else if from + 2 >= inputLength
                            || !self.canEncode(
                                &Mode::NUMERIC,
                                self.stringToEncode
                                    .get(from + 2)
                                    .ok_or(Exceptions::indexOutOfBounds)?,
                            )
                        {
                            2
                        } else {
                            3
                        },
                        previous,
                        version,
                        self.encoders.clone(),
                        self.stringToEncode.clone(),
                    )
                    .ok_or(Exceptions::writer)?,
                )),
            )?;
        }

        Ok(())
    }
    pub fn encodeSpecificVersion(&self, version: VersionRef) -> Result<RXingResultList> {
        // @SuppressWarnings("checkstyle:lineLength")
        /* A vertex represents a tuple of a position in the input, a mode and a character encoding where position 0
         * denotes the position left of the first character, 1 the position left of the second character and so on.
         * Likewise the end vertices are located after the last character at position stringToEncode.length().
         *
         * An edge leading to such a vertex encodes one or more of the characters left of the position that the vertex
         * represents and encodes it in the same encoding and mode as the vertex on which the edge ends. In other words,
         * all edges leading to a particular vertex encode the same characters in the same mode with the same character
         * encoding. They differ only by their source vertices who are all located at i+1 minus the number of encoded
         * characters.
         *
         * The edges leading to a vertex are stored in such a way that there is a fast way to enumerate the edges ending
         * on a particular vertex.
         *
         * The algorithm processes the vertices in order of their position thereby performing the following:
         *
         * For every vertex at position i the algorithm enumerates the edges ending on the vertex and removes all but the
         * shortest from that list.
         * Then it processes the vertices for the position i+1. If i+1 == stringToEncode.length() then the algorithm ends
         * and chooses the the edge with the smallest size from any of the edges leading to vertices at this position.
         * Otherwise the algorithm computes all possible outgoing edges for the vertices at the position i+1
         *
         * Examples:
         * The process is illustrated by showing the graph (edges) after each iteration from left to right over the input:
         * An edge is drawn as follows "(" + fromVertex + ") -- " + encodingMode + "(" + encodedInput + ") (" +
         * accumulatedSize + ") --> (" + toVertex + ")"
         *
         * Example 1 encoding the string "ABCDE":
         * Note: This example assumes that alphanumeric encoding is only possible in multiples of two characters so that
         * the example is both short and showing the principle. In reality this restriction does not exist.
         *
         * Initial situation
         * (initial) -- BYTE(A) (20) --> (1_BYTE)
         * (initial) -- ALPHANUMERIC(AB)                     (24) --> (2_ALPHANUMERIC)
         *
         * Situation after adding edges to vertices at position 1
         * (initial) -- BYTE(A) (20) --> (1_BYTE) -- BYTE(B) (28) --> (2_BYTE)
         *                               (1_BYTE) -- ALPHANUMERIC(BC)                             (44) --> (3_ALPHANUMERIC)
         * (initial) -- ALPHANUMERIC(AB)                     (24) --> (2_ALPHANUMERIC)
         *
         * Situation after adding edges to vertices at position 2
         * (initial) -- BYTE(A) (20) --> (1_BYTE)
         * (initial) -- ALPHANUMERIC(AB)                     (24) --> (2_ALPHANUMERIC)
         * (initial) -- BYTE(A) (20) --> (1_BYTE) -- BYTE(B) (28) --> (2_BYTE)
         * (1_BYTE) -- ALPHANUMERIC(BC)                             (44) --> (3_ALPHANUMERIC)
         * (initial) -- ALPHANUMERIC(AB)                     (24) --> (2_ALPHANUMERIC) -- BYTE(C) (44) --> (3_BYTE)
         *                                                            (2_ALPHANUMERIC) -- ALPHANUMERIC(CD)                             (35) --> (4_ALPHANUMERIC)
         *
         * Situation after adding edges to vertices at position 3
         * (initial) -- BYTE(A) (20) --> (1_BYTE) -- BYTE(B) (28) --> (2_BYTE) -- BYTE(C)         (36) --> (3_BYTE)
         *                               (1_BYTE) -- ALPHANUMERIC(BC)                             (44) --> (3_ALPHANUMERIC) -- BYTE(D) (64) --> (4_BYTE)
         *                                                                                                 (3_ALPHANUMERIC) -- ALPHANUMERIC(DE)                             (55) --> (5_ALPHANUMERIC)
         * (initial) -- ALPHANUMERIC(AB)                     (24) --> (2_ALPHANUMERIC) -- ALPHANUMERIC(CD)                             (35) --> (4_ALPHANUMERIC)
         *                                                            (2_ALPHANUMERIC) -- ALPHANUMERIC(CD)                             (35) --> (4_ALPHANUMERIC)
         *
         * Situation after adding edges to vertices at position 4
         * (initial) -- BYTE(A) (20) --> (1_BYTE) -- BYTE(B) (28) --> (2_BYTE) -- BYTE(C)         (36) --> (3_BYTE) -- BYTE(D) (44) --> (4_BYTE)
         *                               (1_BYTE) -- ALPHANUMERIC(BC)                             (44) --> (3_ALPHANUMERIC) -- ALPHANUMERIC(DE)                             (55) --> (5_ALPHANUMERIC)
         * (initial) -- ALPHANUMERIC(AB)                     (24) --> (2_ALPHANUMERIC) -- ALPHANUMERIC(CD)                             (35) --> (4_ALPHANUMERIC) -- BYTE(E) (55) --> (5_BYTE)
         *
         * Situation after adding edges to vertices at position 5
         * (initial) -- BYTE(A) (20) --> (1_BYTE) -- BYTE(B) (28) --> (2_BYTE) -- BYTE(C)         (36) --> (3_BYTE) -- BYTE(D)         (44) --> (4_BYTE) -- BYTE(E)         (52) --> (5_BYTE)
         *                               (1_BYTE) -- ALPHANUMERIC(BC)                             (44) --> (3_ALPHANUMERIC) -- ALPHANUMERIC(DE)                             (55) --> (5_ALPHANUMERIC)
         * (initial) -- ALPHANUMERIC(AB)                     (24) --> (2_ALPHANUMERIC) -- ALPHANUMERIC(CD)                             (35) --> (4_ALPHANUMERIC)
         *
         * Encoding as BYTE(ABCDE) has the smallest size of 52 and is hence chosen. The encodation ALPHANUMERIC(ABCD),
         * BYTE(E) is longer with a size of 55.
         *
         * Example 2 encoding the string "XXYY" where X denotes a character unique to character set ISO-8859-2 and Y a
         * character unique to ISO-8859-3. Both characters encode as double byte in UTF-8:
         *
         * Initial situation
         * (initial) -- BYTE(X) (32) --> (1_BYTE_ISO-8859-2)
         * (initial) -- BYTE(X) (40) --> (1_BYTE_UTF-8)
         * (initial) -- BYTE(X) (40) --> (1_BYTE_UTF-16BE)
         *
         * Situation after adding edges to vertices at position 1
         * (initial) -- BYTE(X) (32) --> (1_BYTE_ISO-8859-2) -- BYTE(X) (40) --> (2_BYTE_ISO-8859-2)
         *                               (1_BYTE_ISO-8859-2) -- BYTE(X) (72) --> (2_BYTE_UTF-8)
         *                               (1_BYTE_ISO-8859-2) -- BYTE(X) (72) --> (2_BYTE_UTF-16BE)
         * (initial) -- BYTE(X) (40) --> (1_BYTE_UTF-8)
         * (initial) -- BYTE(X) (40) --> (1_BYTE_UTF-16BE)
         *
         * Situation after adding edges to vertices at position 2
         * (initial) -- BYTE(X) (32) --> (1_BYTE_ISO-8859-2) -- BYTE(X) (40) --> (2_BYTE_ISO-8859-2)
         *                                                                       (2_BYTE_ISO-8859-2) -- BYTE(Y) (72) --> (3_BYTE_ISO-8859-3)
         *                                                                       (2_BYTE_ISO-8859-2) -- BYTE(Y) (80) --> (3_BYTE_UTF-8)
         *                                                                       (2_BYTE_ISO-8859-2) -- BYTE(Y) (80) --> (3_BYTE_UTF-16BE)
         * (initial) -- BYTE(X) (40) --> (1_BYTE_UTF-8) -- BYTE(X) (56) --> (2_BYTE_UTF-8)
         * (initial) -- BYTE(X) (40) --> (1_BYTE_UTF-16BE) -- BYTE(X) (56) --> (2_BYTE_UTF-16BE)
         *
         * Situation after adding edges to vertices at position 3
         * (initial) -- BYTE(X) (32) --> (1_BYTE_ISO-8859-2) -- BYTE(X) (40) --> (2_BYTE_ISO-8859-2) -- BYTE(Y) (72) --> (3_BYTE_ISO-8859-3)
         *                                                                                                               (3_BYTE_ISO-8859-3) -- BYTE(Y) (80) --> (4_BYTE_ISO-8859-3)
         *                                                                                                               (3_BYTE_ISO-8859-3) -- BYTE(Y) (112) --> (4_BYTE_UTF-8)
         *                                                                                                               (3_BYTE_ISO-8859-3) -- BYTE(Y) (112) --> (4_BYTE_UTF-16BE)
         * (initial) -- BYTE(X) (40) --> (1_BYTE_UTF-8) -- BYTE(X) (56) --> (2_BYTE_UTF-8) -- BYTE(Y) (72) --> (3_BYTE_UTF-8)
         * (initial) -- BYTE(X) (40) --> (1_BYTE_UTF-16BE) -- BYTE(X) (56) --> (2_BYTE_UTF-16BE) -- BYTE(Y) (72) --> (3_BYTE_UTF-16BE)
         *
         * Situation after adding edges to vertices at position 4
         * (initial) -- BYTE(X) (32) --> (1_BYTE_ISO-8859-2) -- BYTE(X) (40) --> (2_BYTE_ISO-8859-2) -- BYTE(Y) (72) --> (3_BYTE_ISO-8859-3) -- BYTE(Y) (80) --> (4_BYTE_ISO-8859-3)
         *                                                                                                               (3_BYTE_UTF-8) -- BYTE(Y) (88) --> (4_BYTE_UTF-8)
         *                                                                                                               (3_BYTE_UTF-16BE) -- BYTE(Y) (88) --> (4_BYTE_UTF-16BE)
         * (initial) -- BYTE(X) (40) --> (1_BYTE_UTF-8) -- BYTE(X) (56) --> (2_BYTE_UTF-8) -- BYTE(Y) (72) --> (3_BYTE_UTF-8)
         * (initial) -- BYTE(X) (40) --> (1_BYTE_UTF-16BE) -- BYTE(X) (56) --> (2_BYTE_UTF-16BE) -- BYTE(Y) (72) --> (3_BYTE_UTF-16BE)
         *
         * Encoding as ECI(ISO-8859-2),BYTE(XX),ECI(ISO-8859-3),BYTE(YY) has the smallest size of 80 and is hence chosen.
         * The encodation ECI(UTF-8),BYTE(XXYY) is longer with a size of 88.
         */

        // let inputLength = self.stringToEncode.chars().count();
        let inputLength = self.stringToEncode.len();

        // Array that represents vertices. There is a vertex for every character, encoding and mode. The vertex contains
        // a list of all edges that lead to it that have the same encoding and mode.
        // The lists are created lazily

        // The last dimension in the array below encodes the 4 modes KANJI, ALPHANUMERIC, NUMERIC and BYTE via the
        // function getCompactedOrdinal(Mode)
        let mut edges = vec![vec![vec![None; 4]; self.encoders.len()]; inputLength + 1];
        self.addEdges(version, &mut edges, 0, None)?;

        for i in 1..=inputLength {
            for j in 0..self.encoders.len() {
                for k in 0..4 {
                    if edges[i][j][k].is_some() && i < inputLength {
                        let e = edges[i][j][k].clone();
                        self.addEdges(version, &mut edges, i, e)?;
                    }
                }
            }
        }
        let mut minimalJ = None;
        let mut minimalK = None;
        let mut minimalSize = u32::MAX;
        for j in 0..self.encoders.len() {
            for k in 0..4 {
                if let Some(edge) = &edges[inputLength][j][k] {
                    if edge.cachedTotalSize < minimalSize {
                        minimalSize = edge.cachedTotalSize;
                        minimalJ = Some(j);
                        minimalK = Some(k);
                    }
                }
            }
        }

        if let Some((minJ, minK)) = minimalJ.zip(minimalK) {
            Ok(RXingResultList::new(
                version,
                edges[inputLength][minJ][minK]
                    .as_ref()
                    .ok_or(Exceptions::writer)?
                    .clone(),
                self.isGS1,
                &self.ecLevel,
                self.encoders.clone(),
                self.stringToEncode.clone(),
            )
            .ok_or(Exceptions::writer)?)
        } else {
            Err(Exceptions::writerWith(format!(
                r#"Internal error: failed to encode "{}"#,
                self.stringToEncode
                    .iter()
                    .map(String::from)
                    .collect::<String>()
            )))
        }
    }
}

pub struct Edge {
    pub mode: Mode,
    fromPosition: usize,
    charsetEncoderIndex: usize,
    characterLength: u32,
    previous: Option<Rc<Edge>>,
    cachedTotalSize: u32,
    _encoders: ECIEncoderSet,
    _stringToEncode: Vec<String>,
}
impl Edge {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        mode: Mode,
        fromPosition: usize,
        charsetEncoderIndex: usize,
        characterLength: u32,
        previous: Option<Rc<Edge>>,
        version: VersionRef,
        encoders: ECIEncoderSet,
        stringToEncode: Vec<String>,
    ) -> Option<Self> {
        let nci = if mode == Mode::BYTE || previous.is_none() {
            charsetEncoderIndex
        } else {
            previous.as_ref()?.charsetEncoderIndex
        };

        Some(Self {
            mode,
            fromPosition,
            charsetEncoderIndex: nci,
            characterLength,
            previous: previous.clone(),
            _stringToEncode: stringToEncode.clone(),
            cachedTotalSize: {
                let mut size = if previous.is_some() {
                    previous.as_ref()?.cachedTotalSize
                } else {
                    0
                };

                let needECI = mode == Mode::BYTE &&
                          (previous.is_none() && nci != 0) || // at the beginning and charset is not ISO-8859-1
                          (previous.is_some() && nci != previous.as_ref()?.charsetEncoderIndex);

                if previous.is_none() || mode != previous.as_ref()?.mode || needECI {
                    size += 4 + mode.getCharacterCountBits(version) as u32;
                }
                match mode {
                    Mode::NUMERIC => {
                        size += if characterLength == 1 {
                            4
                        } else if characterLength == 2 {
                            7
                        } else {
                            10
                        }
                    }
                    Mode::ALPHANUMERIC => size += if characterLength == 1 { 6 } else { 11 },
                    Mode::BYTE => {
                        let n: String = stringToEncode
                            .iter()
                            .skip(fromPosition)
                            .take(characterLength as usize)
                            .map(String::from)
                            .collect();
                        size += 8 * encoders.encode_string(&n, charsetEncoderIndex)?.len() as u32;
                        if needECI {
                            size += 4 + 8; // the ECI assignment numbers for ISO-8859-x, UTF-8 and UTF-16 are all 8 bit long
                        }
                    }
                    Mode::KANJI => size += 13,
                    _ => {}
                }
                size
            },
            _encoders: encoders,
        })
    }
}

#[derive(Clone)]
pub struct RXingResultList {
    list: Vec<RXingResultNode>,
    version: VersionRef,
}
impl RXingResultList {
    pub fn new(
        version: VersionRef,
        solution: Rc<Edge>,
        isGS1: bool,
        ecLevel: &ErrorCorrectionLevel,
        encoders: ECIEncoderSet,
        stringToEncode: Vec<String>,
    ) -> Option<Self> {
        let mut length = 0;
        let mut current = Some(solution);
        let mut containsECI = false;
        let mut list = Vec::new();

        while let Some(loop_current) = &current {
            length += loop_current.characterLength;
            let previous = current.as_ref()?.previous.clone();

            let needECI = loop_current.mode == Mode::BYTE &&
          (previous.is_none() && loop_current.charsetEncoderIndex != 0) || // at the beginning and charset is not ISO-8859-1
          (previous.is_some() && loop_current.charsetEncoderIndex != previous.as_ref()?.charsetEncoderIndex);

            if needECI {
                containsECI = true;
            }

            if previous.is_none() || previous.as_ref()?.mode != loop_current.mode || needECI {
                list.push(RXingResultNode::new(
                    loop_current.mode,
                    loop_current.fromPosition,
                    loop_current.charsetEncoderIndex,
                    length,
                    encoders.clone(),
                    stringToEncode.clone(),
                    version,
                ));
                length = 0;
            }

            if needECI {
                list.push(RXingResultNode::new(
                    Mode::ECI,
                    loop_current.fromPosition,
                    loop_current.charsetEncoderIndex,
                    0,
                    encoders.clone(),
                    stringToEncode.clone(),
                    version,
                ));
            }
            current = previous;
        }

        // prepend FNC1 if needed. If the bits contain an ECI then the FNC1 must be preceeded by an ECI.
        // If there is no ECI at the beginning then we put an ECI to the default charset (ISO-8859-1)
        if isGS1 {
            if let Some(first) = list.get(0) {
                if first.mode != Mode::ECI && containsECI {
                    // prepend a default character set ECI
                    list.push(RXingResultNode::new(
                        Mode::ECI,
                        0,
                        0,
                        0,
                        encoders.clone(),
                        stringToEncode.clone(),
                        version,
                    ));
                }
            }

            if let Some(first) = list.get(0) {
                // prepend or insert a FNC1_FIRST_POSITION after the ECI (if any)
                if first.mode != Mode::ECI {
                    //&& containsECI {
                    list.insert(
                        if first.mode != Mode::ECI {
                            //first
                            list.len()
                        } else {
                            //second
                            list.len() - 1
                        },
                        RXingResultNode::new(
                            Mode::FNC1_FIRST_POSITION,
                            0,
                            0,
                            0,
                            encoders,
                            stringToEncode,
                            version,
                        ),
                    );
                }
            }
        }

        // set version to smallest version into which the bits fit.
        let mut versionNumber = version.getVersionNumber();
        let (lowerLimit, upperLimit) = match MinimalEncoder::getVersionSize(version) {
            VersionSize::SMALL => (1, 9),
            VersionSize::MEDIUM => (10, 26),
            _ => (27, 40),
        };

        let size = Self::internal_static_get_size(version, &list);
        // increase version if needed
        while versionNumber < upperLimit
            && !qrcode_encoder::willFit(
                size,
                Version::getVersionForNumber(versionNumber).ok()?,
                ecLevel,
            )
        {
            versionNumber += 1;
        }
        // shrink version if possible
        while versionNumber > lowerLimit
            && qrcode_encoder::willFit(
                size,
                Version::getVersionForNumber(versionNumber - 1).ok()?,
                ecLevel,
            )
        {
            versionNumber -= 1;
        }
        let version = Version::getVersionForNumber(versionNumber).ok()?;
        list.reverse();
        Some(Self { list, version })
    }

    /**
     * returns the size in bits
     */
    pub fn getSize(&self) -> u32 {
        self.getSizeLocal(self.version)
    }

    fn getSizeLocal(&self, version: VersionRef) -> u32 {
        let result = self
            .list
            .iter()
            .fold(0, |acc, node| acc + node.getSize(version));
        result
    }

    fn internal_static_get_size(version: VersionRef, list: &Vec<RXingResultNode>) -> u32 {
        let result = list.iter().fold(0, |acc, node| acc + node.getSize(version));
        result
    }

    /**
     * appends the bits
     */
    pub fn getBits(&self, bits: &mut BitArray) -> Result<()> {
        for resultNode in &self.list {
            resultNode.getBits(bits)?;
        }
        Ok(())
    }

    pub fn getVersion(&self) -> VersionRef {
        self.version
    }
}

impl fmt::Display for RXingResultList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        let mut previous = None;
        for current in &self.list {
            // for (RXingResultNode current : list) {
            if previous.is_some() {
                result.push(',');
            }
            result.push_str(&current.to_string());
            previous = Some(current);
        }
        write!(f, "{result}")
    }
}

#[derive(Clone)]
struct RXingResultNode {
    mode: Mode,
    fromPosition: usize,
    charsetEncoderIndex: usize,
    characterLength: u32,
    encoders: ECIEncoderSet,
    version: VersionRef,
    stringToEncode: Vec<String>,
}

impl RXingResultNode {
    pub fn new(
        mode: Mode,
        fromPosition: usize,
        charsetEncoderIndex: usize,
        characterLength: u32,
        encoders: ECIEncoderSet,
        stringToEncode: Vec<String>,
        version: VersionRef,
    ) -> Self {
        Self {
            mode,
            fromPosition,
            charsetEncoderIndex,
            characterLength,
            encoders,
            stringToEncode,
            version,
        }
    }

    /**
     * returns the size in bits
     */
    fn getSize(&self, version: &Version) -> u32 {
        let mut size = 4 + self.mode.getCharacterCountBits(version) as u32;
        match self.mode {
            Mode::NUMERIC => {
                size += (self.characterLength / 3) * 10;
                let rest = self.characterLength % 3;
                size += if rest == 1 {
                    4
                } else if rest == 2 {
                    7
                } else {
                    0
                };
            }
            Mode::ALPHANUMERIC => {
                size += (self.characterLength / 2) * 11;
                size += if (self.characterLength % 2) == 1 {
                    6
                } else {
                    0
                };
            }
            Mode::BYTE => size += 8 * self.getCharacterCountIndicator(),
            Mode::ECI => size += 8,
            Mode::KANJI => size += 13 * self.characterLength,
            _ => {}
        }
        // switch (mode) {
        // case KANJI:
        //   size += 13 * characterLength;
        //   break;
        // case ALPHANUMERIC:
        //   size += (characterLength / 2) * 11;
        //   size += (characterLength % 2) == 1 ? 6 : 0;
        //   break;
        // case NUMERIC:
        //   size += (characterLength / 3) * 10;
        //   int rest = characterLength % 3;
        //   size += rest == 1 ? 4 : rest == 2 ? 7 : 0;
        //   break;
        // case BYTE:
        //   size += 8 * getCharacterCountIndicator();
        //   break;
        // case ECI:
        //   size += 8; // the ECI assignment numbers for ISO-8859-x, UTF-8 and UTF-16 are all 8 bit long
        // }
        size
    }

    /**
     * returns the length in characters according to the specification (differs from getCharacterLength() in BYTE mode
     * for multi byte encoded characters)
     */
    fn getCharacterCountIndicator(&self) -> u32 {
        if self.mode == Mode::BYTE {
            self.encoders
                .encode_string(
                    &(self
                        .stringToEncode
                        .iter()
                        .skip(self.fromPosition)
                        .take(self.characterLength as usize)
                        .map(|s| s.as_str())
                        .collect::<String>()),
                    self.charsetEncoderIndex,
                )
                .unwrap_or_default()
                .len() as u32
        } else {
            self.characterLength
        }
    }

    /**
     * appends the bits
     */
    fn getBits(&self, bits: &mut BitArray) -> Result<()> {
        bits.appendBits(self.mode.getBits() as u32, 4)?;
        if self.characterLength > 0 {
            let length = self.getCharacterCountIndicator();
            bits.appendBits(
                length,
                self.mode.getCharacterCountBits(self.version) as usize,
            )?;
        }
        if self.mode == Mode::ECI {
            bits.appendBits(self.encoders.getECIValue(self.charsetEncoderIndex), 8)?;
        } else if self.characterLength > 0 {
            // append data
            qrcode_encoder::appendBytes(
                &(self
                    .stringToEncode
                    .iter()
                    .skip(self.fromPosition)
                    .take(self.characterLength as usize)
                    .map(|s| s.as_str())
                    .collect::<String>()),
                self.mode,
                bits,
                self.encoders
                    .getCharset(self.charsetEncoderIndex)
                    .ok_or(Exceptions::writer)?,
            )?;
        }
        Ok(())
    }

    fn makePrintable(s: &str) -> String {
        let mut result = String::new();
        for ch in s.chars() {
            if (ch as u32) < 32 || (ch as u32) > 126 {
                result.push('.');
            } else {
                result.push(ch);
            }
        }
        result
    }
}

impl fmt::Display for RXingResultNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        result.push_str(&format!("{:?}", self.mode));
        result.push('(');
        if self.mode == Mode::ECI {
            result.push_str(
                self.encoders
                    .getCharset(self.charsetEncoderIndex)
                    .ok_or(fmt::Error)?
                    .name(),
            );
        } else {
            let sub_string: String = self
                .stringToEncode
                .iter()
                .skip(self.fromPosition)
                .take(self.characterLength as usize)
                .map(String::from)
                .collect();
            // result.push_str(&Self::makePrintable(
            //     &self.stringToEncode[self.fromPosition as usize
            //         ..(self.fromPosition + self.characterLength as usize)],
            // ));
            result.push_str(&Self::makePrintable(&sub_string));
        }
        result.push(')');

        write!(f, "{result}")
    }
}
