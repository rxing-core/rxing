use std::{fmt, rc::Rc};

use super::{MinimalECIInput, ECIEncoderSet, ECIInput, COST_PER_ECI};

pub struct InputEdge {
    pub c: String,
    pub encoderIndex: usize, //the encoding of this edge
    pub previous: Option<Rc<InputEdge>>,
    pub cachedTotalSize: usize,
}
impl InputEdge {
    pub fn new(
        c: &str,
        encoderSet: &ECIEncoderSet,
        encoderIndex: usize,
        previous: Option<Rc<InputEdge>>,
        fnc1: u16,
    ) -> Self {
        let mut size = if c == "\u{1000}" {
            1
        } else {
            encoderSet.encode_char(c, encoderIndex).len()
        };

        let fnc1Str = String::from_utf16(&[fnc1]).unwrap();

        if let Some(prev) = previous {
            let previousEncoderIndex = prev.encoderIndex;
            if previousEncoderIndex != encoderIndex {
                size += COST_PER_ECI;
            }
            size += prev.cachedTotalSize;

            Self {
                c: if c == fnc1Str {
                    String::from("\u{1000}")
                } else {
                    String::from(c)
                },
                encoderIndex,
                previous: Some(prev.clone()),
                cachedTotalSize: size,
            }
        } else {
            let previousEncoderIndex = 0;
            if previousEncoderIndex != encoderIndex {
                size += COST_PER_ECI;
            }

            Self {
                c: if c == fnc1Str {
                    String::from("\u{1000}")
                } else {
                    String::from(c)
                },
                encoderIndex,
                previous: None,
                cachedTotalSize: size,
            }
        }

        //   int size = this.c == 1000 ? 1 : encoderSet.encode(c, encoderIndex).length;
        // let previousEncoderIndex = if previous.is_none() {
        //     0
        // } else {
        //     previous.unwrap().encoderIndex
        // };
        //   int previousEncoderIndex = previous == null ? 0 : previous.encoderIndex;
        // if previousEncoderIndex != encoderIndex {
        //     size += COST_PER_ECI;
        // }
        // if prev_is_some {
        //     size += previous.unwrap().cachedTotalSize;
        // }

        // Self {
        //     c: if c == fnc1 { 1000 as char } else { c },
        //     encoderIndex,
        //     previous: previous,
        //     cachedTotalSize: size,
        // }
        //   this.c = c == fnc1 ? 1000 : c;
        //   this.encoderIndex = encoderIndex;
        //   this.previous = previous;
        //   this.cachedTotalSize = size;
    }

    pub fn isFNC1(&self) -> bool {
        self.c == "\u{1000}"
    }
}

impl fmt::Display for MinimalECIInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for i in 0..self.length() {
            // for (int i = 0; i < length(); i++) {
            if i > 0 {
                result.push_str(", ");
            }
            if self.isECI(i as u32).unwrap() {
                result.push_str("ECI(");
                result.push_str(&self.getECIValue(i).unwrap().to_string());
                result.push(')');
            } else if (self.charAt(i).unwrap() as u8) < 128 {
                result.push('\'');
                result.push(self.charAt(i).unwrap());
                result.push('\'');
            } else {
                result.push(self.charAt(i).unwrap());
            }
        }
        write!(f, "{}", result)
    }
}