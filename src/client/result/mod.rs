mod AbstractDoCoMoResultParser;
mod AddressBookAUResultParser;
mod AddressBookDoCoMoResultParser;
mod AddressBookParsedResult;
mod BizcardResultParser;
mod BookmarkDoCoMoResultParser;
mod CalendarParsedResult;
mod EmailAddressParsedResult;
mod EmailAddressResultParser;
mod EmailDoCoMoResultParser;
mod ExpandedProductParsedResult;
mod ExpandedProductResultParser;
mod GeoParsedResult;
mod GeoResultParser;
mod ISBNParsedResult;
mod ISBNResultParser;
mod ParsedResult;
mod ParsedResultType;
mod ProductParsedResult;
mod ProductResultParser;
mod ResultParser;
mod SMSMMSResultParser;
mod SMSParsedResult;
mod SMSTOMMSTOResultParser;
mod SMTPResultParser;
mod TelParsedResult;
mod TelResultParser;
mod TextParsedResult;
mod URIParsedResult;
mod URIResultParser;
mod URLTOResultParser;
mod VCardResultParser;
mod VEventResultParser;
mod VINParsedResult;
mod VINResultParser;
mod WifiParsedResult;
mod WifiResultParser;

use std::fmt;

pub use ParsedResult::*;
pub use ParsedResultType::*;
pub use ResultParser::*;
pub use TelParsedResult::*;
pub use TextParsedResult::*;
// pub use TelResultParser::*;
pub use ISBNParsedResult::*;
// pub use ISBNResultParser::*;
pub use WifiParsedResult::*;
// pub use WifiResultParser::*;
pub use GeoParsedResult::*;
// pub use GeoResultParser::*;
pub use AddressBookParsedResult::*;
pub use CalendarParsedResult::*;
pub use CalendarParsedResult::*;
pub use EmailAddressParsedResult::*;
pub use ExpandedProductParsedResult::*;
pub use ProductParsedResult::*;
pub use SMSParsedResult::*;
pub use URIParsedResult::*;
pub use VINParsedResult::*;

#[cfg(test)]
mod AddressBookParsedResultTestCase;
#[cfg(test)]
mod CalendarParsedResultTestCase;
#[cfg(test)]
mod EmailAddressParsedResultTestCase;
#[cfg(test)]
mod ExpandedProductParsedResultTestCase;
#[cfg(test)]
mod GeoParsedResultTestCase;
#[cfg(test)]
mod ISBNParsedResultTestCase;
#[cfg(test)]
mod ParsedReaderResultTestCase;
#[cfg(test)]
mod ProductParsedResultTestCase;
#[cfg(test)]
mod SMSMMSParsedResultTestCase;
#[cfg(test)]
mod TelParsedResultTestCase;
#[cfg(test)]
mod URIParsedResultTestCase;
#[cfg(test)]
mod VINParsedResultTestCase;
#[cfg(test)]
mod WifiParsedResultTestCase;

#[derive(PartialEq, Eq, Debug)]
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
    ExpandedProductResult(ExpandedProductParsedRXingResult),
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
            ParsedClientResult::ExpandedProductResult(a) => a.getType(),
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
            ParsedClientResult::ExpandedProductResult(a) => a.getDisplayRXingResult(),
        }
    }
}

impl fmt::Display for ParsedClientResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.getDisplayRXingResult())
    }
}
