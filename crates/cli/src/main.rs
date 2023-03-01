use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use clap::{ArgGroup, Parser, Subcommand};
use rxing::{BarcodeFormat, MultiFormatWriter, Writer};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    file_name: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(group(
        ArgGroup::new("advanced_display_group")
        .required(false)
        .args(["detailed_results","parsed_results","raw_bytes"]),
    ))]
    Decode {
        /// Try much harder to detect barcodes.
        #[arg(short, long)]
        try_harder: bool,

        /// Search for multiple barcodes in an image instead of just one, this can be much slower.
        #[arg(short, long)]
        decode_multi: bool,

        /// Can be specified multiple times with different barcode formats, only listed formats are searched for.
        #[arg(short, long, value_enum)]
        barcode_types: Option<Vec<BarcodeFormat>>,

        /// Print detailed results data
        #[arg(long)]
        detailed_results: bool,

        /// Print parsed results (exclusive with --detailed-results and --raw-bytes)
        #[arg(long)]
        parsed_results: bool,

        /// Print raw bytes (exclusive with --detailed-results and --raw-bytes)
        #[arg(long)]
        raw_bytes: bool,

        /// Unspecified, application-specific hint.
        #[arg(long)]
        other: Option<String>,

        /// Image is a pure monochrome image of a barcode.
        #[arg(long)]
        pure_barcode: Option<bool>,

        /// Specifies what character encoding to use when decoding, where applicable.
        #[arg(long)]
        character_set: Option<String>,

        /// Allowed lengths of encoded data -- reject anything else..
        #[arg(long)]
        allowed_lengths: Option<Vec<u32>>,

        /// Assume Code 39 codes employ a check digit.
        #[arg(long)]
        assume_code_39_check_digit: Option<bool>,

        /// Assume the barcode is being processed as a GS1 barcode, and modify behavior as needed.
        /// For example this affects FNC1 handling for Code 128 (aka GS1-128).
        #[arg(long, verbatim_doc_comment)]
        assume_gs1: Option<bool>,

        /// If true, return the start and end digits in a Codabar barcode instead of stripping them. They
        /// are alpha, whereas the rest are numeric. By default, they are stripped, but this causes them
        /// to not be.
        #[arg(long, verbatim_doc_comment)]
        return_codabar_start_end: Option<bool>,

        /// Allowed extension lengths for EAN or UPC barcodes. Other formats will ignore this.
        /// Maps to an {@code int[]} of the allowed extension lengths, for example [2], [5], or [2, 5].
        /// If it is optional to have an extension, do not set this hint. If this is set,
        /// and a UPC or EAN barcode is found but an extension is not, then no result will be returned
        /// at all.
        #[arg(long, verbatim_doc_comment)]
        allowed_ean_extensions: Option<Vec<u32>>,

        /// If true, also tries to decode as inverted image. All configured decoders are simply called a
        /// second time with an inverted image.
        #[arg(long, verbatim_doc_comment)]
        also_inverted: Option<bool>,
    },
    #[command(group(
        ArgGroup::new("code_set_rules")
        .required(false)
        .args(["code_128_compact", "force_code_set"]),
    ))]
    #[command(group(
        ArgGroup::new("data_source")
        .required(true)
        .args(["data", "data_file"]),
    ))]
    #[command(group(
        ArgGroup::new("data_matrix_encoding")
        .required(false)
        .args(["data_matrix_compact","force_c40"]),
    ))]
    Encode {
        barcode_type: BarcodeFormat,
        #[arg(long)]
        width: u32,
        #[arg(long)]
        height: u32,

        /// String input for the encoder.
        #[arg(short, long)]
        data: Option<String>,

        /// A file containing the text to be encoded.
        #[arg(long)]
        data_file: Option<PathBuf>,

        /// Specifies what degree of error correction to use, for example in QR Codes.
        /// Type depends on the encoder. For example for QR codes it's (L,M,Q,H).
        /// For Aztec it is of type u32, representing the minimal percentage of error correction words.
        /// For PDF417 it is of type u8, valid values being 0 to 8.
        /// Note: an Aztec symbol should have a minimum of 25% EC words.
        #[arg(long, verbatim_doc_comment)]
        error_correction: Option<String>,

        /// Specifies what character encoding to use where applicable.
        #[arg(long)]
        character_set: Option<String>,

        /// Specifies whether to use compact mode for Data Matrix.
        /// The compact encoding mode also supports the encoding of characters that are not in the ISO-8859-1
        /// character set via ECIs.
        /// Please note that in that case, the most compact character encoding is chosen for characters in
        /// the input that are not in the ISO-8859-1 character set. Based on experience, some scanners do not
        /// support encodings like cp-1256 (Arabic). In such cases the encoding can be forced to UTF-8 by
        /// means of the #CHARACTER_SET encoding hint.
        /// Compact encoding also provides GS1-FNC1 support when #GS1_FORMAT is selected. In this case
        /// group-separator character (ASCII 29 decimal) can be used to encode the positions of FNC1 codewords
        /// for the purpose of delimiting AIs.
        #[arg(long, verbatim_doc_comment)]
        data_matrix_compact: Option<bool>,

        /// Specifies margin, in pixels, to use when generating the barcode.
        /// The meaning can vary
        /// by format; for example it controls margin before and after the barcode horizontally for
        /// most 1D formats.
        #[arg(long, verbatim_doc_comment)]
        margin: Option<String>,

        /**
         Specifies whether to use compact mode for PDF417.
        */
        #[arg(long)]
        pdf_417_compact: Option<bool>,

        /**
         Specifies what compaction mode to use for PDF417
         AUTO = 0,
         TEXT = 1,
         BYTE = 2,
         NUMERIC = 3
        */
        #[arg(long)]
        pdf_417_compaction: Option<String>,

        /// Specifies whether to automatically insert ECIs when encoding PDF417.
        /// Please note that in that case, the most compact character encoding is chosen for characters in
        /// the input that are not in the ISO-8859-1 character set. Based on experience, some scanners do not
        /// support encodings like cp-1256 (Arabic). In such cases the encoding can be forced to UTF-8 by
        /// means of the #CHARACTER_SET encoding hint.
        #[arg(long, verbatim_doc_comment)]
        pdf_417_auto_eci: Option<bool>,

        /// Specifies the required number of layers for an Aztec code.
        /// A negative number (-1, -2, -3, -4) specifies a compact Aztec code.
        /// 0 indicates to use the minimum number of layers (the default).
        /// A positive number (1, 2, .. 32) specifies a normal (non-compact) Aztec code.
        #[arg(long, verbatim_doc_comment)]
        aztec_layers: Option<i32>,

        /**
         Specifies the exact version of QR code to be encoded.
        */
        #[arg(long)]
        qr_version: Option<String>,

        /// Specifies the QR code mask pattern to be used. Allowed values are
        /// 0..8. By default the code will automatically select
        /// the optimal mask pattern.
        #[arg(long, verbatim_doc_comment)]
        qr_mask_pattern: Option<String>,

        /// Specifies whether to use compact mode for QR code.
        /// Please note that when compaction is performed, the most compact character encoding is chosen
        /// for characters in the input that are not in the ISO-8859-1 character set. Based on experience,
        /// some scanners do not support encodings like cp-1256 (Arabic). In such cases the encoding can
        /// be forced to UTF-8 by means of the #CHARACTER_SET encoding hint.
        #[arg(long, verbatim_doc_comment)]
        qr_compact: Option<bool>,

        /**
         Specifies whether the data should be encoded to the GS1 standard/
        */
        #[arg(long)]
        gs1_format: Option<bool>,

        /// Forces which encoding will be used. Currently only used for Code-128 code sets.
        /// Valid values are "A", "B", "C".
        #[arg(long, verbatim_doc_comment)]
        force_code_set: Option<String>,

        /**
         Forces C40 encoding for data-matrix. This
        */
        #[arg(long)]
        force_c40: Option<bool>,

        /**
         Specifies whether to use compact mode for Code-128 code.
         This can yield slightly smaller bar codes.
        */
        #[arg(long)]
        code_128_compact: Option<bool>,
    },
}

fn main() {
    let cli = Args::parse();
    match &cli.command {
        Commands::Decode {
            try_harder,
            decode_multi,
            barcode_types,
            other,
            pure_barcode,
            character_set,
            allowed_lengths,
            assume_code_39_check_digit,
            assume_gs1,
            return_codabar_start_end,
            allowed_ean_extensions,
            also_inverted,
            detailed_results,
            parsed_results,
            raw_bytes,
        } => decode_command(
            &cli.file_name,
            try_harder,
            decode_multi,
            barcode_types,
            other,
            pure_barcode,
            character_set,
            allowed_lengths,
            assume_code_39_check_digit,
            assume_gs1,
            return_codabar_start_end,
            allowed_ean_extensions,
            also_inverted,
            detailed_results,
            parsed_results,
            raw_bytes,
        ),
        Commands::Encode {
            barcode_type,
            width,
            height,
            data,
            data_file,
            error_correction,
            character_set,
            data_matrix_compact,
            margin,
            pdf_417_compact,
            pdf_417_compaction,
            pdf_417_auto_eci,
            aztec_layers,
            qr_version,
            qr_mask_pattern,
            qr_compact,
            gs1_format,
            force_code_set,
            force_c40,
            code_128_compact,
        } => encode_command(
            &cli.file_name,
            barcode_type,
            width,
            height,
            data,
            data_file,
            error_correction,
            character_set,
            data_matrix_compact,
            margin,
            pdf_417_compact,
            pdf_417_compaction,
            pdf_417_auto_eci,
            aztec_layers,
            qr_version,
            qr_mask_pattern,
            qr_compact,
            gs1_format,
            force_code_set,
            force_c40,
            code_128_compact,
        ),
    }
}

fn decode_command(
    file_name: &str,
    try_harder: &bool,
    decode_multi: &bool,
    barcode_types: &Option<Vec<BarcodeFormat>>,
    other: &Option<String>,
    pure_barcode: &Option<bool>,
    character_set: &Option<String>,
    allowed_lengths: &Option<Vec<u32>>,
    assume_code_39_check_digit: &Option<bool>,
    assume_gs1: &Option<bool>,
    return_codabar_start_end: &Option<bool>,
    allowed_ean_extensions: &Option<Vec<u32>>,
    also_inverted: &Option<bool>,
    detailed_result: &bool,
    parsed_bytes: &bool,
    raw_bytes: &bool,
) {
    let mut hints: rxing::DecodingHintDictionary = HashMap::new();
    if let Some(other) = other {
        hints.insert(
            rxing::DecodeHintType::OTHER,
            rxing::DecodeHintValue::Other(other.to_owned()),
        );
    }
    if let Some(pure_barcode) = pure_barcode {
        hints.insert(
            rxing::DecodeHintType::PURE_BARCODE,
            rxing::DecodeHintValue::PureBarcode(*pure_barcode),
        );
    }
    if let Some(character_set) = character_set {
        hints.insert(
            rxing::DecodeHintType::CHARACTER_SET,
            rxing::DecodeHintValue::CharacterSet(character_set.to_owned()),
        );
    }
    if let Some(allowed_lengths) = allowed_lengths {
        hints.insert(
            rxing::DecodeHintType::ALLOWED_LENGTHS,
            rxing::DecodeHintValue::AllowedLengths(allowed_lengths.to_vec()),
        );
    }
    if let Some(assume_code_39_check_digit) = assume_code_39_check_digit {
        hints.insert(
            rxing::DecodeHintType::ASSUME_CODE_39_CHECK_DIGIT,
            rxing::DecodeHintValue::AssumeCode39CheckDigit(*assume_code_39_check_digit),
        );
    }
    if let Some(assume_gs1) = assume_gs1 {
        hints.insert(
            rxing::DecodeHintType::ASSUME_GS1,
            rxing::DecodeHintValue::AssumeGs1(*assume_gs1),
        );
    }
    if let Some(return_codabar_start_end) = return_codabar_start_end {
        hints.insert(
            rxing::DecodeHintType::RETURN_CODABAR_START_END,
            rxing::DecodeHintValue::ReturnCodabarStartEnd(*return_codabar_start_end),
        );
    }
    if let Some(allowed_ean_extensions) = allowed_ean_extensions {
        hints.insert(
            rxing::DecodeHintType::ALLOWED_EAN_EXTENSIONS,
            rxing::DecodeHintValue::AllowedEanExtensions(allowed_ean_extensions.to_vec()),
        );
    }
    if let Some(also_inverted) = also_inverted {
        hints.insert(
            rxing::DecodeHintType::ALSO_INVERTED,
            rxing::DecodeHintValue::AlsoInverted(*also_inverted),
        );
    }

    // println!(
    //     "Decode '{}' with: try_harder: {}, decode_multi: {}, barcode_types: {:?}",
    //     file_name, try_harder, decode_multi, barcode_types
    // );

    if !try_harder {
        hints.insert(
            rxing::DecodeHintType::TRY_HARDER,
            rxing::DecodeHintValue::TryHarder(false),
        );
    }
    if let Some(barcode_type) = barcode_types {
        hints.insert(
            rxing::DecodeHintType::POSSIBLE_FORMATS,
            rxing::DecodeHintValue::PossibleFormats(HashSet::from_iter(
                barcode_type.iter().copied(),
            )),
        );
    }

    let path = PathBuf::from(file_name);
    let extension = if let Some(ext) = path.extension() {
        ext.to_string_lossy().to_string()
    } else {
        String::default()
    };

    if *decode_multi {
        let results = if extension == "svg" {
            rxing::helpers::detect_multiple_in_svg_with_hints(file_name, &mut hints)
        } else {
            rxing::helpers::detect_multiple_in_file_with_hints(file_name, &mut hints)
        };
        match results {
            Ok(result_array) => {
                println!("Found {} results", result_array.len());
                for (i, result) in result_array.into_iter().enumerate() {
                    println!(
                        "Result {}:\n{}",
                        i,
                        print_result(&result, *detailed_result, *raw_bytes, *parsed_bytes)
                    );
                }
            }
            Err(search_err) => {
                println!(
                    "Error while attempting to locate multiple barcodes in '{file_name}': {search_err}"
                );
            }
        }
    } else {
        let result = if extension == "svg" {
            rxing::helpers::detect_in_svg_with_hints(file_name, None, &mut hints)
        } else {
            rxing::helpers::detect_in_file_with_hints(file_name, None, &mut hints)
        };
        match result {
            Ok(result) => {
                println!(
                    "Detection result: \n{}",
                    print_result(&result, *detailed_result, *raw_bytes, *parsed_bytes)
                );
            }
            Err(search_err) => {
                println!(
                    "Error while attempting to locate barcode in '{file_name}': {search_err}"
                );
            }
        }
    }
}

fn encode_command(
    file_name: &str,
    barcode_type: &BarcodeFormat,
    width: &u32,
    height: &u32,
    data: &Option<String>,
    data_file: &Option<PathBuf>,
    error_correction: &Option<String>,
    character_set: &Option<String>,
    data_matrix_compact: &Option<bool>,
    margin: &Option<String>,
    pdf_417_compact: &Option<bool>,
    pdf_417_compaction: &Option<String>,
    pdf_417_auto_eci: &Option<bool>,
    aztec_layers: &Option<i32>,
    qr_version: &Option<String>,
    qr_mask_pattern: &Option<String>,
    qr_compact: &Option<bool>,
    gs1_format: &Option<bool>,
    force_code_set: &Option<String>,
    force_c40: &Option<bool>,
    code_128_compact: &Option<bool>,
) {
    // if data.is_none() && data_file.is_none() {
    //     println!("must provide either data string or data file");
    //     return;
    // }
    // if data.is_some() && data_file.is_some() {
    //     println!("provide only data string or data file");
    //     return;
    // }

    let input_data = if let Some(path_from) = data_file {
        if path_from.exists() {
            let Ok(fl) = std::fs::File::open(path_from) else {
                println!("file cannot be opened");
                return;
            };
            std::io::read_to_string(fl).expect("file should read")
        } else {
            println!("{} does not exist", path_from.to_string_lossy());
            return;
        }
    } else if let Some(ds) = data {
        ds.to_owned()
    } else {
        println!("Unknown error getting data");
        return;
    };

    let mut hints: rxing::EncodingHintDictionary = HashMap::new();

    if let Some(ec) = error_correction {
        hints.insert(
            rxing::EncodeHintType::ERROR_CORRECTION,
            rxing::EncodeHintValue::ErrorCorrection(ec.to_owned()),
        );
    }

    if let Some(character_set) = character_set {
        hints.insert(
            rxing::EncodeHintType::CHARACTER_SET,
            rxing::EncodeHintValue::CharacterSet(character_set.to_owned()),
        );
    }

    if let Some(data_matrix_compact) = data_matrix_compact {
        hints.insert(
            rxing::EncodeHintType::DATA_MATRIX_COMPACT,
            rxing::EncodeHintValue::DataMatrixCompact(*data_matrix_compact),
        );
    }

    if let Some(margin) = margin {
        hints.insert(
            rxing::EncodeHintType::MARGIN,
            rxing::EncodeHintValue::Margin(margin.to_owned()),
        );
    }

    if let Some(pdf_417_compact) = pdf_417_compact {
        hints.insert(
            rxing::EncodeHintType::PDF417_COMPACT,
            rxing::EncodeHintValue::Pdf417Compact(pdf_417_compact.to_string()),
        );
    }

    if let Some(pdf_417_compaction) = pdf_417_compaction {
        hints.insert(
            rxing::EncodeHintType::PDF417_COMPACTION,
            rxing::EncodeHintValue::Pdf417Compaction(pdf_417_compaction.to_owned()),
        );
    }

    if let Some(pdf_417_auto_eci) = pdf_417_auto_eci {
        hints.insert(
            rxing::EncodeHintType::PDF417_AUTO_ECI,
            rxing::EncodeHintValue::Pdf417AutoEci(pdf_417_auto_eci.to_string()),
        );
    }

    if let Some(aztec_layers) = aztec_layers {
        hints.insert(
            rxing::EncodeHintType::AZTEC_LAYERS,
            rxing::EncodeHintValue::AztecLayers(*aztec_layers),
        );
    }

    if let Some(qr_version) = qr_version {
        hints.insert(
            rxing::EncodeHintType::QR_VERSION,
            rxing::EncodeHintValue::QrVersion(qr_version.to_owned()),
        );
    }

    if let Some(qr_mask_pattern) = qr_mask_pattern {
        hints.insert(
            rxing::EncodeHintType::QR_MASK_PATTERN,
            rxing::EncodeHintValue::QrMaskPattern(qr_mask_pattern.to_owned()),
        );
    }

    if let Some(qr_compact) = qr_compact {
        println!("Warning, QRCompact can generate unreadable barcodes");
        hints.insert(
            rxing::EncodeHintType::QR_COMPACT,
            rxing::EncodeHintValue::QrCompact(qr_compact.to_string()),
        );
    }

    if let Some(gs1_format) = gs1_format {
        hints.insert(
            rxing::EncodeHintType::GS1_FORMAT,
            rxing::EncodeHintValue::Gs1Format(*gs1_format),
        );
    }

    if let Some(force_code_set) = force_code_set {
        hints.insert(
            rxing::EncodeHintType::FORCE_CODE_SET,
            rxing::EncodeHintValue::ForceCodeSet(force_code_set.to_owned()),
        );
    }

    if let Some(force_c40) = force_c40 {
        hints.insert(
            rxing::EncodeHintType::FORCE_C40,
            rxing::EncodeHintValue::ForceC40(*force_c40),
        );
    }

    if let Some(code_128_compact) = code_128_compact {
        hints.insert(
            rxing::EncodeHintType::CODE128_COMPACT,
            rxing::EncodeHintValue::Code128Compact(*code_128_compact),
        );
    }

    // println!("Encode: file_name: {}, barcode_type: {}, width: {:?}, height: {:?}, data: '{:?}', data_file: {:?}", file_name, barcode_type, width, height, data, data_file);

    let writer = MultiFormatWriter::default();
    match writer.encode_with_hints(
        &input_data,
        barcode_type,
        *width as i32,
        *height as i32,
        &hints,
    ) {
        Ok(result) => {
            println!("Encode successful, saving...");
            match rxing::helpers::save_file(file_name, &result) {
                Ok(_) => println!("Saved to '{file_name}'"),
                Err(error) => println!("Could not save '{file_name}': {error}"),
            }
        }
        Err(encode_error) => println!("Couldn't encode: {encode_error}"),
    }
}

fn print_result(result: &rxing::RXingResult, detailed: bool, raw: bool, parsed: bool) -> String {
    let result_data = result.getText().escape_default().collect::<String>();
    if detailed {
        format!("[Barcode Format] {}\n[Metadata] {:?}\n[Points] {:?}\n[Number of Bits] {}\n[Timestamp] {}\n[Data] {}", result.getBarcodeFormat(),result.getRXingResultMetadata(), result.getRXingResultPoints(), result.getNumBits(), result.getTimestamp(), result_data)
    } else if raw {
        result
            .getRawBytes()
            .iter()
            .fold(String::from(""), |acc, b| acc + " " + &b.to_string())
    } else if parsed {
        let parsed_data = rxing::client::result::parseRXingResult(result);
        parsed_data.to_string()
    } else {
        format!("({}) {}", result.getBarcodeFormat(), result_data)
    }
}
