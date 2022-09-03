mod ParsedResult;
mod ResultParser;
mod TelParsedResult;
mod TextParsedResult;
mod ParsedResultType;
mod TelResultParser;

pub use ParsedResultType::*;
pub use ResultParser::*;
pub use TelParsedResult::*;
pub use TextParsedResult::*;
pub use ParsedResult::*;
pub use TelResultParser::*;

#[cfg(test)]
mod TelParsedResultTestCase;

pub enum ParsedClientResult {
    TextResult(TextParsedRXingResult),
    TelResult(TelParsedRXingResult)
}