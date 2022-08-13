use crate::XRingResult;

// ParsedResultType.java
/**
 * Represents the type of data encoded by a barcode -- from plain text, to a
 * URI, to an e-mail address, etc.
 *
 * @author Sean Owen
 */
pub enum ParsedResultType {

    ADDRESSBOOK, EMAIL_ADDRESS, PRODUCT, URI, TEXT, GEO, TEL, SMS, CALENDAR, WIFI, ISBN, VIN
}

// ParsedResult.java
/**
 * <p>Abstract class representing the result of decoding a barcode, as more than
 * a String -- as some type of structured data. This might be a subclass which represents
 * a URL, or an e-mail address. {@link ResultParser#parseResult(com.google.zxing.Result)} will turn a raw
 * decoded string into the most appropriate type of structured representation.</p>
 *
 * <p>Thanks to Jeff Griffin for proposing rewrite of these classes that relies less
 * on exception-based mechanisms during parsing.</p>
 *
 * @author Sean Owen
 */
trait ParsedResult {

   pub fn new( type: &ParsedResultType) -> ParsedResult {
       let .type = type;
   }

   pub fn  get_type(&self) -> ParsedResultType  {
       return self.type;
   }

   pub fn  get_display_result(&self) -> String ;

   pub fn  to_string(&self) -> String  {
       return self.get_display_result();
   }

   pub fn  maybe_append( value: &String,  result: &StringBuilder)   {
       if value != null && !value.is_empty() {
           // Don't add a newline before the first value
           if result.length() > 0 {
               result.append('\n');
           }
           result.append(&value);
       }
   }

   pub fn  maybe_append( values: &Vec<String>,  result: &StringBuilder)   {
       if values != null {
           for  let value: String in values {
               ::maybe_append(&value, &result);
           }
       }
   }
}

// ResultParser.java
/**
 * <p>Abstract class representing the result of decoding a barcode, as more than
 * a String -- as some type of structured data. This might be a subclass which represents
 * a URL, or an e-mail address. {@link #parseResult(Result)} will turn a raw
 * decoded string into the most appropriate type of structured representation.</p>
 *
 * <p>Thanks to Jeff Griffin for proposing rewrite of these classes that relies less
 * on exception-based mechanisms during parsing.</p>
 *
 * @author Sean Owen
 */

const PARSERS: vec![Vec<ResultParser>; 20] = vec![BookmarkDoCoMoResultParser::new(), AddressBookDoCoMoResultParser::new(), EmailDoCoMoResultParser::new(), AddressBookAUResultParser::new(), VCardResultParser::new(), BizcardResultParser::new(), VEventResultParser::new(), EmailAddressResultParser::new(), SMTPResultParser::new(), TelResultParser::new(), SMSMMSResultParser::new(), SMSTOMMSTOResultParser::new(), GeoResultParser::new(), WifiResultParser::new(), URLTOResultParser::new(), URIResultParser::new(), ISBNResultParser::new(), ProductResultParser::new(), ExpandedProductResultParser::new(), VINResultParser::new(), ]
;

 const DIGITS: Pattern = Pattern::compile("\\d+");

 const AMPERSAND: Pattern = Pattern::compile("&");

 const EQUALS: Pattern = Pattern::compile("=");

 const BYTE_ORDER_MARK: &'static str = "﻿";

 const EMPTY_STR_ARRAY: [Option<String>; 0] = [None; 0];

trait ResultParser {

    /**
   * Attempts to parse the raw {@link Result}'s contents as a particular type
   * of information (email, URL, etc.) and return a {@link ParsedResult} encapsulating
   * the result of parsing.
   *
   * @param theResult the raw {@link Result} to parse
   * @return {@link ParsedResult} encapsulating the parsing result
   */
    pub fn  parse(&self,  the_result: &Result) -> ParsedResult ;

    pub fn  get_massaged_text( result: &Result) -> String  {
         let mut text: String = result.get_text();
        if text.starts_with(&BYTE_ORDER_MARK) {
            text = text.substring(1);
        }
        return text;
    }

    pub fn  parse_result( the_result: &Result) -> ParsedResult  {
        for  let parser: ResultParser in PARSERS {
             let result: ParsedResult = parser.parse(the_result);
            if result != null {
                return result;
            }
        }
        return TextParsedResult::new(&the_result.get_text(), null);
    }

    pub fn  maybe_append( value: &String,  result: &StringBuilder)   {
        if value != null {
            result.append('\n');
            result.append(&value);
        }
    }

    pub fn  maybe_append( value: &Vec<String>,  result: &StringBuilder)   {
        if value != null {
            for  let s: String in value {
                result.append('\n');
                result.append(&s);
            }
        }
    }

    pub fn  maybe_wrap( value: &String) -> Vec<String>  {
        return  if value == null { null } else {  : vec![String; 1] = vec![value, ]
         };
    }

    pub fn  unescape_backslash( escaped: &String) -> String  {
         let backslash: i32 = escaped.index_of('\\');
        if backslash < 0 {
            return escaped;
        }
         let max: i32 = escaped.length();
         let unescaped: StringBuilder = StringBuilder::new(max - 1);
        unescaped.append(&escaped.to_char_array(), 0, backslash);
         let next_is_escaped: bool = false;
         {
             let mut i: i32 = backslash;
            while i < max {
                {
                     let c: char = escaped.char_at(i);
                    if next_is_escaped || c != '\\' {
                        unescaped.append(c);
                        next_is_escaped = false;
                    } else {
                        next_is_escaped = true;
                    }
                }
                i += 1;
             }
         }

        return unescaped.to_string();
    }

    pub fn  parse_hex_digit( c: char) -> i32  {
        if c >= '0' && c <= '9' {
            return c - '0';
        }
        if c >= 'a' && c <= 'f' {
            return 10 + (c - 'a');
        }
        if c >= 'A' && c <= 'F' {
            return 10 + (c - 'A');
        }
        return -1;
    }

    pub fn  is_string_of_digits( value: &CharSequence,  length: i32) -> bool  {
        return value != null && length > 0 && length == value.length() && DIGITS::matcher(&value)::matches();
    }

    pub fn  is_substring_of_digits( value: &CharSequence,  offset: i32,  length: i32) -> bool  {
        if value == null || length <= 0 {
            return false;
        }
         let max: i32 = offset + length;
        return value.length() >= max && DIGITS::matcher(&value.sub_sequence(offset, max))::matches();
    }

    fn  parse_name_value_pairs( uri: &String) -> Map<String, String>  {
         let param_start: i32 = uri.index_of('?');
        if param_start < 0 {
            return null;
        }
         let result: Map<String, String> = HashMap<>::new(3);
        for  let key_value: String in AMPERSAND::split(&uri.substring(param_start + 1)) {
            ::append_key_value(&key_value, &result);
        }
        return result;
    }

    fn  append_key_value( key_value: &CharSequence,  result: &Map<String, String>)   {
         let key_value_tokens: Vec<String> = EQUALS::split(&key_value, 2);
        if key_value_tokens.len() == 2 {
             let key: String = key_value_tokens[0];
             let mut value: String = key_value_tokens[1];
            let tryResult1 = 0;
            'try1: loop {
            {
                value = ::url_decode(&value);
                result.put(&key, &value);
            }
            break 'try1
            }
            match tryResult1 {
                 catch ( iae: &IllegalArgumentException) {
                }  0 => break
            }

        }
    }

    fn  url_decode( encoded: &String) -> String  {
        let tryResult1 = 0;
        'try1: loop {
        {
            return URLDecoder::decode(&encoded, "UTF-8");
        }
        break 'try1
        }
        match tryResult1 {
             catch ( uee: &UnsupportedEncodingException) {
                throw IllegalStateException::new(&uee);
            }  0 => break
        }

    }

    fn  match_prefixed_field( prefix: &String,  raw_text: &String,  end_char: char,  trim: bool) -> Vec<String>  {
         let mut matches: List<String> = null;
         let mut i: i32 = 0;
         let max: i32 = raw_text.length();
        while i < max {
            i = raw_text.index_of(&prefix, i);
            if i < 0 {
                break;
            }
            // Skip past this prefix we found to start
            i += prefix.length();
            // Found the start of a match here
             let start: i32 = i;
             let mut more: bool = true;
            while more {
                i = raw_text.index_of(end_char, i);
                if i < 0 {
                    // No terminating end character? uh, done. Set i such that loop terminates and break
                    i = raw_text.length();
                    more = false;
                } else if ::count_preceding_backslashes(&raw_text, i) % 2 != 0 {
                    // semicolon was escaped (odd count of preceding backslashes) so continue
                    i += 1;
                } else {
                    // found a match
                    if matches == null {
                        // lazy init
                        matches = ArrayList<>::new(3);
                    }
                     let mut element: String = ::unescape_backslash(&raw_text.substring(start, i));
                    if trim {
                        element = element.trim();
                    }
                    if !element.is_empty() {
                        matches.add(&element);
                    }
                    i += 1;
                    more = false;
                }
            }
        }
        if matches == null || matches.is_empty() {
            return null;
        }
        return matches.to_array(&EMPTY_STR_ARRAY);
    }

    fn  count_preceding_backslashes( s: &CharSequence,  pos: i32) -> i32  {
         let mut count: i32 = 0;
         {
             let mut i: i32 = pos - 1;
            while i >= 0 {
                {
                    if s.char_at(i) == '\\' {
                        count += 1;
                    } else {
                        break;
                    }
                }
                i -= 1;
             }
         }

        return count;
    }

    fn  match_single_prefixed_field( prefix: &String,  raw_text: &String,  end_char: char,  trim: bool) -> String  {
         let matches: Vec<String> = ::match_prefixed_field(&prefix, &raw_text, end_char, trim);
        return  if matches == null { null } else { matches[0] };
    }
}

// AbstractDoCoMoResultParser.java
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

trait AbstractDoCoMoResultParser : ResultParser {

    fn  match_do_co_mo_prefixed_field( prefix: &String,  raw_text: &String) -> Vec<String>  {
        return match_prefixed_field(&prefix, &raw_text, ';', true);
    }

    fn  match_single_do_co_mo_prefixed_field( prefix: &String,  raw_text: &String,  trim: bool) -> String  {
        return match_single_prefixed_field(&prefix, &raw_text, ';', trim);
    }
}

// AddressBookAUResultParser.java
/**
 * Implements KDDI AU's address book format. See
 * <a href="http://www.au.kddi.com/ezfactory/tec/two_dimensions/index.html">
 * http://www.au.kddi.com/ezfactory/tec/two_dimensions/index.html</a>.
 * (Thanks to Yuzo for translating!)
 *
 * @author Sean Owen
 */
pub struct AddressBookAUResultParser {
    super: ResultParser;
}

impl ResultParser for AddressBookAUResultParser{}

impl AddressBookAUResultParser {

    pub fn  parse(&self,  result: &Result) -> AddressBookParsedResult  {
         let raw_text: String = get_massaged_text(result);
        // MEMORY is mandatory; seems like a decent indicator, as does end-of-record separator CR/LF
        if !raw_text.contains("MEMORY") || !raw_text.contains("\r\n") {
            return null;
        }
        // NAME1 and NAME2 have specific uses, namely written name and pronunciation, respectively.
        // Therefore we treat them specially instead of as an array of names.
         let name: String = match_single_prefixed_field("NAME1:", &raw_text, '\r', true);
         let pronunciation: String = match_single_prefixed_field("NAME2:", &raw_text, '\r', true);
         let phone_numbers: Vec<String> = ::match_multiple_value_prefix("TEL", &raw_text);
         let emails: Vec<String> = ::match_multiple_value_prefix("MAIL", &raw_text);
         let note: String = match_single_prefixed_field("MEMORY:", &raw_text, '\r', false);
         let address: String = match_single_prefixed_field("ADD:", &raw_text, '\r', true);
         let addresses: Vec<String> =  if address == null { null } else {  : vec![String; 1] = vec![address, ]
         };
        return AddressBookParsedResult::new(&maybe_wrap(&name), null, &pronunciation, &phone_numbers, null, &emails, null, null, &note, &addresses, null, null, null, null, null, null);
    }

    fn  match_multiple_value_prefix( prefix: &String,  raw_text: &String) -> Vec<String>  {
         let mut values: List<String> = null;
        // For now, always 3, and always trim
         {
             let mut i: i32 = 1;
            while i <= 3 {
                {
                     let value: String = match_single_prefixed_field(format!("{}{}:", prefix, i), &raw_text, '\r', true);
                    if value == null {
                        break;
                    }
                    if values == null {
                        // lazy init
                        values = ArrayList<>::new(3);
                    }
                    values.add(&value);
                }
                i += 1;
             }
         }

        if values == null {
            return null;
        }
        return values.to_array(EMPTY_STR_ARRAY);
    }
}

// AddressBookDoCoMoResultParser.java
/**
 * Implements the "MECARD" address book entry format.
 *
 * Supported keys: N, SOUND, TEL, EMAIL, NOTE, ADR, BDAY, URL, plus ORG
 * Unsupported keys: TEL-AV, NICKNAME
 *
 * Except for TEL, multiple values for keys are also not supported;
 * the first one found takes precedence.
 *
 * Our understanding of the MECARD format is based on this document:
 *
 * http://www.mobicode.org.tw/files/OMIA%20Mobile%20Bar%20Code%20Standard%20v3.2.1.doc
 *
 * @author Sean Owen
 */
pub struct AddressBookDoCoMoResultParser {
    super: AbstractDoCoMoResultParser;
}

impl AbstractDoCoMoResultParser for AddressBookDoCoMoResultParser{}

impl AddressBookDoCoMoResultParser {

    pub fn  parse(&self,  result: &Result) -> AddressBookParsedResult  {
         let raw_text: String = get_massaged_text(result);
        if !raw_text.starts_with("MECARD:") {
            return null;
        }
         let raw_name: Vec<String> = match_do_co_mo_prefixed_field("N:", &raw_text);
        if raw_name == null {
            return null;
        }
         let name: String = ::parse_name(raw_name[0]);
         let pronunciation: String = match_single_do_co_mo_prefixed_field("SOUND:", &raw_text, true);
         let phone_numbers: Vec<String> = match_do_co_mo_prefixed_field("TEL:", &raw_text);
         let emails: Vec<String> = match_do_co_mo_prefixed_field("EMAIL:", &raw_text);
         let note: String = match_single_do_co_mo_prefixed_field("NOTE:", &raw_text, false);
         let addresses: Vec<String> = match_do_co_mo_prefixed_field("ADR:", &raw_text);
         let mut birthday: String = match_single_do_co_mo_prefixed_field("BDAY:", &raw_text, true);
        if !is_string_of_digits(&birthday, 8) {
            // No reason to throw out the whole card because the birthday is formatted wrong.
            birthday = null;
        }
         let urls: Vec<String> = match_do_co_mo_prefixed_field("URL:", &raw_text);
        // Although ORG may not be strictly legal in MECARD, it does exist in VCARD and we might as well
        // honor it when found in the wild.
         let org: String = match_single_do_co_mo_prefixed_field("ORG:", &raw_text, true);
        return AddressBookParsedResult::new(&maybe_wrap(&name), null, &pronunciation, &phone_numbers, null, &emails, null, null, &note, &addresses, null, &org, &birthday, null, &urls, null);
    }

    fn  parse_name( name: &String) -> String  {
         let comma: i32 = name.index_of(',');
        if comma >= 0 {
            // Format may be last,first; switch it around
            return name.substring(comma + 1) + ' ' + name.substring(0, comma);
        }
        return name;
    }
}

// AddressBookParsedResult.java
/**
 * Represents a parsed result that encodes contact information, like that in an address book
 * entry.
 *
 * @author Sean Owen
 */
pub struct AddressBookParsedResult {
    super: ParsedResult;

     let names: Vec<String>;

     let nicknames: Vec<String>;

     let pronunciation: String;

     let phone_numbers: Vec<String>;

     let phone_types: Vec<String>;

     let emails: Vec<String>;

     let email_types: Vec<String>;

     let instant_messenger: String;

     let note: String;

     let addresses: Vec<String>;

     let address_types: Vec<String>;

     let org: String;

     let birthday: String;

     let title: String;

     let urls: Vec<String>;

     let geo: Vec<String>;
}

impl ParsedResult for AddressBookParsedResult{}

impl AddressBookParsedResult {

    pub fn new( names: &Vec<String>,  phone_numbers: &Vec<String>,  phone_types: &Vec<String>,  emails: &Vec<String>,  email_types: &Vec<String>,  addresses: &Vec<String>,  address_types: &Vec<String>) -> AddressBookParsedResult {
        this(&names, null, null, &phone_numbers, &phone_types, &emails, &email_types, null, null, &addresses, &address_types, null, null, null, null, null);
    }

    pub fn new( names: &Vec<String>,  nicknames: &Vec<String>,  pronunciation: &String,  phone_numbers: &Vec<String>,  phone_types: &Vec<String>,  emails: &Vec<String>,  email_types: &Vec<String>,  instant_messenger: &String,  note: &String,  addresses: &Vec<String>,  address_types: &Vec<String>,  org: &String,  birthday: &String,  title: &String,  urls: &Vec<String>,  geo: &Vec<String>) -> AddressBookParsedResult {
        super(ParsedResultType::ADDRESSBOOK);
        if phone_numbers != null && phone_types != null && phone_numbers.len() != phone_types.len() {
            throw IllegalArgumentException::new("Phone numbers and types lengths differ");
        }
        if emails != null && email_types != null && emails.len() != email_types.len() {
            throw IllegalArgumentException::new("Emails and types lengths differ");
        }
        if addresses != null && address_types != null && addresses.len() != address_types.len() {
            throw IllegalArgumentException::new("Addresses and types lengths differ");
        }
        let .names = names;
        let .nicknames = nicknames;
        let .pronunciation = pronunciation;
        let .phoneNumbers = phone_numbers;
        let .phoneTypes = phone_types;
        let .emails = emails;
        let .emailTypes = email_types;
        let .instantMessenger = instant_messenger;
        let .note = note;
        let .addresses = addresses;
        let .addressTypes = address_types;
        let .org = org;
        let .birthday = birthday;
        let .title = title;
        let .urls = urls;
        let .geo = geo;
    }

    pub fn  get_names(&self) -> Vec<String>  {
        return self.names;
    }

    pub fn  get_nicknames(&self) -> Vec<String>  {
        return self.nicknames;
    }

    /**
   * In Japanese, the name is written in kanji, which can have multiple readings. Therefore a hint
   * is often provided, called furigana, which spells the name phonetically.
   *
   * @return The pronunciation of the getNames() field, often in hiragana or katakana.
   */
    pub fn  get_pronunciation(&self) -> String  {
        return self.pronunciation;
    }

    pub fn  get_phone_numbers(&self) -> Vec<String>  {
        return self.phone_numbers;
    }

    /**
   * @return optional descriptions of the type of each phone number. It could be like "HOME", but,
   *  there is no guaranteed or standard format.
   */
    pub fn  get_phone_types(&self) -> Vec<String>  {
        return self.phone_types;
    }

    pub fn  get_emails(&self) -> Vec<String>  {
        return self.emails;
    }

    /**
   * @return optional descriptions of the type of each e-mail. It could be like "WORK", but,
   *  there is no guaranteed or standard format.
   */
    pub fn  get_email_types(&self) -> Vec<String>  {
        return self.email_types;
    }

    pub fn  get_instant_messenger(&self) -> String  {
        return self.instant_messenger;
    }

    pub fn  get_note(&self) -> String  {
        return self.note;
    }

    pub fn  get_addresses(&self) -> Vec<String>  {
        return self.addresses;
    }

    /**
   * @return optional descriptions of the type of each e-mail. It could be like "WORK", but,
   *  there is no guaranteed or standard format.
   */
    pub fn  get_address_types(&self) -> Vec<String>  {
        return self.address_types;
    }

    pub fn  get_title(&self) -> String  {
        return self.title;
    }

    pub fn  get_org(&self) -> String  {
        return self.org;
    }

    pub fn  get_u_r_ls(&self) -> Vec<String>  {
        return self.urls;
    }

    /**
   * @return birthday formatted as yyyyMMdd (e.g. 19780917)
   */
    pub fn  get_birthday(&self) -> String  {
        return self.birthday;
    }

    /**
   * @return a location as a latitude/longitude pair
   */
    pub fn  get_geo(&self) -> Vec<String>  {
        return self.geo;
    }

    pub fn  get_display_result(&self) -> String  {
         let result: StringBuilder = StringBuilder::new(100);
        maybe_append(&self.names, &result);
        maybe_append(&self.nicknames, &result);
        maybe_append(&self.pronunciation, &result);
        maybe_append(&self.title, &result);
        maybe_append(&self.org, &result);
        maybe_append(&self.addresses, &result);
        maybe_append(&self.phone_numbers, &result);
        maybe_append(&self.emails, &result);
        maybe_append(&self.instant_messenger, &result);
        maybe_append(&self.urls, &result);
        maybe_append(&self.birthday, &result);
        maybe_append(&self.geo, &result);
        maybe_append(&self.note, &result);
        return result.to_string();
    }
}

// BizcardResultParser.java
/**
 * Implements the "BIZCARD" address book entry format, though this has been
 * largely reverse-engineered from examples observed in the wild -- still
 * looking for a definitive reference.
 *
 * @author Sean Owen
 */
pub struct BizcardResultParser {
    super: AbstractDoCoMoResultParser;
}

impl AbstractDoCoMoResultParser for BizcardResultParser{}

impl BizcardResultParser {

    // Yes, we extend AbstractDoCoMoResultParser since the format is very much
    // like the DoCoMo MECARD format, but this is not technically one of
    // DoCoMo's proposed formats
    pub fn  parse(&self,  result: &Result) -> AddressBookParsedResult  {
         let raw_text: String = get_massaged_text(result);
        if !raw_text.starts_with("BIZCARD:") {
            return null;
        }
         let first_name: String = match_single_do_co_mo_prefixed_field("N:", &raw_text, true);
         let last_name: String = match_single_do_co_mo_prefixed_field("X:", &raw_text, true);
         let full_name: String = ::build_name(&first_name, &last_name);
         let title: String = match_single_do_co_mo_prefixed_field("T:", &raw_text, true);
         let org: String = match_single_do_co_mo_prefixed_field("C:", &raw_text, true);
         let addresses: Vec<String> = match_do_co_mo_prefixed_field("A:", &raw_text);
         let phone_number1: String = match_single_do_co_mo_prefixed_field("B:", &raw_text, true);
         let phone_number2: String = match_single_do_co_mo_prefixed_field("M:", &raw_text, true);
         let phone_number3: String = match_single_do_co_mo_prefixed_field("F:", &raw_text, true);
         let email: String = match_single_do_co_mo_prefixed_field("E:", &raw_text, true);
        return AddressBookParsedResult::new(&maybe_wrap(&full_name), null, null, &::build_phone_numbers(&phone_number1, &phone_number2, &phone_number3), null, &maybe_wrap(&email), null, null, null, &addresses, null, &org, null, &title, null, null);
    }

    fn  build_phone_numbers( number1: &String,  number2: &String,  number3: &String) -> Vec<String>  {
         let numbers: List<String> = ArrayList<>::new(3);
        if number1 != null {
            numbers.add(&number1);
        }
        if number2 != null {
            numbers.add(&number2);
        }
        if number3 != null {
            numbers.add(&number3);
        }
         let size: i32 = numbers.size();
        if size == 0 {
            return null;
        }
        return numbers.to_array(: [Option<String>; size] = [None; size]);
    }

    fn  build_name( first_name: &String,  last_name: &String) -> String  {
        if first_name == null {
            return last_name;
        } else {
            return  if last_name == null { first_name } else { format!("{} {}", first_name, last_name) };
        }
    }
}

// BookmarkDoCoMoResultParser.java
/**
 * @author Sean Owen
 */
pub struct BookmarkDoCoMoResultParser {
    super: AbstractDoCoMoResultParser;
}

impl AbstractDoCoMoResultParser for BookmarkDoCoMoResultParser{}

impl BookmarkDoCoMoResultParser {

    pub fn  parse(&self,  result: &Result) -> URIParsedResult  {
         let raw_text: String = result.get_text();
        if !raw_text.starts_with("MEBKM:") {
            return null;
        }
         let title: String = match_single_do_co_mo_prefixed_field("TITLE:", &raw_text, true);
         let raw_uri: Vec<String> = match_do_co_mo_prefixed_field("URL:", &raw_text);
        if raw_uri == null {
            return null;
        }
         let uri: String = raw_uri[0];
        return  if URIResultParser::is_basically_valid_u_r_i(&uri) { URIParsedResult::new(&uri, &title) } else { null };
    }
}

// CalendarParsedResult.java
/**
 * Represents a parsed result that encodes a calendar event at a certain time, optionally
 * with attendees and a location.
 *
 * @author Sean Owen
 */

const RFC2445_DURATION: Pattern = Pattern::compile("P(?:(\\d+)W)?(?:(\\d+)D)?(?:T(?:(\\d+)H)?(?:(\\d+)M)?(?:(\\d+)S)?)?");

const RFC2445_DURATION_FIELD_UNITS: vec![Vec<i64>; 5] = vec![// 1 week
7 * 24 * 60 * 60 * 1000, // 1 day
24 * 60 * 60 * 1000, // 1 hour
60 * 60 * 1000, // 1 minute
60 * 1000, // 1 second
1000, ]
;

const DATE_TIME: Pattern = Pattern::compile("[0-9]{8}(T[0-9]{6}Z?)?");
pub struct CalendarParsedResult {
   super: ParsedResult;

    let summary: String;

    let mut start: i64;

    let start_all_day: bool;

    let mut end: i64;

    let end_all_day: bool;

    let location: String;

    let organizer: String;

    let attendees: Vec<String>;

    let description: String;

    let latitude: f64;

    let longitude: f64;
}

impl ParsedResult for CalendarParsedResult{}

impl CalendarParsedResult {

   pub fn new( summary: &String,  start_string: &String,  end_string: &String,  duration_string: &String,  location: &String,  organizer: &String,  attendees: &Vec<String>,  description: &String,  latitude: f64,  longitude: f64) -> CalendarParsedResult {
       super(ParsedResultType::CALENDAR);
       let .summary = summary;
       let tryResult1 = 0;
       'try1: loop {
       {
           let .start = ::parse_date(&start_string);
       }
       break 'try1
       }
       match tryResult1 {
            catch ( pe: &ParseException) {
               throw IllegalArgumentException::new(&pe.to_string());
           }  0 => break
       }

       if end_string == null {
            let duration_m_s: i64 = ::parse_duration_m_s(&duration_string);
           end =  if duration_m_s < 0 { -1 } else { start + duration_m_s };
       } else {
           let tryResult1 = 0;
           'try1: loop {
           {
               let .end = ::parse_date(&end_string);
           }
           break 'try1
           }
           match tryResult1 {
                catch ( pe: &ParseException) {
                   throw IllegalArgumentException::new(&pe.to_string());
               }  0 => break
           }

       }
       let .startAllDay = start_string.length() == 8;
       let .endAllDay = end_string != null && end_string.length() == 8;
       let .location = location;
       let .organizer = organizer;
       let .attendees = attendees;
       let .description = description;
       let .latitude = latitude;
       let .longitude = longitude;
   }

   pub fn  get_summary(&self) -> String  {
       return self.summary;
   }

   /**
  * @return start time
  * @deprecated use {@link #getStartTimestamp()}
  */
   pub fn  get_start(&self) -> Date  {
       return Date::new(self.start);
   }

   /**
  * @return start time
  * @see #getEndTimestamp()
  */
   pub fn  get_start_timestamp(&self) -> i64  {
       return self.start;
   }

   /**
  * @return true if start time was specified as a whole day
  */
   pub fn  is_start_all_day(&self) -> bool  {
       return self.start_all_day;
   }

   /**
  * @return event end {@link Date}, or {@code null} if event has no duration
  * @deprecated use {@link #getEndTimestamp()}
  */
   pub fn  get_end(&self) -> Date  {
       return  if self.end < 0 { null } else { Date::new(self.end) };
   }

   /**
  * @return event end {@link Date}, or -1 if event has no duration
  * @see #getStartTimestamp()
  */
   pub fn  get_end_timestamp(&self) -> i64  {
       return self.end;
   }

   /**
  * @return true if end time was specified as a whole day
  */
   pub fn  is_end_all_day(&self) -> bool  {
       return self.end_all_day;
   }

   pub fn  get_location(&self) -> String  {
       return self.location;
   }

   pub fn  get_organizer(&self) -> String  {
       return self.organizer;
   }

   pub fn  get_attendees(&self) -> Vec<String>  {
       return self.attendees;
   }

   pub fn  get_description(&self) -> String  {
       return self.description;
   }

   pub fn  get_latitude(&self) -> f64  {
       return self.latitude;
   }

   pub fn  get_longitude(&self) -> f64  {
       return self.longitude;
   }

   pub fn  get_display_result(&self) -> String  {
        let result: StringBuilder = StringBuilder::new(100);
       maybe_append(&self.summary, &result);
       maybe_append(&::format(self.start_all_day, self.start), &result);
       maybe_append(&::format(self.end_all_day, self.end), &result);
       maybe_append(&self.location, &result);
       maybe_append(&self.organizer, &result);
       maybe_append(&self.attendees, &result);
       maybe_append(&self.description, &result);
       return result.to_string();
   }

   /**
  * Parses a string as a date. RFC 2445 allows the start and end fields to be of type DATE (e.g. 20081021)
  * or DATE-TIME (e.g. 20081021T123000 for local time, or 20081021T123000Z for UTC).
  *
  * @param when The string to parse
  * @throws ParseException if not able to parse as a date
  */
   fn  parse_date( when: &String) -> /*  throws ParseException */Result<i64, Rc<Exception>>   {
       if !DATE_TIME::matcher(&when)::matches() {
           throw ParseException::new(&when, 0);
       }
       if when.length() == 8 {
           // Show only year/month/day
            let format: DateFormat = SimpleDateFormat::new("yyyyMMdd", Locale::ENGLISH);
           // For dates without a time, for purposes of interacting with Android, the resulting timestamp
           // needs to be midnight of that day in GMT. See:
           // http://code.google.com/p/android/issues/detail?id=8330
           format.set_time_zone(&TimeZone::get_time_zone("GMT"));
           return Ok(format.parse(&when).get_time());
       }
       // The when string can be local time, or UTC if it ends with a Z
       if when.length() == 16 && when.char_at(15) == 'Z' {
            let mut milliseconds: i64 = ::parse_date_time_string(&when.substring(0, 15));
            let calendar: Calendar = GregorianCalendar::new();
           // Account for time zone difference
           milliseconds += calendar.get(Calendar::ZONE_OFFSET);
           // Might need to correct for daylight savings time, but use target time since
           // now might be in DST but not then, or vice versa
           calendar.set_time(Date::new(milliseconds));
           return Ok(milliseconds + calendar.get(Calendar::DST_OFFSET));
       }
       return Ok(::parse_date_time_string(&when));
   }

   fn  format( all_day: bool,  date: i64) -> String  {
       if date < 0 {
           return null;
       }
        let format: DateFormat =  if all_day { DateFormat::get_date_instance(DateFormat::MEDIUM) } else { DateFormat::get_date_time_instance(DateFormat::MEDIUM, DateFormat::MEDIUM) };
       return format.format(date);
   }

   fn  parse_duration_m_s( duration_string: &CharSequence) -> i64  {
       if duration_string == null {
           return -1;
       }
        let m: Matcher = RFC2445_DURATION::matcher(&duration_string);
       if !m.matches() {
           return -1;
       }
        let duration_m_s: i64 = 0;
        {
            let mut i: i32 = 0;
           while i < RFC2445_DURATION_FIELD_UNITS.len() {
               {
                    let field_value: String = m.group(i + 1);
                   if field_value != null {
                       duration_m_s += RFC2445_DURATION_FIELD_UNITS[i] * Integer::parse_int(&field_value);
                   }
               }
               i += 1;
            }
        }

       return duration_m_s;
   }

   fn  parse_date_time_string( date_time_string: &String) -> /*  throws ParseException */Result<i64, Rc<Exception>>   {
        let format: DateFormat = SimpleDateFormat::new("yyyyMMdd'T'HHmmss", Locale::ENGLISH);
       return Ok(format.parse(&date_time_string).get_time());
   }
}

// EmailAddressParsedResult.java
/**
 * Represents a parsed result that encodes an email message including recipients, subject
 * and body text.
 *
 * @author Sean Owen
 */
pub struct EmailAddressParsedResult {
    super: ParsedResult;

     let tos: Vec<String>;

     let ccs: Vec<String>;

     let bccs: Vec<String>;

     let subject: String;

     let body: String;
}

impl ParsedResult for EmailAddressParsedResult{}

impl EmailAddressParsedResult {

    fn new( to: &String) -> EmailAddressParsedResult {
        this( : vec![String; 1] = vec![to, ]
        , null, null, null, null);
    }

    fn new( tos: &Vec<String>,  ccs: &Vec<String>,  bccs: &Vec<String>,  subject: &String,  body: &String) -> EmailAddressParsedResult {
        super(ParsedResultType::EMAIL_ADDRESS);
        let .tos = tos;
        let .ccs = ccs;
        let .bccs = bccs;
        let .subject = subject;
        let .body = body;
    }

    /**
   * @return first elements of {@link #getTos()} or {@code null} if none
   * @deprecated use {@link #getTos()}
   */
    pub fn  get_email_address(&self) -> String  {
        return  if self.tos == null || self.tos.len() == 0 { null } else { self.tos[0] };
    }

    pub fn  get_tos(&self) -> Vec<String>  {
        return self.tos;
    }

    pub fn  get_c_cs(&self) -> Vec<String>  {
        return self.ccs;
    }

    pub fn  get_b_c_cs(&self) -> Vec<String>  {
        return self.bccs;
    }

    pub fn  get_subject(&self) -> String  {
        return self.subject;
    }

    pub fn  get_body(&self) -> String  {
        return self.body;
    }

    /**
   * @return "mailto:"
   * @deprecated without replacement
   */
    pub fn  get_mailto_u_r_i(&self) -> String  {
        return "mailto:";
    }

    pub fn  get_display_result(&self) -> String  {
         let result: StringBuilder = StringBuilder::new(30);
        maybe_append(&self.tos, &result);
        maybe_append(&self.ccs, &result);
        maybe_append(&self.bccs, &result);
        maybe_append(&self.subject, &result);
        maybe_append(&self.body, &result);
        return result.to_string();
    }
}

// EmailAddressResultParser.java
/**
 * Represents a result that encodes an e-mail address, either as a plain address
 * like "joe@example.org" or a mailto: URL like "mailto:joe@example.org".
 *
 * @author Sean Owen
 */

const COMMA: Pattern = Pattern::compile(",");
pub struct EmailAddressResultParser {
    super: ResultParser;
}

impl ResultParser for EmailAddressResultParser{}

impl EmailAddressResultParser {

    pub fn  parse(&self,  result: &Result) -> EmailAddressParsedResult  {
         let raw_text: String = get_massaged_text(result);
        if raw_text.starts_with("mailto:") || raw_text.starts_with("MAILTO:") {
            // If it starts with mailto:, assume it is definitely trying to be an email address
             let host_email: String = raw_text.substring(7);
             let query_start: i32 = host_email.index_of('?');
            if query_start >= 0 {
                host_email = host_email.substring(0, query_start);
            }
            let tryResult1 = 0;
            'try1: loop {
            {
                host_email = url_decode(&host_email);
            }
            break 'try1
            }
            match tryResult1 {
                 catch ( iae: &IllegalArgumentException) {
                    return null;
                }  0 => break
            }

             let mut tos: Vec<String> = null;
            if !host_email.is_empty() {
                tos = COMMA::split(&host_email);
            }
             let name_values: Map<String, String> = parse_name_value_pairs(&raw_text);
             let mut ccs: Vec<String> = null;
             let mut bccs: Vec<String> = null;
             let mut subject: String = null;
             let mut body: String = null;
            if name_values != null {
                if tos == null {
                     let tos_string: String = name_values.get("to");
                    if tos_string != null {
                        tos = COMMA::split(&tos_string);
                    }
                }
                 let cc_string: String = name_values.get("cc");
                if cc_string != null {
                    ccs = COMMA::split(&cc_string);
                }
                 let bcc_string: String = name_values.get("bcc");
                if bcc_string != null {
                    bccs = COMMA::split(&bcc_string);
                }
                subject = name_values.get("subject");
                body = name_values.get("body");
            }
            return EmailAddressParsedResult::new(&tos, &ccs, &bccs, &subject, &body);
        } else {
            if !EmailDoCoMoResultParser::is_basically_valid_email_address(&raw_text) {
                return null;
            }
            return EmailAddressParsedResult::new(&raw_text);
        }
    }
}

// EmailDoCoMoResultParser.java
/**
 * Implements the "MATMSG" email message entry format.
 *
 * Supported keys: TO, SUB, BODY
 *
 * @author Sean Owen
 */

const ATEXT_ALPHANUMERIC: Pattern = Pattern::compile("[a-zA-Z0-9@.!#$%&'*+\\-/=?^_`{|}~]+");
pub struct EmailDoCoMoResultParser {
    super: AbstractDoCoMoResultParser;
}

impl AbstractDoCoMoResultParser for EmailDoCoMoResultParser{}

impl EmailDoCoMoResultParser {

    pub fn  parse(&self,  result: &Result) -> EmailAddressParsedResult  {
         let raw_text: String = get_massaged_text(result);
        if !raw_text.starts_with("MATMSG:") {
            return null;
        }
         let tos: Vec<String> = match_do_co_mo_prefixed_field("TO:", &raw_text);
        if tos == null {
            return null;
        }
        for  let to: String in tos {
            if !::is_basically_valid_email_address(&to) {
                return null;
            }
        }
         let subject: String = match_single_do_co_mo_prefixed_field("SUB:", &raw_text, false);
         let body: String = match_single_do_co_mo_prefixed_field("BODY:", &raw_text, false);
        return EmailAddressParsedResult::new(&tos, null, null, &subject, &body);
    }

    /**
   * This implements only the most basic checking for an email address's validity -- that it contains
   * an '@' and contains no characters disallowed by RFC 2822. This is an overly lenient definition of
   * validity. We want to generally be lenient here since this class is only intended to encapsulate what's
   * in a barcode, not "judge" it.
   */
    fn  is_basically_valid_email_address( email: &String) -> bool  {
        return email != null && ATEXT_ALPHANUMERIC::matcher(&email)::matches() && email.index_of('@') >= 0;
    }
}

// ExpandedProductParsedResult.java
/**
 * Represents a parsed result that encodes extended product information as encoded
 * by the RSS format, like weight, price, dates, etc.
 *
 * @author Antonio Manuel Benjumea Conde, Servinform, S.A.
 * @author AgustÃ­n Delgado, Servinform, S.A.
 */

const KILOGRAM: &'static str = "KG";

const POUND: &'static str = "LB";
pub struct ExpandedProductParsedResult {
   super: ParsedResult;

    let raw_text: String;

    let product_i_d: String;

    let sscc: String;

    let lot_number: String;

    let production_date: String;

    let packaging_date: String;

    let best_before_date: String;

    let expiration_date: String;

    let weight: String;

    let weight_type: String;

    let weight_increment: String;

    let price: String;

    let price_increment: String;

    let price_currency: String;

   // For AIS that not exist in this object
    let uncommon_a_is: Map<String, String>;
}

impl ParsedResult for ExpandedProductParsedResult{}

impl ExpandedProductParsedResult {

   pub fn new( raw_text: &String,  product_i_d: &String,  sscc: &String,  lot_number: &String,  production_date: &String,  packaging_date: &String,  best_before_date: &String,  expiration_date: &String,  weight: &String,  weight_type: &String,  weight_increment: &String,  price: &String,  price_increment: &String,  price_currency: &String,  uncommon_a_is: &Map<String, String>) -> ExpandedProductParsedResult {
       super(ParsedResultType::PRODUCT);
       let .rawText = raw_text;
       let .productID = product_i_d;
       let .sscc = sscc;
       let .lotNumber = lot_number;
       let .productionDate = production_date;
       let .packagingDate = packaging_date;
       let .bestBeforeDate = best_before_date;
       let .expirationDate = expiration_date;
       let .weight = weight;
       let .weightType = weight_type;
       let .weightIncrement = weight_increment;
       let .price = price;
       let .priceIncrement = price_increment;
       let .priceCurrency = price_currency;
       let .uncommonAIs = uncommon_a_is;
   }

   pub fn  equals(&self,  o: &Object) -> bool  {
       if !(o instanceof ExpandedProductParsedResult) {
           return false;
       }
        let other: ExpandedProductParsedResult = o as ExpandedProductParsedResult;
       return Objects::equals(&self.product_i_d, other.productID) && Objects::equals(&self.sscc, other.sscc) && Objects::equals(&self.lot_number, other.lotNumber) && Objects::equals(&self.production_date, other.productionDate) && Objects::equals(&self.best_before_date, other.bestBeforeDate) && Objects::equals(&self.expiration_date, other.expirationDate) && Objects::equals(&self.weight, other.weight) && Objects::equals(&self.weight_type, other.weightType) && Objects::equals(&self.weight_increment, other.weightIncrement) && Objects::equals(&self.price, other.price) && Objects::equals(&self.price_increment, other.priceIncrement) && Objects::equals(&self.price_currency, other.priceCurrency) && Objects::equals(&self.uncommon_a_is, other.uncommonAIs);
   }

   pub fn  hash_code(&self) -> i32  {
        let mut hash: i32 = Objects::hash_code(&self.product_i_d);
       hash ^= Objects::hash_code(&self.sscc);
       hash ^= Objects::hash_code(&self.lot_number);
       hash ^= Objects::hash_code(&self.production_date);
       hash ^= Objects::hash_code(&self.best_before_date);
       hash ^= Objects::hash_code(&self.expiration_date);
       hash ^= Objects::hash_code(&self.weight);
       hash ^= Objects::hash_code(&self.weight_type);
       hash ^= Objects::hash_code(&self.weight_increment);
       hash ^= Objects::hash_code(&self.price);
       hash ^= Objects::hash_code(&self.price_increment);
       hash ^= Objects::hash_code(&self.price_currency);
       hash ^= Objects::hash_code(&self.uncommon_a_is);
       return hash;
   }

   pub fn  get_raw_text(&self) -> String  {
       return self.raw_text;
   }

   pub fn  get_product_i_d(&self) -> String  {
       return self.product_i_d;
   }

   pub fn  get_sscc(&self) -> String  {
       return self.sscc;
   }

   pub fn  get_lot_number(&self) -> String  {
       return self.lot_number;
   }

   pub fn  get_production_date(&self) -> String  {
       return self.production_date;
   }

   pub fn  get_packaging_date(&self) -> String  {
       return self.packaging_date;
   }

   pub fn  get_best_before_date(&self) -> String  {
       return self.best_before_date;
   }

   pub fn  get_expiration_date(&self) -> String  {
       return self.expiration_date;
   }

   pub fn  get_weight(&self) -> String  {
       return self.weight;
   }

   pub fn  get_weight_type(&self) -> String  {
       return self.weight_type;
   }

   pub fn  get_weight_increment(&self) -> String  {
       return self.weight_increment;
   }

   pub fn  get_price(&self) -> String  {
       return self.price;
   }

   pub fn  get_price_increment(&self) -> String  {
       return self.price_increment;
   }

   pub fn  get_price_currency(&self) -> String  {
       return self.price_currency;
   }

   pub fn  get_uncommon_a_is(&self) -> Map<String, String>  {
       return self.uncommon_a_is;
   }

   pub fn  get_display_result(&self) -> String  {
       return String::value_of(&self.raw_text);
   }
}

// ExpandedProductResultParser.java
/**
 * Parses strings of digits that represent a RSS Extended code.
 * 
 * @author Antonio Manuel Benjumea Conde, Servinform, S.A.
 * @author AgustÃ­n Delgado, Servinform, S.A.
 */
pub struct ExpandedProductResultParser {
    super: ResultParser;
}

impl ResultParser for ExpandedProductResultParser{}

impl ExpandedProductResultParser {

    pub fn  parse(&self,  result: &Result) -> ExpandedProductParsedResult  {
         let format: BarcodeFormat = result.get_barcode_format();
        if format != BarcodeFormat::RSS_EXPANDED {
            // ExtendedProductParsedResult NOT created. Not a RSS Expanded barcode
            return null;
        }
         let raw_text: String = get_massaged_text(result);
         let product_i_d: String = null;
         let mut sscc: String = null;
         let lot_number: String = null;
         let production_date: String = null;
         let packaging_date: String = null;
         let best_before_date: String = null;
         let expiration_date: String = null;
         let mut weight: String = null;
         let weight_type: String = null;
         let weight_increment: String = null;
         let mut price: String = null;
         let price_increment: String = null;
         let price_currency: String = null;
         let uncommon_a_is: Map<String, String> = HashMap<>::new();
         let mut i: i32 = 0;
        while i < raw_text.length() {
             let ai: String = ::find_a_ivalue(i, &raw_text);
            if ai == null {
                // ExtendedProductParsedResult NOT created. Not match with RSS Expanded pattern
                return null;
            }
            i += ai.length() + 2;
             let value: String = ::find_value(i, &raw_text);
            i += value.length();
            match ai {
                  "00" => 
                     {
                        sscc = value;
                        break;
                    }
                  "01" => 
                     {
                        product_i_d = value;
                        break;
                    }
                  "10" => 
                     {
                        lot_number = value;
                        break;
                    }
                  "11" => 
                     {
                        production_date = value;
                        break;
                    }
                  "13" => 
                     {
                        packaging_date = value;
                        break;
                    }
                  "15" => 
                     {
                        best_before_date = value;
                        break;
                    }
                  "17" => 
                     {
                        expiration_date = value;
                        break;
                    }
                  "3100" => 
                     {
                    }
                  "3101" => 
                     {
                    }
                  "3102" => 
                     {
                    }
                  "3103" => 
                     {
                    }
                  "3104" => 
                     {
                    }
                  "3105" => 
                     {
                    }
                  "3106" => 
                     {
                    }
                  "3107" => 
                     {
                    }
                  "3108" => 
                     {
                    }
                  "3109" => 
                     {
                        weight = value;
                        weight_type = ExpandedProductParsedResult::KILOGRAM;
                        weight_increment = ai.substring(3);
                        break;
                    }
                  "3200" => 
                     {
                    }
                  "3201" => 
                     {
                    }
                  "3202" => 
                     {
                    }
                  "3203" => 
                     {
                    }
                  "3204" => 
                     {
                    }
                  "3205" => 
                     {
                    }
                  "3206" => 
                     {
                    }
                  "3207" => 
                     {
                    }
                  "3208" => 
                     {
                    }
                  "3209" => 
                     {
                        weight = value;
                        weight_type = ExpandedProductParsedResult::POUND;
                        weight_increment = ai.substring(3);
                        break;
                    }
                  "3920" => 
                     {
                    }
                  "3921" => 
                     {
                    }
                  "3922" => 
                     {
                    }
                  "3923" => 
                     {
                        price = value;
                        price_increment = ai.substring(3);
                        break;
                    }
                  "3930" => 
                     {
                    }
                  "3931" => 
                     {
                    }
                  "3932" => 
                     {
                    }
                  "3933" => 
                     {
                        if value.length() < 4 {
                            // ExtendedProductParsedResult NOT created. Not match with RSS Expanded pattern
                            return null;
                        }
                        price = value.substring(3);
                        price_currency = value.substring(0, 3);
                        price_increment = ai.substring(3);
                        break;
                    }
                _ => 
                     {
                        // No match with common AIs
                        uncommon_a_is.put(&ai, &value);
                        break;
                    }
            }
        }
        return ExpandedProductParsedResult::new(&raw_text, &product_i_d, &sscc, &lot_number, &production_date, &packaging_date, &best_before_date, &expiration_date, &weight, &weight_type, &weight_increment, &price, &price_increment, &price_currency, &uncommon_a_is);
    }

    fn  find_a_ivalue( i: i32,  raw_text: &String) -> String  {
         let c: char = raw_text.char_at(i);
        // First character must be a open parenthesis.If not, ERROR
        if c != '(' {
            return null;
        }
         let raw_text_aux: CharSequence = raw_text.substring(i + 1);
         let buf: StringBuilder = StringBuilder::new();
         {
             let mut index: i32 = 0;
            while index < raw_text_aux.length() {
                {
                     let current_char: char = raw_text_aux.char_at(index);
                    if current_char == ')' {
                        return buf.to_string();
                    }
                    if current_char < '0' || current_char > '9' {
                        return null;
                    }
                    buf.append(current_char);
                }
                index += 1;
             }
         }

        return buf.to_string();
    }

    fn  find_value( i: i32,  raw_text: &String) -> String  {
         let buf: StringBuilder = StringBuilder::new();
         let raw_text_aux: String = raw_text.substring(i);
         {
             let mut index: i32 = 0;
            while index < raw_text_aux.length() {
                {
                     let c: char = raw_text_aux.char_at(index);
                    if c == '(' {
                        // with the iteration
                        if ::find_a_ivalue(index, &raw_text_aux) != null {
                            break;
                        }
                        buf.append('(');
                    } else {
                        buf.append(c);
                    }
                }
                index += 1;
             }
         }

        return buf.to_string();
    }
}

// GeoResultParser.java
/**
 * Parses a "geo:" URI result, which specifies a location on the surface of
 * the Earth as well as an optional altitude above the surface. See
 * <a href="http://tools.ietf.org/html/draft-mayrhofer-geo-uri-00">
 * http://tools.ietf.org/html/draft-mayrhofer-geo-uri-00</a>.
 *
 * @author Sean Owen
 */

const GEO_URL_PATTERN: Pattern = Pattern::compile("geo:([\\-0-9.]+),([\\-0-9.]+)(?:,([\\-0-9.]+))?(?:\\?(.*))?", Pattern::CASE_INSENSITIVE);
pub struct GeoResultParser {
    super: ResultParser;
}

impl ResultParser for GeoResultParser {}

impl GeoResultParser {

    pub fn  parse(&self,  result: &Result) -> GeoParsedResult  {
         let raw_text: CharSequence = get_massaged_text(result);
         let matcher: Matcher = GEO_URL_PATTERN::matcher(&raw_text);
        if !matcher.matches() {
            return null;
        }
         let query: String = matcher.group(4);
         let mut latitude: f64;
         let mut longitude: f64;
         let mut altitude: f64;
        let tryResult1 = 0;
        'try1: loop {
        {
            latitude = Double::parse_double(&matcher.group(1));
            if latitude > 90.0 || latitude < -90.0 {
                return null;
            }
            longitude = Double::parse_double(&matcher.group(2));
            if longitude > 180.0 || longitude < -180.0 {
                return null;
            }
            if matcher.group(3) == null {
                altitude = 0.0;
            } else {
                altitude = Double::parse_double(&matcher.group(3));
                if altitude < 0.0 {
                    return null;
                }
            }
        }
        break 'try1
        }
        match tryResult1 {
             catch ( ignored: &NumberFormatException) {
                return null;
            }  0 => break
        }

        return GeoParsedResult::new(latitude, longitude, altitude, &query);
    }
}



// GeoParsedResult.java
/**
 * Represents a parsed result that encodes a geographic coordinate, with latitude,
 * longitude and altitude.
 *
 * @author Sean Owen
 */
pub struct GeoParsedResult {
    super: ParsedResult;

     let latitude: f64;

     let longitude: f64;

     let altitude: f64;

     let query: String;
}

impl ParsedResult for GeoParsedResult{}

impl GeoParsedResult {

    fn new( latitude: f64,  longitude: f64,  altitude: f64,  query: &String) -> GeoParsedResult {
        super(ParsedResultType::GEO);
        let .latitude = latitude;
        let .longitude = longitude;
        let .altitude = altitude;
        let .query = query;
    }

    pub fn  get_geo_u_r_i(&self) -> String  {
         let result: StringBuilder = StringBuilder::new();
        result.append("geo:");
        result.append(self.latitude);
        result.append(',');
        result.append(self.longitude);
        if self.altitude > 0.0 {
            result.append(',');
            result.append(self.altitude);
        }
        if self.query != null {
            result.append('?');
            result.append(&self.query);
        }
        return result.to_string();
    }

    /**
   * @return latitude in degrees
   */
    pub fn  get_latitude(&self) -> f64  {
        return self.latitude;
    }

    /**
   * @return longitude in degrees
   */
    pub fn  get_longitude(&self) -> f64  {
        return self.longitude;
    }

    /**
   * @return altitude in meters. If not specified, in the geo URI, returns 0.0
   */
    pub fn  get_altitude(&self) -> f64  {
        return self.altitude;
    }

    /**
   * @return query string associated with geo URI or null if none exists
   */
    pub fn  get_query(&self) -> String  {
        return self.query;
    }

    pub fn  get_display_result(&self) -> String  {
         let result: StringBuilder = StringBuilder::new(20);
        result.append(self.latitude);
        result.append(", ");
        result.append(self.longitude);
        if self.altitude > 0.0 {
            result.append(", ");
            result.append(self.altitude);
            result.append('m');
        }
        if self.query != null {
            result.append(" (");
            result.append(&self.query);
            result.append(')');
        }
        return result.to_string();
    }
}


// ProductParsedResult.java
/**
 * Represents a parsed result that encodes a product by an identifier of some kind.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
pub struct ProductParsedResult {
    super: ParsedResult;

     let product_i_d: String;

     let normalized_product_i_d: String;
}

impl ParsedResult for ProductParsedResult{}

impl ProductParsedResult {

    fn new( product_i_d: &String) -> ProductParsedResult {
        this(&product_i_d, &product_i_d);
    }

    fn new( product_i_d: &String,  normalized_product_i_d: &String) -> ProductParsedResult {
        super(ParsedResultType::PRODUCT);
        let .productID = product_i_d;
        let .normalizedProductID = normalized_product_i_d;
    }

    pub fn  get_product_i_d(&self) -> String  {
        return self.product_i_d;
    }

    pub fn  get_normalized_product_i_d(&self) -> String  {
        return self.normalized_product_i_d;
    }

    pub fn  get_display_result(&self) -> String  {
        return self.product_i_d;
    }
}


// ProductResultParser.java
/**
 * Parses strings of digits that represent a UPC code.
 * 
 * @author dswitkin@google.com (Daniel Switkin)
 */
pub struct ProductResultParser {
    super: ResultParser;
}

impl ResultParser for ProductResultParser {}

impl ProductResultParser {

    // Treat all UPC and EAN variants as UPCs, in the sense that they are all product barcodes.
    pub fn  parse(&self,  result: &Result) -> ProductParsedResult  {
         let format: BarcodeFormat = result.get_barcode_format();
        if !(format == BarcodeFormat::UPC_A || format == BarcodeFormat::UPC_E || format == BarcodeFormat::EAN_8 || format == BarcodeFormat::EAN_13) {
            return null;
        }
         let raw_text: String = get_massaged_text(result);
        if !is_string_of_digits(&raw_text, &raw_text.length()) {
            return null;
        }
        // Not actually checking the checksum again here    
         let normalized_product_i_d: String;
        // Expand UPC-E for purposes of searching
        if format == BarcodeFormat::UPC_E && raw_text.length() == 8 {
            normalized_product_i_d = UPCEReader::convert_u_p_c_eto_u_p_c_a(&raw_text);
        } else {
            normalized_product_i_d = raw_text;
        }
        return ProductParsedResult::new(&raw_text, &normalized_product_i_d);
    }
}


// SMSMMSResultParser.java
/**
 * <p>Parses an "sms:" URI result, which specifies a number to SMS.
 * See <a href="http://tools.ietf.org/html/rfc5724"> RFC 5724</a> on this.</p>
 *
 * <p>This class supports "via" syntax for numbers, which is not part of the spec.
 * For example "+12125551212;via=+12124440101" may appear as a number.
 * It also supports a "subject" query parameter, which is not mentioned in the spec.
 * These are included since they were mentioned in earlier IETF drafts and might be
 * used.</p>
 *
 * <p>This actually also parses URIs starting with "mms:" and treats them all the same way,
 * and effectively converts them to an "sms:" URI for purposes of forwarding to the platform.</p>
 *
 * @author Sean Owen
 */
pub struct SMSMMSResultParser {
    super: ResultParser;
}

impl ResultParser for SMSMMSResultParser {}

impl SMSMMSResultParser {

    pub fn  parse(&self,  result: &Result) -> SMSParsedResult  {
         let raw_text: String = get_massaged_text(result);
        if !(raw_text.starts_with("sms:") || raw_text.starts_with("SMS:") || raw_text.starts_with("mms:") || raw_text.starts_with("MMS:")) {
            return null;
        }
        // Check up front if this is a URI syntax string with query arguments
         let name_value_pairs: Map<String, String> = parse_name_value_pairs(&raw_text);
         let mut subject: String = null;
         let mut body: String = null;
         let query_syntax: bool = false;
        if name_value_pairs != null && !name_value_pairs.is_empty() {
            subject = name_value_pairs.get("subject");
            body = name_value_pairs.get("body");
            query_syntax = true;
        }
        // Drop sms, query portion
         let query_start: i32 = raw_text.index_of('?', 4);
         let sms_u_r_i_without_query: String;
        // If it's not query syntax, the question mark is part of the subject or message
        if query_start < 0 || !query_syntax {
            sms_u_r_i_without_query = raw_text.substring(4);
        } else {
            sms_u_r_i_without_query = raw_text.substring(4, query_start);
        }
         let last_comma: i32 = -1;
         let mut comma: i32;
         let numbers: List<String> = ArrayList<>::new(1);
         let vias: List<String> = ArrayList<>::new(1);
        while (comma = sms_u_r_i_without_query.index_of(',', last_comma + 1)) > last_comma {
             let number_part: String = sms_u_r_i_without_query.substring(last_comma + 1, comma);
            ::add_number_via(&numbers, &vias, &number_part);
            last_comma = comma;
        }
        ::add_number_via(&numbers, &vias, &sms_u_r_i_without_query.substring(last_comma + 1));
        return SMSParsedResult::new(&numbers.to_array(EMPTY_STR_ARRAY), &vias.to_array(EMPTY_STR_ARRAY), &subject, &body);
    }

    fn  add_number_via( numbers: &Collection<String>,  vias: &Collection<String>,  number_part: &String)   {
         let number_end: i32 = number_part.index_of(';');
        if number_end < 0 {
            numbers.add(&number_part);
            vias.add(null);
        } else {
            numbers.add(&number_part.substring(0, number_end));
             let maybe_via: String = number_part.substring(number_end + 1);
             let mut via: String;
            if maybe_via.starts_with("via=") {
                via = maybe_via.substring(4);
            } else {
                via = null;
            }
            vias.add(&via);
        }
    }
}


// SMSParsedResult.java
/**
 * Represents a parsed result that encodes an SMS message, including recipients, subject
 * and body text.
 *
 * @author Sean Owen
 */
pub struct SMSParsedResult {
    super: ParsedResult;

     let mut numbers: Vec<String>;

     let mut vias: Vec<String>;

     let subject: String;

     let body: String;
}

impl ParsedResult for SMSParsedResult {}

impl SMSParsedResult {

    pub fn new( number: &String,  via: &String,  subject: &String,  body: &String) -> SMSParsedResult {
        super(ParsedResultType::SMS);
        let .numbers =  : vec![String; 1] = vec![number, ]
        ;
        let .vias =  : vec![String; 1] = vec![via, ]
        ;
        let .subject = subject;
        let .body = body;
    }

    pub fn new( numbers: &Vec<String>,  vias: &Vec<String>,  subject: &String,  body: &String) -> SMSParsedResult {
        super(ParsedResultType::SMS);
        let .numbers = numbers;
        let .vias = vias;
        let .subject = subject;
        let .body = body;
    }

    pub fn  get_s_m_s_u_r_i(&self) -> String  {
         let result: StringBuilder = StringBuilder::new();
        result.append("sms:");
         let mut first: bool = true;
         {
             let mut i: i32 = 0;
            while i < self.numbers.len() {
                {
                    if first {
                        first = false;
                    } else {
                        result.append(',');
                    }
                    result.append(self.numbers[i]);
                    if self.vias != null && self.vias[i] != null {
                        result.append(";via=");
                        result.append(self.vias[i]);
                    }
                }
                i += 1;
             }
         }

         let has_body: bool = self.body != null;
         let has_subject: bool = self.subject != null;
        if has_body || has_subject {
            result.append('?');
            if has_body {
                result.append("body=");
                result.append(&self.body);
            }
            if has_subject {
                if has_body {
                    result.append('&');
                }
                result.append("subject=");
                result.append(&self.subject);
            }
        }
        return result.to_string();
    }

    pub fn  get_numbers(&self) -> Vec<String>  {
        return self.numbers;
    }

    pub fn  get_vias(&self) -> Vec<String>  {
        return self.vias;
    }

    pub fn  get_subject(&self) -> String  {
        return self.subject;
    }

    pub fn  get_body(&self) -> String  {
        return self.body;
    }

    pub fn  get_display_result(&self) -> String  {
         let result: StringBuilder = StringBuilder::new(100);
        maybe_append(&self.numbers, &result);
        maybe_append(&self.subject, &result);
        maybe_append(&self.body, &result);
        return result.to_string();
    }
}


// SMSTOMMSTOResultParser.java
/**
 * <p>Parses an "smsto:" URI result, whose format is not standardized but appears to be like:
 * {@code smsto:number(:body)}.</p>
 *
 * <p>This actually also parses URIs starting with "smsto:", "mmsto:", "SMSTO:", and
 * "MMSTO:", and treats them all the same way, and effectively converts them to an "sms:" URI
 * for purposes of forwarding to the platform.</p>
 *
 * @author Sean Owen
 */
pub struct SMSTOMMSTOResultParser {
    super: ResultParser;
}

impl ResultParser for SMSTOMMSTOResultParser{}

impl SMSTOMMSTOResultParser {

    pub fn  parse(&self,  result: &Result) -> SMSParsedResult  {
         let raw_text: String = get_massaged_text(result);
        if !(raw_text.starts_with("smsto:") || raw_text.starts_with("SMSTO:") || raw_text.starts_with("mmsto:") || raw_text.starts_with("MMSTO:")) {
            return null;
        }
        // Thanks to dominik.wild for suggesting this enhancement to support
        // smsto:number:body URIs
         let mut number: String = raw_text.substring(6);
         let mut body: String = null;
         let body_start: i32 = number.index_of(':');
        if body_start >= 0 {
            body = number.substring(body_start + 1);
            number = number.substring(0, body_start);
        }
        return SMSParsedResult::new(&number, null, null, &body);
    }
}


// SMTPResultParser.java
/**
 * <p>Parses an "smtp:" URI result, whose format is not standardized but appears to be like:
 * {@code smtp[:subject[:body]]}.</p>
 *
 * @author Sean Owen
 */
pub struct SMTPResultParser {
    super: ResultParser;
}

impl ResultParser for SMTPResultParser {}

impl SMTPResultParser {

    pub fn  parse(&self,  result: &Result) -> EmailAddressParsedResult  {
         let raw_text: String = get_massaged_text(result);
        if !(raw_text.starts_with("smtp:") || raw_text.starts_with("SMTP:")) {
            return null;
        }
         let email_address: String = raw_text.substring(5);
         let mut subject: String = null;
         let mut body: String = null;
         let mut colon: i32 = email_address.index_of(':');
        if colon >= 0 {
            subject = email_address.substring(colon + 1);
            email_address = email_address.substring(0, colon);
            colon = subject.index_of(':');
            if colon >= 0 {
                body = subject.substring(colon + 1);
                subject = subject.substring(0, colon);
            }
        }
        return EmailAddressParsedResult::new( : vec![String; 1] = vec![email_address, ]
        , null, null, &subject, &body);
    }
}


// TelParsedResult.java
/**
 * Represents a parsed result that encodes a telephone number.
 *
 * @author Sean Owen
 */
pub struct TelParsedResult {
    super: ParsedResult;

     let number: String;

     let tel_u_r_i: String;

     let title: String;
}

impl ParsedResult for TelParsedResult {}

impl TelParsedResult {

    pub fn new( number: &String,  tel_u_r_i: &String,  title: &String) -> TelParsedResult {
        super(ParsedResultType::TEL);
        let .number = number;
        let .telURI = tel_u_r_i;
        let .title = title;
    }

    pub fn  get_number(&self) -> String  {
        return self.number;
    }

    pub fn  get_tel_u_r_i(&self) -> String  {
        return self.tel_u_r_i;
    }

    pub fn  get_title(&self) -> String  {
        return self.title;
    }

    pub fn  get_display_result(&self) -> String  {
         let result: StringBuilder = StringBuilder::new(20);
        maybe_append(&self.number, &result);
        maybe_append(&self.title, &result);
        return result.to_string();
    }
}


// TelResultParser.java
/**
 * Parses a "tel:" URI result, which specifies a phone number.
 *
 * @author Sean Owen
 */
pub struct TelResultParser {
    super: ResultParser;
}

impl ResultParser for TelResultParser{}

impl TelResultParser {

    pub fn  parse(&self,  result: &Result) -> TelParsedResult  {
         let raw_text: String = get_massaged_text(result);
        if !raw_text.starts_with("tel:") && !raw_text.starts_with("TEL:") {
            return null;
        }
        // Normalize "TEL:" to "tel:"
         let tel_u_r_i: String =  if raw_text.starts_with("TEL:") { format!("tel:{}", raw_text.substring(4)) } else { raw_text };
        // Drop tel, query portion
         let query_start: i32 = raw_text.index_of('?', 4);
         let number: String =  if query_start < 0 { raw_text.substring(4) } else { raw_text.substring(4, query_start) };
        return TelParsedResult::new(&number, &tel_u_r_i, null);
    }
}


// TextParsedResult.java
/**
 * A simple result type encapsulating a string that has no further
 * interpretation.
 * 
 * @author Sean Owen
 */
pub struct TextParsedResult {
    super: ParsedResult;

     let text: String;

     let language: String;
}

impl ParsedResult for TextParsedResult{}

impl TextParsedResult {

    pub fn new( text: &String,  language: &String) -> TextParsedResult {
        super(ParsedResultType::TEXT);
        let .text = text;
        let .language = language;
    }

    pub fn  get_text(&self) -> String  {
        return self.text;
    }

    pub fn  get_language(&self) -> String  {
        return self.language;
    }

    pub fn  get_display_result(&self) -> String  {
        return self.text;
    }
}

// URIParsedResult.java
/**
 * A simple result type encapsulating a URI that has no further interpretation.
 *
 * @author Sean Owen
 */
pub struct URIParsedResult {
    super: ParsedResult;

     let uri: String;

     let title: String;
}

impl ParsedResult for URIParsedResult{}

impl URIParsedResult {

    pub fn new( uri: &String,  title: &String) -> URIParsedResult {
        super(ParsedResultType::URI);
        let .uri = ::massage_u_r_i(&uri);
        let .title = title;
    }

    pub fn  get_u_r_i(&self) -> String  {
        return self.uri;
    }

    pub fn  get_title(&self) -> String  {
        return self.title;
    }

    /**
   * @return true if the URI contains suspicious patterns that may suggest it intends to
   *  mislead the user about its true nature
   * @deprecated see {@link URIResultParser#isPossiblyMaliciousURI(String)}
   */
    pub fn  is_possibly_malicious_u_r_i(&self) -> bool  {
        return URIResultParser::is_possibly_malicious_u_r_i(&self.uri);
    }

    pub fn  get_display_result(&self) -> String  {
         let result: StringBuilder = StringBuilder::new(30);
        maybe_append(&self.title, &result);
        maybe_append(&self.uri, &result);
        return result.to_string();
    }

    /**
   * Transforms a string that represents a URI into something more proper, by adding or canonicalizing
   * the protocol.
   */
    fn  massage_u_r_i( uri: &String) -> String  {
        uri = uri.trim();
         let protocol_end: i32 = uri.index_of(':');
        if protocol_end < 0 || ::is_colon_followed_by_port_number(&uri, protocol_end) {
            // No protocol, or found a colon, but it looks like it is after the host, so the protocol is still missing,
            // so assume http
            uri = format!("http://{}", uri);
        }
        return uri;
    }

    fn  is_colon_followed_by_port_number( uri: &String,  protocol_end: i32) -> bool  {
         let start: i32 = protocol_end + 1;
         let next_slash: i32 = uri.index_of('/', start);
        if next_slash < 0 {
            next_slash = uri.length();
        }
        return ResultParser::is_substring_of_digits(&uri, start, next_slash - start);
    }
}



// URIResultParser.java
/**
 * Tries to parse results that are a URI of some kind.
 * 
 * @author Sean Owen
 */

const ALLOWED_URI_CHARS_PATTERN: Pattern = Pattern::compile("[-._~:/?#\\[\\]@!$&'()*+,;=%A-Za-z0-9]+");

const USER_IN_HOST: Pattern = Pattern::compile(":/*([^/@]+)@[^/]+");

// See http://www.ietf.org/rfc/rfc2396.txt
const URL_WITH_PROTOCOL_PATTERN: Pattern = Pattern::compile("[a-zA-Z][a-zA-Z0-9+-.]+:");

const URL_WITHOUT_PROTOCOL_PATTERN: Pattern = Pattern::compile(format!("([a-zA-Z0-9\\-]+\\.){1,6}[a-zA-Z]{2,}(:\\d{1,5})?(/|\\?|$)"));
pub struct URIResultParser {
   super: ResultParser;
}

impl ResultParser for URIResultParser{}

impl URIResultParser {

   pub fn  parse(&self,  result: &Result) -> URIParsedResult  {
        let raw_text: String = get_massaged_text(result);
       // Assume anything starting this way really means to be a URI
       if raw_text.starts_with("URL:") || raw_text.starts_with("URI:") {
           return URIParsedResult::new(&raw_text.substring(4).trim(), null);
       }
       raw_text = raw_text.trim();
       if !::is_basically_valid_u_r_i(&raw_text) || ::is_possibly_malicious_u_r_i(&raw_text) {
           return null;
       }
       return URIParsedResult::new(&raw_text, null);
   }

   /**
  * @return true if the URI contains suspicious patterns that may suggest it intends to
  *  mislead the user about its true nature. At the moment this looks for the presence
  *  of user/password syntax in the host/authority portion of a URI which may be used
  *  in attempts to make the URI's host appear to be other than it is. Example:
  *  http://yourbank.com@phisher.com  This URI connects to phisher.com but may appear
  *  to connect to yourbank.com at first glance.
  */
   fn  is_possibly_malicious_u_r_i( uri: &String) -> bool  {
       return !ALLOWED_URI_CHARS_PATTERN::matcher(&uri)::matches() || USER_IN_HOST::matcher(&uri)::find();
   }

   fn  is_basically_valid_u_r_i( uri: &String) -> bool  {
       if uri.contains(" ") {
           // Quick hack check for a common case
           return false;
       }
        let mut m: Matcher = URL_WITH_PROTOCOL_PATTERN::matcher(&uri);
       if m.find() && m.start() == 0 {
           // match at start only
           return true;
       }
       m = URL_WITHOUT_PROTOCOL_PATTERN::matcher(&uri);
       return m.find() && m.start() == 0;
   }
}


// URLTOResultParser.java
/**
 * Parses the "URLTO" result format, which is of the form "URLTO:[title]:[url]".
 * This seems to be used sometimes, but I am not able to find documentation
 * on its origin or official format?
 *
 * @author Sean Owen
 */
pub struct URLTOResultParser {
    super: ResultParser;
}

impl ResultParser for URLTOResultParser{}

impl URLTOResultParser {

    pub fn  parse(&self,  result: &Result) -> URIParsedResult  {
         let raw_text: String = get_massaged_text(result);
        if !raw_text.starts_with("urlto:") && !raw_text.starts_with("URLTO:") {
            return null;
        }
         let title_end: i32 = raw_text.index_of(':', 6);
        if title_end < 0 {
            return null;
        }
         let title: String =  if title_end <= 6 { null } else { raw_text.substring(6, title_end) };
         let uri: String = raw_text.substring(title_end + 1);
        return URIParsedResult::new(&uri, &title);
    }
}



// VCardResultParser.java
/**
 * Parses contact information formatted according to the VCard (2.1) format. This is not a complete
 * implementation but should parse information as commonly encoded in 2D barcodes.
 *
 * @author Sean Owen
 */

const BEGIN_VCARD: Pattern = Pattern::compile("BEGIN:VCARD", Pattern::CASE_INSENSITIVE);

const VCARD_LIKE_DATE: Pattern = Pattern::compile("\\d{4}-?\\d{2}-?\\d{2}");

const CR_LF_SPACE_TAB: Pattern = Pattern::compile("\r\n[ \t]");

const NEWLINE_ESCAPE: Pattern = Pattern::compile("\\\\[nN]");

const VCARD_ESCAPES: Pattern = Pattern::compile("\\\\([,;\\\\])");

const EQUALS: Pattern = Pattern::compile("=");

const SEMICOLON: Pattern = Pattern::compile(";");

const UNESCAPED_SEMICOLONS: Pattern = Pattern::compile("(?<!\\\\);+");

const COMMA: Pattern = Pattern::compile(",");

const SEMICOLON_OR_COMMA: Pattern = Pattern::compile("[;,]");
pub struct VCardResultParser {
   super: ResultParser;
}

impl ResultParser for VCardResultParser{}

impl VCardResultParser {

   pub fn  parse(&self,  result: &Result) -> AddressBookParsedResult  {
       // Although we should insist on the raw text ending with "END:VCARD", there's no reason
       // to throw out everything else we parsed just because this was omitted. In fact, Eclair
       // is doing just that, and we can't parse its contacts without this leniency.
        let raw_text: String = get_massaged_text(result);
        let m: Matcher = BEGIN_VCARD::matcher(&raw_text);
       if !m.find() || m.start() != 0 {
           return null;
       }
        let mut names: List<List<String>> = ::match_v_card_prefixed_field("FN", &raw_text, true, false);
       if names == null {
           // If no display names found, look for regular name fields and format them
           names = ::match_v_card_prefixed_field("N", &raw_text, true, false);
           ::format_names(&names);
       }
        let nickname_string: List<String> = ::match_single_v_card_prefixed_field("NICKNAME", &raw_text, true, false);
        let nicknames: Vec<String> =  if nickname_string == null { null } else { COMMA::split(&nickname_string.get(0)) };
        let phone_numbers: List<List<String>> = ::match_v_card_prefixed_field("TEL", &raw_text, true, false);
        let emails: List<List<String>> = ::match_v_card_prefixed_field("EMAIL", &raw_text, true, false);
        let note: List<String> = ::match_single_v_card_prefixed_field("NOTE", &raw_text, false, false);
        let addresses: List<List<String>> = ::match_v_card_prefixed_field("ADR", &raw_text, true, true);
        let org: List<String> = ::match_single_v_card_prefixed_field("ORG", &raw_text, true, true);
        let mut birthday: List<String> = ::match_single_v_card_prefixed_field("BDAY", &raw_text, true, false);
       if birthday != null && !::is_like_v_card_date(&birthday.get(0)) {
           birthday = null;
       }
        let title: List<String> = ::match_single_v_card_prefixed_field("TITLE", &raw_text, true, false);
        let urls: List<List<String>> = ::match_v_card_prefixed_field("URL", &raw_text, true, false);
        let instant_messenger: List<String> = ::match_single_v_card_prefixed_field("IMPP", &raw_text, true, false);
        let geo_string: List<String> = ::match_single_v_card_prefixed_field("GEO", &raw_text, true, false);
        let mut geo: Vec<String> =  if geo_string == null { null } else { SEMICOLON_OR_COMMA::split(&geo_string.get(0)) };
       if geo != null && geo.len() != 2 {
           geo = null;
       }
       return AddressBookParsedResult::new(&::to_primary_values(&names), &nicknames, null, &::to_primary_values(&phone_numbers), &::to_types(&phone_numbers), &::to_primary_values(&emails), &::to_types(&emails), &::to_primary_value(&instant_messenger), &::to_primary_value(&note), &::to_primary_values(&addresses), &::to_types(&addresses), &::to_primary_value(&org), &::to_primary_value(&birthday), &::to_primary_value(&title), &::to_primary_values(&urls), &geo);
   }

   fn  match_v_card_prefixed_field( prefix: &CharSequence,  raw_text: &String,  trim: bool,  parse_field_divider: bool) -> List<List<String>>  {
        let mut matches: List<List<String>> = null;
        let mut i: i32 = 0;
        let max: i32 = raw_text.length();
       while i < max {
           // At start or after newline, match prefix, followed by optional metadata 
           // (led by ;) ultimately ending in colon
            let matcher: Matcher = Pattern::compile(format!("(?:^|\n){}(?:;([^:]*))?:", prefix), Pattern::CASE_INSENSITIVE)::matcher(&raw_text);
           if i > 0 {
               // Find from i-1 not i since looking at the preceding character
               i -= 1;
           }
           if !matcher.find(i) {
               break;
           }
           // group 0 = whole pattern; end(0) is past final colon
           i = matcher.end(0);
           // group 1 = metadata substring
            let metadata_string: String = matcher.group(1);
            let mut metadata: List<String> = null;
            let quoted_printable: bool = false;
            let quoted_printable_charset: String = null;
            let value_type: String = null;
           if metadata_string != null {
               for  let metadatum: String in SEMICOLON::split(&metadata_string) {
                   if metadata == null {
                       metadata = ArrayList<>::new(1);
                   }
                   metadata.add(&metadatum);
                    let metadatum_tokens: Vec<String> = EQUALS::split(&metadatum, 2);
                   if metadatum_tokens.len() > 1 {
                        let key: String = metadatum_tokens[0];
                        let value: String = metadatum_tokens[1];
                       if "ENCODING".equals_ignore_case(&key) && "QUOTED-PRINTABLE".equals_ignore_case(&value) {
                           quoted_printable = true;
                       } else if "CHARSET".equals_ignore_case(&key) {
                           quoted_printable_charset = value;
                       } else if "VALUE".equals_ignore_case(&key) {
                           value_type = value;
                       }
                   }
               }
           }
           // Found the start of a match here
            let match_start: i32 = i;
           while (i = raw_text.index_of('\n', i)) >= 0 {
               // Really, end in \r\n
               if // But if followed by tab or space,
               i < raw_text.length() - 1 && (// this is only a continuation
               raw_text.char_at(i + 1) == ' ' || raw_text.char_at(i + 1) == '\t') {
                   // Skip \n and continutation whitespace
                   i += 2;
               } else if // If preceded by = in quoted printable
               quoted_printable && (// this is a continuation
               (i >= 1 && raw_text.char_at(i - 1) == '=') || (i >= 2 && raw_text.char_at(i - 2) == '=')) {
                   // Skip \n
                   i += 1;
               } else {
                   break;
               }
           }
           if i < 0 {
               // No terminating end character? uh, done. Set i such that loop terminates and break
               i = max;
           } else if i > match_start {
               // found a match
               if matches == null {
                   // lazy init
                   matches = ArrayList<>::new(1);
               }
               if i >= 1 && raw_text.char_at(i - 1) == '\r' {
                   // Back up over \r, which really should be there
                   i -= 1;
               }
                let mut element: String = raw_text.substring(match_start, i);
               if trim {
                   element = element.trim();
               }
               if quoted_printable {
                   element = ::decode_quoted_printable(&element, &quoted_printable_charset);
                   if parse_field_divider {
                       element = UNESCAPED_SEMICOLONS::matcher(&element)::replace_all("\n")::trim();
                   }
               } else {
                   if parse_field_divider {
                       element = UNESCAPED_SEMICOLONS::matcher(&element)::replace_all("\n")::trim();
                   }
                   element = CR_LF_SPACE_TAB::matcher(&element)::replace_all("");
                   element = NEWLINE_ESCAPE::matcher(&element)::replace_all("\n");
                   element = VCARD_ESCAPES::matcher(&element)::replace_all("$1");
               }
               // Only handle VALUE=uri specially
               if "uri".equals(&value_type) {
                   // as value, to support tel: and mailto:
                   let tryResult1 = 0;
                   'try1: loop {
                   {
                       element = URI::create(&element)::get_scheme_specific_part();
                   }
                   break 'try1
                   }
                   match tryResult1 {
                        catch ( iae: &IllegalArgumentException) {
                       }  0 => break
                   }

               }
               if metadata == null {
                    let match: List<String> = ArrayList<>::new(1);
                   match.add(&element);
                   matches.add(&match);
               } else {
                   metadata.add(0, &element);
                   matches.add(&metadata);
               }
               i += 1;
           } else {
               i += 1;
           }
       }
       return matches;
   }

   fn  decode_quoted_printable( value: &CharSequence,  charset: &String) -> String  {
        let length: i32 = value.length();
        let result: StringBuilder = StringBuilder::new(length);
        let fragment_buffer: ByteArrayOutputStream = ByteArrayOutputStream::new();
        {
            let mut i: i32 = 0;
           while i < length {
               {
                    let c: char = value.char_at(i);
                   match c {
                         '\r' => 
                            {
                           }
                         '\n' => 
                            {
                               break;
                           }
                         '=' => 
                            {
                               if i < length - 2 {
                                    let next_char: char = value.char_at(i + 1);
                                   if next_char != '\r' && next_char != '\n' {
                                        let next_next_char: char = value.char_at(i + 2);
                                        let first_digit: i32 = parse_hex_digit(next_char);
                                        let second_digit: i32 = parse_hex_digit(next_next_char);
                                       if first_digit >= 0 && second_digit >= 0 {
                                           fragment_buffer.write((first_digit << 4) + second_digit);
                                       }
                                       // else ignore it, assume it was incorrectly encoded
                                       i += 2;
                                   }
                               }
                               break;
                           }
                       _ => 
                            {
                               ::maybe_append_fragment(&fragment_buffer, &charset, &result);
                               result.append(c);
                           }
                   }
               }
               i += 1;
            }
        }

       ::maybe_append_fragment(&fragment_buffer, &charset, &result);
       return result.to_string();
   }

   fn  maybe_append_fragment( fragment_buffer: &ByteArrayOutputStream,  charset: &String,  result: &StringBuilder)   {
       if fragment_buffer.size() > 0 {
            let fragment_bytes: Vec<i8> = fragment_buffer.to_byte_array();
            let mut fragment: String;
           if charset == null {
               fragment = String::new(&fragment_bytes, StandardCharsets::UTF_8);
           } else {
               let tryResult1 = 0;
               'try1: loop {
               {
                   fragment = String::new(&fragment_bytes, &charset);
               }
               break 'try1
               }
               match tryResult1 {
                    catch ( e: &UnsupportedEncodingException) {
                       fragment = String::new(&fragment_bytes, StandardCharsets::UTF_8);
                   }  0 => break
               }

           }
           fragment_buffer.reset();
           result.append(&fragment);
       }
   }

   fn  match_single_v_card_prefixed_field( prefix: &CharSequence,  raw_text: &String,  trim: bool,  parse_field_divider: bool) -> List<String>  {
        let values: List<List<String>> = ::match_v_card_prefixed_field(&prefix, &raw_text, trim, parse_field_divider);
       return  if values == null || values.is_empty() { null } else { values.get(0) };
   }

   fn  to_primary_value( list: &List<String>) -> String  {
       return  if list == null || list.is_empty() { null } else { list.get(0) };
   }

   fn  to_primary_values( lists: &Collection<List<String>>) -> Vec<String>  {
       if lists == null || lists.is_empty() {
           return null;
       }
        let result: List<String> = ArrayList<>::new(&lists.size());
       for  let list: List<String> in lists {
            let value: String = list.get(0);
           if value != null && !value.is_empty() {
               result.add(&value);
           }
       }
       return result.to_array(EMPTY_STR_ARRAY);
   }

   fn  to_types( lists: &Collection<List<String>>) -> Vec<String>  {
       if lists == null || lists.is_empty() {
           return null;
       }
        let result: List<String> = ArrayList<>::new(&lists.size());
       for  let list: List<String> in lists {
            let value: String = list.get(0);
           if value != null && !value.is_empty() {
                let mut type: String = null;
                {
                    let mut i: i32 = 1;
                   while i < list.size() {
                       {
                            let metadatum: String = list.get(i);
                            let equals: i32 = metadatum.index_of('=');
                           if equals < 0 {
                               // take the whole thing as a usable label
                               type = metadatum;
                               break;
                           }
                           if "TYPE".equals_ignore_case(&metadatum.substring(0, equals)) {
                               type = metadatum.substring(equals + 1);
                               break;
                           }
                       }
                       i += 1;
                    }
                }

               result.add(&type);
           }
       }
       return result.to_array(EMPTY_STR_ARRAY);
   }

   fn  is_like_v_card_date( value: &CharSequence) -> bool  {
       return value == null || VCARD_LIKE_DATE::matcher(&value)::matches();
   }

   /**
  * Formats name fields of the form "Public;John;Q.;Reverend;III" into a form like
  * "Reverend John Q. Public III".
  *
  * @param names name values to format, in place
  */
   fn  format_names( names: &Iterable<List<String>>)   {
       if names != null {
           for  let list: List<String> in names {
                let name: String = list.get(0);
                let mut components: [Option<String>; 5] = [None; 5];
                let mut start: i32 = 0;
                let mut end: i32;
                let component_index: i32 = 0;
               while component_index < components.len() - 1 && (end = name.index_of(';', start)) >= 0 {
                   components[component_index] = name.substring(start, end);
                   component_index += 1;
                   start = end + 1;
               }
               components[component_index] = name.substring(start);
                let new_name: StringBuilder = StringBuilder::new(100);
               ::maybe_append_component(&components, 3, &new_name);
               ::maybe_append_component(&components, 1, &new_name);
               ::maybe_append_component(&components, 2, &new_name);
               ::maybe_append_component(&components, 0, &new_name);
               ::maybe_append_component(&components, 4, &new_name);
               list.set(0, &new_name.to_string().trim());
           }
       }
   }

   fn  maybe_append_component( components: &Vec<String>,  i: i32,  new_name: &StringBuilder)   {
       if components[i] != null && !components[i].is_empty() {
           if new_name.length() > 0 {
               new_name.append(' ');
           }
           new_name.append(components[i]);
       }
   }
}

// VEventResultParser.java
/**
 * Partially implements the iCalendar format's "VEVENT" format for specifying a
 * calendar event. See RFC 2445. This supports SUMMARY, LOCATION, GEO, DTSTART and DTEND fields.
 *
 * @author Sean Owen
 */
pub struct VEventResultParser {
    super: ResultParser;
}

impl ResultParser for VEventResultParser{}

impl VEventResultParser {

    pub fn  parse(&self,  result: &Result) -> CalendarParsedResult  {
         let raw_text: String = get_massaged_text(result);
         let v_event_start: i32 = raw_text.index_of("BEGIN:VEVENT");
        if v_event_start < 0 {
            return null;
        }
         let summary: String = ::match_single_v_card_prefixed_field("SUMMARY", &raw_text);
         let start: String = ::match_single_v_card_prefixed_field("DTSTART", &raw_text);
        if start == null {
            return null;
        }
         let end: String = ::match_single_v_card_prefixed_field("DTEND", &raw_text);
         let duration: String = ::match_single_v_card_prefixed_field("DURATION", &raw_text);
         let location: String = ::match_single_v_card_prefixed_field("LOCATION", &raw_text);
         let organizer: String = ::strip_mailto(&::match_single_v_card_prefixed_field("ORGANIZER", &raw_text));
         let mut attendees: Vec<String> = ::match_v_card_prefixed_field("ATTENDEE", &raw_text);
        if attendees != null {
             {
                 let mut i: i32 = 0;
                while i < attendees.len() {
                    {
                        attendees[i] = ::strip_mailto(attendees[i]);
                    }
                    i += 1;
                 }
             }

        }
         let description: String = ::match_single_v_card_prefixed_field("DESCRIPTION", &raw_text);
         let geo_string: String = ::match_single_v_card_prefixed_field("GEO", &raw_text);
         let mut latitude: f64;
         let mut longitude: f64;
        if geo_string == null {
            latitude = Double::NaN;
            longitude = Double::NaN;
        } else {
             let semicolon: i32 = geo_string.index_of(';');
            if semicolon < 0 {
                return null;
            }
            let tryResult1 = 0;
            'try1: loop {
            {
                latitude = Double::parse_double(&geo_string.substring(0, semicolon));
                longitude = Double::parse_double(&geo_string.substring(semicolon + 1));
            }
            break 'try1
            }
            match tryResult1 {
                 catch ( ignored: &NumberFormatException) {
                    return null;
                }  0 => break
            }

        }
        let tryResult1 = 0;
        'try1: loop {
        {
            return CalendarParsedResult::new(&summary, &start, &end, &duration, &location, &organizer, &attendees, &description, latitude, longitude);
        }
        break 'try1
        }
        match tryResult1 {
             catch ( ignored: &IllegalArgumentException) {
                return null;
            }  0 => break
        }

    }

    fn  match_single_v_card_prefixed_field( prefix: &CharSequence,  raw_text: &String) -> String  {
         let values: List<String> = VCardResultParser::match_single_v_card_prefixed_field(&prefix, &raw_text, true, false);
        return  if values == null || values.is_empty() { null } else { values.get(0) };
    }

    fn  match_v_card_prefixed_field( prefix: &CharSequence,  raw_text: &String) -> Vec<String>  {
         let values: List<List<String>> = VCardResultParser::match_v_card_prefixed_field(&prefix, &raw_text, true, false);
        if values == null || values.is_empty() {
            return null;
        }
         let size: i32 = values.size();
         let mut result: [Option<String>; size] = [None; size];
         {
             let mut i: i32 = 0;
            while i < size {
                {
                    result[i] = values.get(i).get(0);
                }
                i += 1;
             }
         }

        return result;
    }

    fn  strip_mailto( s: &String) -> String  {
        if s != null && (s.starts_with("mailto:") || s.starts_with("MAILTO:")) {
            s = s.substring(7);
        }
        return s;
    }
}


// VINParsedResult.java

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

impl ParsedResult for VINParsedResult {}

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


// VINResultParser.java
/**
 * Detects a result that is likely a vehicle identification number.
 *
 * @author Sean Owen
 */

const IOQ: Pattern = Pattern::compile("[IOQ]");

const AZ09: Pattern = Pattern::compile("[A-Z0-9]{17}");
pub struct VINResultParser {
   super: ResultParser;
}

impl ResultParser for VINResultParser {}

impl VINResultParser {

   pub fn  parse(&self,  result: &Result) -> VINParsedResult  {
       if result.get_barcode_format() != BarcodeFormat::CODE_39 {
           return null;
       }
        let raw_text: String = result.get_text();
       raw_text = IOQ::matcher(&raw_text)::replace_all("")::trim();
       if !AZ09::matcher(&raw_text)::matches() {
           return null;
       }
       let tryResult1 = 0;
       'try1: loop {
       {
           if !::check_checksum(&raw_text) {
               return null;
           }
            let wmi: String = raw_text.substring(0, 3);
           return VINParsedResult::new(&raw_text, &wmi, &raw_text.substring(3, 9), &raw_text.substring(9, 17), &::country_code(&wmi), &raw_text.substring(3, 8), &::model_year(&raw_text.char_at(9)), &raw_text.char_at(10), &raw_text.substring(11));
       }
       break 'try1
       }
       match tryResult1 {
            catch ( iae: &IllegalArgumentException) {
               return null;
           }  0 => break
       }

   }

   fn  check_checksum( vin: &CharSequence) -> bool  {
        let mut sum: i32 = 0;
        {
            let mut i: i32 = 0;
           while i < vin.length() {
               {
                   sum += ::vin_position_weight(i + 1) * ::vin_char_value(&vin.char_at(i));
               }
               i += 1;
            }
        }

        let check_char: char = vin.char_at(8);
        let expected_check_char: char = self.check_char(sum % 11);
       return check_char == expected_check_char;
   }

   fn  vin_char_value( c: char) -> i32  {
       if c >= 'A' && c <= 'I' {
           return (c - 'A') + 1;
       }
       if c >= 'J' && c <= 'R' {
           return (c - 'J') + 1;
       }
       if c >= 'S' && c <= 'Z' {
           return (c - 'S') + 2;
       }
       if c >= '0' && c <= '9' {
           return c - '0';
       }
       throw IllegalArgumentException::new();
   }

   fn  vin_position_weight( position: i32) -> i32  {
       if position >= 1 && position <= 7 {
           return 9 - position;
       }
       if position == 8 {
           return 10;
       }
       if position == 9 {
           return 0;
       }
       if position >= 10 && position <= 17 {
           return 19 - position;
       }
       throw IllegalArgumentException::new();
   }

   fn  check_char( remainder: i32) -> char  {
       if remainder < 10 {
           return ('0' + remainder) as char;
       }
       if remainder == 10 {
           return 'X';
       }
       throw IllegalArgumentException::new();
   }

   fn  model_year( c: char) -> i32  {
       if c >= 'E' && c <= 'H' {
           return (c - 'E') + 1984;
       }
       if c >= 'J' && c <= 'N' {
           return (c - 'J') + 1988;
       }
       if c == 'P' {
           return 1993;
       }
       if c >= 'R' && c <= 'T' {
           return (c - 'R') + 1994;
       }
       if c >= 'V' && c <= 'Y' {
           return (c - 'V') + 1997;
       }
       if c >= '1' && c <= '9' {
           return (c - '1') + 2001;
       }
       if c >= 'A' && c <= 'D' {
           return (c - 'A') + 2010;
       }
       throw IllegalArgumentException::new();
   }

   fn  country_code( wmi: &CharSequence) -> String  {
        let c1: char = wmi.char_at(0);
        let c2: char = wmi.char_at(1);
       match c1 {
             '1' => 
                {
               }
             '4' => 
                {
               }
             '5' => 
                {
                   return "US";
               }
             '2' => 
                {
                   return "CA";
               }
             '3' => 
                {
                   if c2 >= 'A' && c2 <= 'W' {
                       return "MX";
                   }
                   break;
               }
             '9' => 
                {
                   if (c2 >= 'A' && c2 <= 'E') || (c2 >= '3' && c2 <= '9') {
                       return "BR";
                   }
                   break;
               }
             'J' => 
                {
                   if c2 >= 'A' && c2 <= 'T' {
                       return "JP";
                   }
                   break;
               }
             'K' => 
                {
                   if c2 >= 'L' && c2 <= 'R' {
                       return "KO";
                   }
                   break;
               }
             'L' => 
                {
                   return "CN";
               }
             'M' => 
                {
                   if c2 >= 'A' && c2 <= 'E' {
                       return "IN";
                   }
                   break;
               }
             'S' => 
                {
                   if c2 >= 'A' && c2 <= 'M' {
                       return "UK";
                   }
                   if c2 >= 'N' && c2 <= 'T' {
                       return "DE";
                   }
                   break;
               }
             'V' => 
                {
                   if c2 >= 'F' && c2 <= 'R' {
                       return "FR";
                   }
                   if c2 >= 'S' && c2 <= 'W' {
                       return "ES";
                   }
                   break;
               }
             'W' => 
                {
                   return "DE";
               }
             'X' => 
                {
                   if c2 == '0' || (c2 >= '3' && c2 <= '9') {
                       return "RU";
                   }
                   break;
               }
             'Z' => 
                {
                   if c2 >= 'A' && c2 <= 'R' {
                       return "IT";
                   }
                   break;
               }
       }
       return null;
   }
}


// WifiParsedResult.java
/**
 * Represents a parsed result that encodes wifi network information, like SSID and password.
 *
 * @author Vikram Aggarwal
 */
pub struct WifiParsedResult {
    super: ParsedResult;

     let ssid: String;

     let network_encryption: String;

     let password: String;

     let hidden: bool;

     let identity: String;

     let anonymous_identity: String;

     let eap_method: String;

     let phase2_method: String;
}

impl ParsedResult for WifiParsedResult {}

impl WifiParsedResult {

    pub fn new( network_encryption: &String,  ssid: &String,  password: &String) -> WifiParsedResult {
        this(&network_encryption, &ssid, &password, false);
    }

    pub fn new( network_encryption: &String,  ssid: &String,  password: &String,  hidden: bool) -> WifiParsedResult {
        this(&network_encryption, &ssid, &password, hidden, null, null, null, null);
    }

    pub fn new( network_encryption: &String,  ssid: &String,  password: &String,  hidden: bool,  identity: &String,  anonymous_identity: &String,  eap_method: &String,  phase2_method: &String) -> WifiParsedResult {
        super(ParsedResultType::WIFI);
        let .ssid = ssid;
        let .networkEncryption = network_encryption;
        let .password = password;
        let .hidden = hidden;
        let .identity = identity;
        let .anonymousIdentity = anonymous_identity;
        let .eapMethod = eap_method;
        let .phase2Method = phase2_method;
    }

    pub fn  get_ssid(&self) -> String  {
        return self.ssid;
    }

    pub fn  get_network_encryption(&self) -> String  {
        return self.network_encryption;
    }

    pub fn  get_password(&self) -> String  {
        return self.password;
    }

    pub fn  is_hidden(&self) -> bool  {
        return self.hidden;
    }

    pub fn  get_identity(&self) -> String  {
        return self.identity;
    }

    pub fn  get_anonymous_identity(&self) -> String  {
        return self.anonymous_identity;
    }

    pub fn  get_eap_method(&self) -> String  {
        return self.eap_method;
    }

    pub fn  get_phase2_method(&self) -> String  {
        return self.phase2_method;
    }

    pub fn  get_display_result(&self) -> String  {
         let result: StringBuilder = StringBuilder::new(80);
        maybe_append(&self.ssid, &result);
        maybe_append(&self.network_encryption, &result);
        maybe_append(&self.password, &result);
        maybe_append(&Boolean::to_string(self.hidden), &result);
        return result.to_string();
    }
}


// WifiResultParser.java
pub struct WifiResultParser {
    super: ResultParser;
}

impl ResultParser for WifiResultParser{}

impl WifiResultParser {

    pub fn  parse(&self,  result: &Result) -> WifiParsedResult  {
         let raw_text: String = get_massaged_text(result);
        if !raw_text.starts_with("WIFI:") {
            return null;
        }
        raw_text = raw_text.substring(&"WIFI:".length());
         let ssid: String = match_single_prefixed_field("S:", &raw_text, ';', false);
        if ssid == null || ssid.is_empty() {
            return null;
        }
         let pass: String = match_single_prefixed_field("P:", &raw_text, ';', false);
         let mut type: String = match_single_prefixed_field("T:", &raw_text, ';', false);
        if type == null {
            type = "nopass";
        }
        // Unfortunately, in the past, H: was not just used for boolean 'hidden', but 'phase 2 method'.
        // To try to retain backwards compatibility, we set one or the other based on whether the string
        // is 'true' or 'false':
         let mut hidden: bool = false;
         let phase2_method: String = match_single_prefixed_field("PH2:", &raw_text, ';', false);
         let h_value: String = match_single_prefixed_field("H:", &raw_text, ';', false);
        if h_value != null {
            // If PH2 was specified separately, or if the value is clearly boolean, interpret it as 'hidden'
            if phase2_method != null || "true".equals_ignore_case(&h_value) || "false".equals_ignore_case(&h_value) {
                hidden = Boolean::parse_boolean(&h_value);
            } else {
                phase2_method = h_value;
            }
        }
         let identity: String = match_single_prefixed_field("I:", &raw_text, ';', false);
         let anonymous_identity: String = match_single_prefixed_field("A:", &raw_text, ';', false);
         let eap_method: String = match_single_prefixed_field("E:", &raw_text, ';', false);
        return WifiParsedResult::new(&type, &ssid, &pass, hidden, &identity, &anonymous_identity, &eap_method, &phase2_method);
    }
}


// ISBNParsedResult.java
/**
 * Represents a parsed result that encodes a product ISBN number.
 *
 * @author jbreiden@google.com (Jeff Breidenbach)
 */
pub struct ISBNParsedResult {
    super: ParsedResult;

     let isbn: String;
}

impl ParsedResult for ISBNParsedResult{}

impl ISBNParsedResult {

    fn new( isbn: &String) -> ISBNParsedResult {
        super(ParsedResultType::ISBN);
        let .isbn = isbn;
    }

    pub fn  get_i_s_b_n(&self) -> String  {
        return self.isbn;
    }

    pub fn  get_display_result(&self) -> String  {
        return self.isbn;
    }
}


// ISBNResultParser.java
/**
 * Parses strings of digits that represent a ISBN.
 * 
 * @author jbreiden@google.com (Jeff Breidenbach)
 */
pub struct ISBNResultParser {
    super: ResultParser;
}

impl ResultParser for ISBNResultParser {}

impl ISBNResultParser {

    /**
   * See <a href="http://www.bisg.org/isbn-13/for.dummies.html">ISBN-13 For Dummies</a>
   */
    pub fn  parse(&self,  result: &Result) -> ISBNParsedResult  {
         let format: BarcodeFormat = result.get_barcode_format();
        if format != BarcodeFormat::EAN_13 {
            return null;
        }
         let raw_text: String = get_massaged_text(result);
         let length: i32 = raw_text.length();
        if length != 13 {
            return null;
        }
        if !raw_text.starts_with("978") && !raw_text.starts_with("979") {
            return null;
        }
        return ISBNParsedResult::new(&raw_text);
    }
}

