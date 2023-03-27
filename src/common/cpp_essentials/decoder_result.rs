use std::{any::Any, rc::Rc};

use crate::common::ECIStringBuilder;

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
        todo!()
    }

    pub fn isValid(&self) -> bool {
        //return includeErrors || (_content.symbology.code != 0 && !_error);
        todo!()
    }

    pub fn content(&self) -> &ECIStringBuilder {
        &self.content
    }
}

// DecoderResult(const DecoderResult &) = delete;
// DecoderResult& operator=(const DecoderResult &) = delete;
// }

// public:
// DecoderResult() = default;
// DecoderResult(Error error) : _error(std::move(error)) {}
// DecoderResult(Content&& bytes) : _content(std::move(bytes)) {}

// DecoderResult(DecoderResult&&) noexcept = default;
// DecoderResult& operator=(DecoderResult&&) noexcept = default;

// bool isValid(bool includeErrors = false) const
// {
// 	return includeErrors || (_content.symbology.code != 0 && !_error);
// }

// const Content& content() const & { return _content; }
// Content&& content() && { return std::move(_content); }

// to keep the unit tests happy for now:
// std::wstring text() const { return _content.utfW(); }
// std::string symbologyIdentifier() const { return _content.symbology.toString(false); }

// Simple macro to set up getter/setter methods that save lots of boilerplate.
// It sets up a standard 'const & () const', 2 setters for setting lvalues via
// copy and 2 for setting rvalues via move. They are provided each to work
// either on lvalues (normal 'void (...)') or on rvalues (returning '*this' as
// rvalue). The latter can be used to optionally initialize a temporary in a
// return statement, e.g.
//    return DecoderResult(bytes, text).setEcLevel(level);
// #define ZX_PROPERTY(TYPE, GETTER, SETTER) \
// 	const TYPE& GETTER() const & { return _##GETTER; } \
// 	TYPE&& GETTER() && { return std::move(_##GETTER); } \
// 	void SETTER(const TYPE& v) & { _##GETTER = v; } \
// 	void SETTER(TYPE&& v) & { _##GETTER = std::move(v); } \
// 	DecoderResult&& SETTER(const TYPE& v) && { _##GETTER = v; return std::move(*this); } \
// 	DecoderResult&& SETTER(TYPE&& v) && { _##GETTER = std::move(v); return std::move(*this); }

// 	ZX_PROPERTY(std::string, ecLevel, setEcLevel)
// 	ZX_PROPERTY(int, lineCount, setLineCount)
// 	ZX_PROPERTY(int, versionNumber, setVersionNumber)
// 	ZX_PROPERTY(StructuredAppendInfo, structuredAppend, setStructuredAppend)

// 	ZX_PROPERTY(Error, error, setError)

// 	ZX_PROPERTY(bool, isMirrored, setIsMirrored)
// 	ZX_PROPERTY(bool, readerInit, setReaderInit)
// 	ZX_PROPERTY(std::shared_ptr<CustomData>, extra, setExtra)

// #undef ZX_PROPERTY
// };

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

    // pub fn build(self) -> DecoderResult<T> {

    // }
}
