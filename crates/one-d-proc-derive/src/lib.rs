use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(OneDReader)]
pub fn one_d_reader_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the trait implementation
    impl_one_d_reader_macro(&ast)
}

fn impl_one_d_reader_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let gen = quote! {
        impl #impl_generics crate::Reader for #name #ty_generics #where_clause {
            fn decode<B: crate::Binarizer>(&mut self, image: &mut crate::BinaryBitmap<B>) -> Result<crate::RXingResult, Exceptions> {
              self.decode_with_hints(image, &crate::DecodeHints::default())
            }

            // Note that we don't try rotation without the try harder flag, even if rotation was supported.
            fn decode_with_hints<B: crate::Binarizer>(
                &mut self,
                image: &mut crate::BinaryBitmap<B>,
                hints: &crate::DecodeHints,
            ) -> Result<crate::RXingResult, Exceptions> {
                use crate::result_point::ResultPoint;

            if let Ok(res) = self._do_decode(image, hints) {
                Ok(res)
             }else {
               let tryHarder = hints.TryHarder.unwrap_or(false);
               if tryHarder && image.is_rotate_supported() {
                 let mut rotated_image = image.rotate_counter_clockwise();
                 let mut result = self._do_decode(&mut rotated_image, hints)?;
                 // Record that we found it rotated 90 degrees CCW / 270 degrees CW
                 let metadata = result.getRXingResultMetadata();
                 let mut orientation = 270;
                 if metadata.contains_key(&crate::RXingResultMetadataType::ORIENTATION) {
                   // But if we found it reversed in doDecode(), add in that result here:
                   orientation = (orientation +
                        if let Some(crate::RXingResultMetadataValue::Orientation(or)) = metadata.get(&crate::RXingResultMetadataType::ORIENTATION) {
                         *or
                        }else {
                         0
                        }) % 360;
                 }
                 result.putMetadata(crate::RXingResultMetadataType::ORIENTATION, crate::RXingResultMetadataValue::Orientation(orientation));
                 // Update result points
                   let height = rotated_image.get_height();
                     let total_points = result.getRXingResultPoints().len();
                     for point in result.getRXingResultPointsMut()[..total_points].iter_mut() {
                       *point = crate::Point::new(height as f32- point.getY() - 1.0, point.getX());
                     }


                 Ok(result)
               } else {
                 return Err(Exceptions::NOT_FOUND)
               }
             }
            }
        }
    };

    TokenStream::from(gen)
}

#[proc_macro_derive(EANReader)]
pub fn ean_reader_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the trait implementation
    impl_ean_reader_macro(&ast)
}

fn impl_ean_reader_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
      impl super::OneDReader for #name {
        fn decode_row(
          &mut self,
          rowNumber: u32,
          row: &crate::common::BitArray,
          hints: &crate::DecodeHints,
      ) -> Result<crate::RXingResult, crate::Exceptions> {
        self.decodeRowWithGuardRange(rowNumber, row, &self.find_start_guard_pattern(row)?, hints)
      }
    }
    };

    TokenStream::from(gen)
}

#[proc_macro_derive(OneDWriter)]
pub fn one_d_writer_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the trait implementation
    impl_one_d_writer_macro(&ast)
}

fn impl_one_d_writer_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
      use crate::{
        EncodeHints, EncodeHintType, EncodeHintValue, Exceptions, Writer,
     };
     use std::collections::HashMap;
      impl Writer for #name {
        fn encode(
            &self,
            contents: &str,
            format: &crate::BarcodeFormat,
            width: i32,
            height: i32,
        ) -> Result<crate::common::BitMatrix, crate::Exceptions> {
            self.encode_with_hints(contents, format, width, height, &EncodeHints::default())
        }

        fn encode_with_hints(
            &self,
            contents: &str,
            format: &crate::BarcodeFormat,
            width: i32,
            height: i32,
            hints: &EncodeHints,
        ) -> Result<crate::common::BitMatrix, crate::Exceptions> {
            if contents.is_empty() {
                return Err(Exceptions::illegal_argument_with(
                    "Found empty contents"
                ));
            }

            if width < 0 || height < 0 {
                return Err(Exceptions::illegal_argument_with(format!(
                    "Negative size is not allowed. Input: {}x{}",
                    width, height
                )));
            }
            if let Some(supportedFormats) = self.getSupportedWriteFormats() {
                if !supportedFormats.contains(format) {
                    return Err(Exceptions::illegal_argument_with(format!(
                        "Can only encode {:?}, but got {:?}",
                        supportedFormats, format
                    )));
                }
            }

            let mut sidesMargin = self.getDefaultMargin();
            if let Some(margin) = &hints.Margin {
                sidesMargin = margin.parse::<u32>().unwrap();
            }

            let code = self.encode_oned_with_hints(contents, hints)?;

            Self::renderRXingResult(&code, width, height, sidesMargin)
        }
    }
    };

    TokenStream::from(gen)
}
