mod ParsedResult;
mod ResultParser;
mod TelParsedResult;
mod TextParsedResult;
mod ParsedResultType;
mod TelResultParser;
mod ISBNParsedResult;
mod ISBNResultParser;
mod WifiParsedResult;
mod WifiResultParser;
mod GeoResultParser;
mod GeoParsedResult;
mod SMSParsedResult;
mod SMSMMSResultParser;
mod ProductParsedResult;
mod ProductResultParser;
mod URIParsedResult;
mod URIResultParser;
mod URLTOResultParser;
mod AbstractDoCoMoResultParser;
mod BookmarkDoCoMoResultParser;
mod SMSTOMMSTOResultParser;
mod EmailAddressParsedResult;
mod EmailAddressResultParser;
mod EmailDoCoMoResultParser;
mod SMTPResultParser;
mod VINParsedResult;
mod VINResultParser;
mod AddressBookParsedResult;
mod AddressBookDoCoMoResultParser;
mod AddressBookAUResultParser;
mod VCardResultParser;
mod BizcardResultParser;
mod CalendarParsedResult;
mod VEventResultParser;

use std::fmt;

pub use ParsedResultType::*;
pub use ResultParser::*;
pub use TelParsedResult::*;
pub use TextParsedResult::*;
pub use ParsedResult::*;
// pub use TelResultParser::*;
pub use ISBNParsedResult::*;
// pub use ISBNResultParser::*;
pub use WifiParsedResult::*;
// pub use WifiResultParser::*;
pub use GeoParsedResult::*;
// pub use GeoResultParser::*;
pub use SMSParsedResult::*;
pub use ProductParsedResult::*;
pub use URIParsedResult::*;
pub use EmailAddressParsedResult::*;
pub use VINParsedResult::*;
pub use AddressBookParsedResult::*;
pub use CalendarParsedResult::*;
pub use CalendarParsedResult::*;

#[cfg(test)]
mod TelParsedResultTestCase;
#[cfg(test)]
mod ISBNParsedResultTestCase;
#[cfg(test)]
mod WifiParsedResultTestCase;
#[cfg(test)]
mod GeoParsedResultTestCase;
#[cfg(test)]
mod SMSMMSParsedResultTestCase;
#[cfg(test)]
mod ProductParsedResultTestCase;
#[cfg(test)]
mod URIParsedResultTestCase;
#[cfg(test)]
mod EmailAddressParsedResultTestCase;
#[cfg(test)]
mod VINParsedResultTestCase;
#[cfg(test)]
mod AddressBookParsedResultTestCase;
#[cfg(test)]
mod CalendarParsedResultTestCase;

pub enum ParsedClientResult {
    TextResult(TextParsedRXingResult),
    TelResult(TelParsedRXingResult),
    ISBNResult(ISBNParsedRXingResult),
    WiFiResult(WifiParsedRXingResult),
    GeoResult(GeoParsedRXingResult),
    SMSResult(SMSParsedRXingResult),
    ProductResult(ProductParsedRXingResult),
    URIResult(URIParsedRXingResult),
    EmailResult(EmailAddressParsedRXingResult),
    VINResult(VINParsedRXingResult),
    AddressBookResult(AddressBookParsedRXingResult),
    CalendarEventResult(CalendarParsedRXingResult),
}

impl ParsedRXingResult for ParsedClientResult {
    fn getType(&self) -> ParsedRXingResultType {
        match self {
            ParsedClientResult::TextResult(a) => a.getType(),
            ParsedClientResult::TelResult(a) => a.getType(),
            ParsedClientResult::ISBNResult(a) => a.getType(),
            ParsedClientResult::WiFiResult(a) => a.getType(),
            ParsedClientResult::GeoResult(a) => a.getType(),
            ParsedClientResult::SMSResult(a) => a.getType(),
            ParsedClientResult::ProductResult(a) => a.getType(),
            ParsedClientResult::URIResult(a) => a.getType(),
            ParsedClientResult::EmailResult(a) => a.getType(),
            ParsedClientResult::VINResult(a) => a.getType(),
            ParsedClientResult::AddressBookResult(a) => a.getType(),
            ParsedClientResult::CalendarEventResult(a) => a.getType(),
        }
    }

    fn getDisplayRXingResult(&self) -> String {
        match self {
            ParsedClientResult::TextResult(a) => a.getDisplayRXingResult(),
            ParsedClientResult::TelResult(a) => a.getDisplayRXingResult(),
            ParsedClientResult::ISBNResult(a) => a.getDisplayRXingResult(),
            ParsedClientResult::WiFiResult(a) => a.getDisplayRXingResult(),
            ParsedClientResult::GeoResult(a) => a.getDisplayRXingResult(),
            ParsedClientResult::SMSResult(a) => a.getDisplayRXingResult(),
            ParsedClientResult::ProductResult(a) => a.getDisplayRXingResult(),
            ParsedClientResult::URIResult(a) => a.getDisplayRXingResult(),
            ParsedClientResult::EmailResult(a) => a.getDisplayRXingResult(),
            ParsedClientResult::VINResult(a) => a.getDisplayRXingResult(),
            ParsedClientResult::AddressBookResult(a) => a.getDisplayRXingResult(),
            ParsedClientResult::CalendarEventResult(a) => a.getDisplayRXingResult(),
        }
    }
}

impl fmt::Display for ParsedClientResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}", self.getDisplayRXingResult())
    }
}