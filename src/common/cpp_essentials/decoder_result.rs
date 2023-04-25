use std::{any::Any, rc::Rc};

use crate::{common::ECIStringBuilder, Exceptions, RXingResult};

use super::StructuredAppendInfo;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DecoderResult<T>
where
    T: Copy + Clone + Default + Eq + PartialEq,
{
    content: ECIStringBuilder,
    ecLevel: String,
    lineCount: u32,     // = 0;
    versionNumber: u32, // = 0;
    structuredAppend: StructuredAppendInfo,
    isMirrored: bool, // = false;
    readerInit: bool, // = false;
    //Error _error;
    //std::shared_ptr<CustomData> _extra;
    error: Option<Exceptions>,
    extra: Rc<T>,
}

impl<T> Default for DecoderResult<T>
where
    T: Copy + Clone + Default + Eq + PartialEq,
{
    fn default() -> Self {
        Self {
            content: Default::default(),
            ecLevel: Default::default(),
            lineCount: 0,
            versionNumber: 0,
            structuredAppend: Default::default(),
            isMirrored: false,
            readerInit: false,
            error: None,
            extra: Default::default(),
        }
    }
}

impl<T> DecoderResult<T>
where
    T: Copy + Clone + Default + Eq + PartialEq,
{
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_eci_string_builder(src: ECIStringBuilder) -> Self {
        let mut new_self = Self::default();
        new_self.content = src;

        new_self
    }

    pub fn isValid(&self) -> bool {
        self.content.symbology.code != 0 && self.error.is_none()
        //return includeErrors || (_content.symbology.code != 0 && !_error);
    }

    pub fn content(&self) -> &ECIStringBuilder {
        &self.content
    }
}

impl<T> DecoderResult<T>
where
    T: Copy + Clone + Default + Eq + PartialEq,
{
    pub fn ecLevel(&self) -> &str {
        &self.ecLevel
    }
    pub fn setEcLevel(&mut self, ecLevel: String) {
        self.ecLevel = ecLevel
    }
    pub fn withEcLevel(mut self, ecLevel: String) -> DecoderResult<T> {
        self.setEcLevel(ecLevel);
        self
    }

    pub fn lineCount(&self) -> u32 {
        self.lineCount
    }
    pub fn setLineCount(&mut self, lc: u32) {
        self.lineCount = lc
    }
    pub fn withLineCount(mut self, lc: u32) -> DecoderResult<T> {
        self.setLineCount(lc);
        self
    }

    pub fn versionNumber(&self) -> u32 {
        self.versionNumber
    }
    pub fn setVersionNumber(&mut self, vn: u32) {
        self.versionNumber = vn
    }
    pub fn withVersionNumber(mut self, vn: u32) -> DecoderResult<T> {
        self.setVersionNumber(vn);
        self
    }

    pub fn structuredAppend(&self) -> &StructuredAppendInfo {
        &self.structuredAppend
    }
    pub fn setStructuredAppend(&mut self, sai: StructuredAppendInfo) {
        self.structuredAppend = sai
    }
    pub fn withStructuredAppend(mut self, sai: StructuredAppendInfo) -> DecoderResult<T> {
        self.setStructuredAppend(sai);
        self
    }

    pub fn isMirrored(&self) -> bool {
        self.isMirrored
    }
    pub fn setIsMirrored(&mut self, is_mirrored: bool) {
        self.isMirrored = is_mirrored
    }
    pub fn withIsMirrored(mut self, is_mirrored: bool) -> DecoderResult<T> {
        self.setIsMirrored(is_mirrored);
        self
    }

    pub fn readerInit(&self) -> bool {
        self.readerInit
    }
    pub fn setReaderInit(&mut self, reader_init: bool) {
        self.readerInit = reader_init
    }
    pub fn withReaderInit(mut self, reader_init: bool) -> DecoderResult<T> {
        self.setReaderInit(reader_init);
        self
    }

    pub fn extra(&self) -> Rc<T> {
        self.extra.clone()
    }
    pub fn setExtra(&mut self, extra: Rc<T>) {
        self.extra = extra
    }
    pub fn withExtra(mut self, extra: Rc<T>) -> DecoderResult<T> {
        self.setExtra(extra);
        self
    }

    pub fn error(&self) -> &Option<Exceptions> {
        &self.error
    }
    pub fn setError(&mut self, error: Option<Exceptions>) {
        self.error = error
    }
    pub fn withError(mut self, error: Option<Exceptions>) -> DecoderResult<T> {
        self.setError(error);
        self
    }

    // pub fn build(self) -> DecoderResult<T> {

    // }
}

impl<T> DecoderResult<T>
where
    T: Copy + Clone + Default + Eq + PartialEq,
{
    pub fn text(&self) -> String {
        self.content.to_string()
    }

    pub fn symbologyIdentifier(&self) -> String {
        let s = self.content.symbology;
        if s.code > 0 {
            format!(
                "]{}{}",
                char::from(s.code),
                char::from(
                    s.modifier
                        + if self.content.has_eci {
                            s.eciModifierOffset
                        } else {
                            0
                        }
                )
            )
        } else {
            String::default()
        }
    }
}
