use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(OneDReader)]
pub fn one_d_reader_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_one_d_reader_macro(&ast)
}

fn impl_one_d_reader_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        use std::collections::HashMap;
        use crate::result_point::ResultPoint;
        use crate::DecodeHintType;
        use crate::DecodingHintDictionary;
        use crate::RXingResultMetadataType;
        use crate::RXingResultMetadataValue;
        use crate::RXingResultPoint;
        use crate::Reader;

        impl Reader for #name {
            fn decode(&mut self, image: &crate::BinaryBitmap) -> Result<crate::RXingResult, Exceptions> {
              self.decode_with_hints(image, &HashMap::new())
            }
        
            // Note that we don't try rotation without the try harder flag, even if rotation was supported.
            fn decode_with_hints(
                &mut self,
                image: &crate::BinaryBitmap,
                hints: &DecodingHintDictionary,
            ) -> Result<crate::RXingResult, Exceptions> {
              if let Ok(res) = self.doDecode(image, hints) {
                 Ok(res)
              }else {
                let tryHarder = hints.contains_key(&DecodeHintType::TRY_HARDER);
                if tryHarder && image.isRotateSupported() {
                  let rotatedImage = image.rotateCounterClockwise();
                  let mut result = self.doDecode(&rotatedImage, hints)?;
                  // Record that we found it rotated 90 degrees CCW / 270 degrees CW
                 let metadata = result.getRXingResultMetadata();
                  let mut orientation = 270;
                  if metadata.contains_key(&RXingResultMetadataType::ORIENTATION) {
                    // But if we found it reversed in doDecode(), add in that result here:
                    orientation = (orientation +
                         if let Some(crate::RXingResultMetadataValue::Orientation(or)) = metadata.get(&RXingResultMetadataType::ORIENTATION) {
                          *or
                         }else {
                          0
                         }) % 360;
                  }
                  result.putMetadata(RXingResultMetadataType::ORIENTATION, RXingResultMetadataValue::Orientation(orientation));
                  // Update result points
                  // let points = result.getRXingResultPoints();
                  // if points != null {
                    let height = rotatedImage.getHeight();
                    // for point in result.getRXingResultPointsMut().iter_mut() {
                      let total_points = result.getRXingResultPoints().len();
                      let points = result.getRXingResultPointsMut();
                      for i in 0..total_points{
                    // for (int i = 0; i < points.length; i++) {
                      points[i] =  RXingResultPoint::new(height as f32- points[i].getY() - 1.0, points[i].getX());
                    }
                  // }
                  
                  Ok(result)
                } else {
                  return Err(Exceptions::NotFoundException("".to_owned()))
                }
              }
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(EANReader)]
pub fn ean_reader_derive(input: TokenStream) -> TokenStream {
  // Construct a representation of Rust code as a syntax tree
  // that we can manipulate
  let ast = syn::parse(input).unwrap();

  // Build the trait implementation
  impl_ean_reader_macro(&ast)
}

fn impl_ean_reader_macro(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let gen = quote! {
    impl super::OneDReader for #name {
      fn decodeRow(
        &mut self,
        rowNumber: u32,
        row: &crate::common::BitArray,
        hints: &crate::DecodingHintDictionary,
    ) -> Result<crate::RXingResult, crate::Exceptions> {
      self.decodeRowWithGuardRange(rowNumber, row, &self.findStartGuardPattern(row)?, hints)
    }
  }
  };
  gen.into()
}

#[proc_macro_derive(OneDWriter)]
pub fn one_d_writer_derive(input: TokenStream) -> TokenStream {
  // Construct a representation of Rust code as a syntax tree
  // that we can manipulate
  let ast = syn::parse(input).unwrap();

  // Build the trait implementation
  impl_one_d_writer_macro(&ast)
}

fn impl_one_d_writer_macro(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let gen = quote! {
    use crate::{
      EncodeHintType, EncodeHintValue, Exceptions, Writer,
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
          self.encode_with_hints(contents, format, width, height, &HashMap::new())
      }
  
      fn encode_with_hints(
          &self,
          contents: &str,
          format: &crate::BarcodeFormat,
          width: i32,
          height: i32,
          hints: &crate::EncodingHintDictionary,
      ) -> Result<crate::common::BitMatrix, crate::Exceptions> {
          if contents.is_empty() {
              return Err(Exceptions::IllegalArgumentException(
                  "Found empty contents".to_owned(),
              ));
          }
  
          if width < 0 || height < 0 {
              return Err(Exceptions::IllegalArgumentException(format!(
                  "Negative size is not allowed. Input: {}x{}",
                  width, height
              )));
          }
          if let Some(supportedFormats) = self.getSupportedWriteFormats() {
              if !supportedFormats.contains(format) {
                  return Err(Exceptions::IllegalArgumentException(format!(
                      "Can only encode {:?}, but got {:?}",
                      supportedFormats, format
                  )));
              }
          }
  
          let mut sidesMargin = self.getDefaultMargin();
          if let Some(EncodeHintValue::Margin(margin)) = hints.get(&EncodeHintType::MARGIN) {
              sidesMargin = margin.parse::<u32>().unwrap();
          }
          // if  hints.contains_key(&EncodeHintType::MARGIN) {
          //   sidesMargin = Integer.parseInt(hints.get(EncodeHintType.MARGIN).toString());
          // }
  
          let code = self.encode_oned_with_hints(contents, hints)?;
  
          Self::renderRXingResult(&code, width, height, sidesMargin)
      }
  }
  };
  gen.into()
}