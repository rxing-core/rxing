use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(OneDReader)]
pub fn one_d_reader_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_one_d_reader_macro(&ast)
}

fn impl_one_d_reader_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let gen = quote! {
        impl #impl_generics crate::Reader for #name #ty_generics #where_clause {
            fn decode<B: crate::Binarizer>(&mut self, image: &mut crate::BinaryBitmap<B>) -> Result<crate::RXingResult, crate::Error> {
              self.decode_with_hints(image, &crate::DecodeHints::default())
            }

            // Note that we don't try rotation without the try harder flag, even if rotation was supported.
            fn decode_with_hints<B: crate::Binarizer>(
                &mut self,
                image: &mut crate::BinaryBitmap<B>,
                hints: &crate::DecodeHints,
            ) -> Result<crate::RXingResult, crate::Error> {
                use crate::result_point::ResultPoint;

            if let Ok(res) = self._do_decode(image, hints) {
                Ok(res)
             } else {
               let try_harder = hints.TryHarder.unwrap_or(false);
               if try_harder && image.is_rotate_supported() {
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
                        } else {
                         0
                        }) % 360;
                 }
                 result.putMetadata(crate::RXingResultMetadataType::ORIENTATION, crate::RXingResultMetadataValue::Orientation(orientation));
                 // Update result points
                 let height = rotated_image.get_height();
                 for point in result.getRXingResultPointsMut().iter_mut() {
                   *point = crate::Point::new(height as f32 - point.get_y() - 1.0, point.get_x());
                 }

                 Ok(result)
               } else {
                 Err(crate::Error::NOT_FOUND)
               }
             }
            }
        }
    };

    TokenStream::from(gen)
}

#[proc_macro_derive(EANReader)]
pub fn ean_reader_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_ean_reader_macro(&ast)
}

fn impl_ean_reader_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let gen = quote! {
      impl #impl_generics crate::oned::OneDReader for #name #ty_generics #where_clause {
        fn decode_row(
          &mut self,
          rowNumber: u32,
          row: &crate::common::BitArray,
          hints: &crate::DecodeHints,
      ) -> Result<crate::RXingResult, crate::Error> {
        self.decodeRowWithGuardRange(rowNumber, row, &self.find_start_guard_pattern(row)?, hints)
      }
    }
    };

    TokenStream::from(gen)
}

#[proc_macro_derive(OneDWriter)]
pub fn one_d_writer_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_one_d_writer_macro(&ast)
}

fn impl_one_d_writer_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let gen = quote! {
      impl #impl_generics crate::Writer for #name #ty_generics #where_clause {
        fn encode(
            &self,
            contents: &str,
            format: &crate::BarcodeFormat,
            width: i32,
            height: i32,
        ) -> Result<crate::common::BitMatrix, crate::Error> {
            self.encode_with_hints(contents, format, width, height, &crate::EncodeHints::default())
        }

        fn encode_with_hints(
            &self,
            contents: &str,
            format: &crate::BarcodeFormat,
            width: i32,
            height: i32,
            hints: &crate::EncodeHints,
        ) -> Result<crate::common::BitMatrix, crate::Error> {
            if contents.is_empty() {
                return Err(crate::Error::illegal_argument_with(
                    "Found empty contents"
                ));
            }

            if width < 0 || height < 0 {
                return Err(crate::Error::illegal_argument_with(format!(
                    "Negative size is not allowed. Input: {}x{}",
                    width, height
                )));
            }
            if let Some(supported_formats) = self.getSupportedWriteFormats() {
                if !supported_formats.contains(format) {
                    return Err(crate::Error::illegal_argument_with(format!(
                        "Can only encode {:?}, but got {:?}",
                        supported_formats, format
                    )));
                }
            }

            let mut sides_margin = self.getDefaultMargin();
            if let Some(margin) = &hints.Margin {
                sides_margin = margin.parse::<u32>().map_err(|_| {
                    crate::Error::illegal_argument_with(format!("Invalid margin value: '{}'", margin))
                })?;
            }

            let code = self.encode_oned_with_hints(contents, hints)?;

            Self::renderRXingResult(&code, width, height, sides_margin)
        }
    }
    };

    TokenStream::from(gen)
}
