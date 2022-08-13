use crate::{DecodeHintType,NotFoundException,ReaderException,ResultPointCallback};
use crate::common::{BitMatrix,DetectorResult};
use crate::qrcode::detector::{Detector,FinderPatternInfo,FinderPattern,FinderPatternFinder};

// MultiDetector.java
/**
 * <p>Encapsulates logic that can detect one or more QR Codes in an image, even if the QR Code
 * is rotated or skewed, or partially obscured.</p>
 *
 * @author Sean Owen
 * @author Hannes Erven
 */

const EMPTY_DETECTOR_RESULTS: [Option<DetectorResult>; 0] = [None; 0];
pub struct MultiDetector {
    super: Detector;
}

impl Detector for MultiDetector{}

impl MultiDetector {

    pub fn new( image: &BitMatrix) -> MultiDetector {
        super(image);
    }

    pub fn  detect_multi(&self,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException */Result<Vec<DetectorResult>, Rc<Exception>>   {
         let image: BitMatrix = get_image();
         let result_point_callback: ResultPointCallback =  if hints == null { null } else { hints.get(DecodeHintType::NEED_RESULT_POINT_CALLBACK) as ResultPointCallback };
         let finder: MultiFinderPatternFinder = MultiFinderPatternFinder::new(image, result_point_callback);
         let infos: Vec<FinderPatternInfo> = finder.find_multi(&hints);
        if infos.len() == 0 {
            throw NotFoundException::get_not_found_instance();
        }
         let result: List<DetectorResult> = ArrayList<>::new();
        for  let info: FinderPatternInfo in infos {
            let tryResult1 = 0;
            'try1: loop {
            {
                result.add(&process_finder_pattern_info(info));
            }
            break 'try1
            }
            match tryResult1 {
                 catch ( e: &ReaderException) {
                }  0 => break
            }

        }
        if result.is_empty() {
            return Ok(EMPTY_DETECTOR_RESULTS);
        } else {
            return Ok(result.to_array(EMPTY_DETECTOR_RESULTS));
        }
    }
}

// MultiFinderPatternFinder.java
/**
 * <p>This class attempts to find finder patterns in a QR Code. Finder patterns are the square
 * markers at three corners of a QR Code.</p>
 *
 * <p>This class is thread-safe but not reentrant. Each thread must allocate its own object.
 *
 * <p>In contrast to {@link FinderPatternFinder}, this class will return an array of all possible
 * QR code locations in the image.</p>
 *
 * <p>Use the TRY_HARDER hint to ask for a more thorough detection.</p>
 *
 * @author Sean Owen
 * @author Hannes Erven
 */

const EMPTY_RESULT_ARRAY: [Option<FinderPatternInfo>; 0] = [None; 0];

const EMPTY_FP_ARRAY: [Option<FinderPattern>; 0] = [None; 0];

const EMPTY_FP_2D_ARRAY: [Option<FinderPattern>; 0] = [None; 0];

// TODO MIN_MODULE_COUNT and MAX_MODULE_COUNT would be great hints to ask the user for
// since it limits the number of regions to decode
// max. legal count of modules per QR code edge (177)
const MAX_MODULE_COUNT_PER_EDGE: f32 = 180;

// min. legal count per modules per QR code edge (11)
const MIN_MODULE_COUNT_PER_EDGE: f32 = 9;

/**
  * More or less arbitrary cutoff point for determining if two finder patterns might belong
  * to the same code if they differ less than DIFF_MODSIZE_CUTOFF_PERCENT percent in their
  * estimated modules sizes.
  */
const DIFF_MODSIZE_CUTOFF_PERCENT: f32 = 0.05f;

/**
  * More or less arbitrary cutoff point for determining if two finder patterns might belong
  * to the same code if they differ less than DIFF_MODSIZE_CUTOFF pixels/module in their
  * estimated modules sizes.
  */
const DIFF_MODSIZE_CUTOFF: f32 = 0.5f;
pub struct MultiFinderPatternFinder {
   super: FinderPatternFinder;
}

impl FinderPatternFinder for MultiFinderPatternFinder {}

impl MultiFinderPatternFinder {

   /**
  * A comparator that orders FinderPatterns by their estimated module size.
  */
   #[derive(Comparator<FinderPattern>, Serializable)]
   struct ModuleSizeComparator {
   }
   
   impl ModuleSizeComparator {

       pub fn  compare(&self,  center1: &FinderPattern,  center2: &FinderPattern) -> i32  {
            let value: f32 = center2.get_estimated_module_size() - center1.get_estimated_module_size();
           return  if value < 0.0 { -1 } else {  if value > 0.0 { 1 } else { 0 } };
       }
   }


   pub fn new( image: &BitMatrix,  result_point_callback: &ResultPointCallback) -> MultiFinderPatternFinder {
       super(image, result_point_callback);
   }

   /**
  * @return the 3 best {@link FinderPattern}s from our list of candidates. The "best" are
  *         those that have been detected at least 2 times, and whose module
  *         size differs from the average among those patterns the least
  * @throws NotFoundException if 3 such finder patterns do not exist
  */
   fn  select_multiple_best_patterns(&self) -> /*  throws NotFoundException */Result<Vec<Vec<FinderPattern>>, Rc<Exception>>   {
        let possible_centers: List<FinderPattern> = ArrayList<>::new();
       for  let fp: FinderPattern in get_possible_centers() {
           if fp.get_count() >= 2 {
               possible_centers.add(fp);
           }
       }
        let size: i32 = possible_centers.size();
       if size < 3 {
           // Couldn't find enough finder patterns
           throw NotFoundException::get_not_found_instance();
       }
       /*
    * Begin HE modifications to safely detect multiple codes of equal size
    */
       if size == 3 {
           return Ok( : vec![FinderPattern; 1] = vec![possible_centers.to_array(EMPTY_FP_ARRAY), ]
           );
       }
       // Sort by estimated module size to speed up the upcoming checks
       Collections::sort(&possible_centers, ModuleSizeComparator::new());
       /*
    * Now lets start: build a list of tuples of three finder locations that
    *  - feature similar module sizes
    *  - are placed in a distance so the estimated module count is within the QR specification
    *  - have similar distance between upper left/right and left top/bottom finder patterns
    *  - form a triangle with 90Â° angle (checked by comparing top right/bottom left distance
    *    with pythagoras)
    *
    * Note: we allow each point to be used for more than one code region: this might seem
    * counterintuitive at first, but the performance penalty is not that big. At this point,
    * we cannot make a good quality decision whether the three finders actually represent
    * a QR code, or are just by chance laid out so it looks like there might be a QR code there.
    * So, if the layout seems right, lets have the decoder try to decode.
    */
       // holder for the results
        let results: List<Vec<FinderPattern>> = ArrayList<>::new();
        {
            let mut i1: i32 = 0;
           while i1 < (size - 2) {
               {
                    let p1: FinderPattern = possible_centers.get(i1);
                   if p1 == null {
                       continue;
                   }
                    {
                        let mut i2: i32 = i1 + 1;
                       while i2 < (size - 1) {
                           {
                                let p2: FinderPattern = possible_centers.get(i2);
                               if p2 == null {
                                   continue;
                               }
                               // Compare the expected module sizes; if they are really off, skip
                                let v_mod_size12: f32 = (p1.get_estimated_module_size() - p2.get_estimated_module_size()) / Math::min(&p1.get_estimated_module_size(), &p2.get_estimated_module_size());
                                let v_mod_size12_a: f32 = Math::abs(p1.get_estimated_module_size() - p2.get_estimated_module_size());
                               if v_mod_size12_a > DIFF_MODSIZE_CUTOFF && v_mod_size12 >= DIFF_MODSIZE_CUTOFF_PERCENT {
                                   // any more interesting elements for the given p1.
                                   break;
                               }
                                {
                                    let mut i3: i32 = i2 + 1;
                                   while i3 < size {
                                       {
                                            let p3: FinderPattern = possible_centers.get(i3);
                                           if p3 == null {
                                               continue;
                                           }
                                           // Compare the expected module sizes; if they are really off, skip
                                            let v_mod_size23: f32 = (p2.get_estimated_module_size() - p3.get_estimated_module_size()) / Math::min(&p2.get_estimated_module_size(), &p3.get_estimated_module_size());
                                            let v_mod_size23_a: f32 = Math::abs(p2.get_estimated_module_size() - p3.get_estimated_module_size());
                                           if v_mod_size23_a > DIFF_MODSIZE_CUTOFF && v_mod_size23 >= DIFF_MODSIZE_CUTOFF_PERCENT {
                                               // any more interesting elements for the given p1.
                                               break;
                                           }
                                            let test: vec![Vec<FinderPattern>; 3] = vec![p1, p2, p3, ]
                                           ;
                                           ResultPoint::order_best_patterns(test);
                                           // Calculate the distances: a = topleft-bottomleft, b=topleft-topright, c = diagonal
                                            let info: FinderPatternInfo = FinderPatternInfo::new(test);
                                            let d_a: f32 = ResultPoint::distance(&info.get_top_left(), &info.get_bottom_left());
                                            let d_c: f32 = ResultPoint::distance(&info.get_top_right(), &info.get_bottom_left());
                                            let d_b: f32 = ResultPoint::distance(&info.get_top_left(), &info.get_top_right());
                                           // Check the sizes
                                            let estimated_module_count: f32 = (d_a + d_b) / (p1.get_estimated_module_size() * 2.0f);
                                           if estimated_module_count > MAX_MODULE_COUNT_PER_EDGE || estimated_module_count < MIN_MODULE_COUNT_PER_EDGE {
                                               continue;
                                           }
                                           // Calculate the difference of the edge lengths in percent
                                            let v_a_b_b_c: f32 = Math::abs((d_a - d_b) / Math::min(d_a, d_b));
                                           if v_a_b_b_c >= 0.1f {
                                               continue;
                                           }
                                           // Calculate the diagonal length by assuming a 90Â° angle at topleft
                                            let d_cpy: f32 = Math::sqrt(d_a as f64 * d_a + d_b as f64 * d_b) as f32;
                                           // Compare to the real distance in %
                                            let v_py_c: f32 = Math::abs((d_c - d_cpy) / Math::min(d_c, d_cpy));
                                           if v_py_c >= 0.1f {
                                               continue;
                                           }
                                           // All tests passed!
                                           results.add(test);
                                       }
                                       i3 += 1;
                                    }
                                }

                           }
                           i2 += 1;
                        }
                    }

               }
               i1 += 1;
            }
        }

       if !results.is_empty() {
           return Ok(results.to_array(EMPTY_FP_2D_ARRAY));
       }
       // Nothing found!
       throw NotFoundException::get_not_found_instance();
   }

   pub fn  find_multi(&self,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException */Result<Vec<FinderPatternInfo>, Rc<Exception>>   {
        let try_harder: bool = hints != null && hints.contains_key(DecodeHintType::TRY_HARDER);
        let image: BitMatrix = get_image();
        let max_i: i32 = image.get_height();
        let max_j: i32 = image.get_width();
       // We are looking for black/white/black/white/black modules in
       // 1:1:3:1:1 ratio; this tracks the number of such modules seen so far
       // Let's assume that the maximum version QR Code we support takes up 1/4 the height of the
       // image, and then account for the center being 3 modules in size. This gives the smallest
       // number of pixels the center could be, so skip this often. When trying harder, look for all
       // QR versions regardless of how dense they are.
        let i_skip: i32 = (3 * max_i) / (4 * MAX_MODULES);
       if i_skip < MIN_SKIP || try_harder {
           i_skip = MIN_SKIP;
       }
        let state_count: [i32; 5] = [0; 5];
        {
            let mut i: i32 = i_skip - 1;
           while i < max_i {
               {
                   // Get a row of black/white values
                   do_clear_counts(&state_count);
                    let current_state: i32 = 0;
                    {
                        let mut j: i32 = 0;
                       while j < max_j {
                           {
                               if image.get(j, i) {
                                   // Black pixel
                                   if (current_state & 1) == 1 {
                                       // Counting white pixels
                                       current_state += 1;
                                   }
                                   state_count[current_state] += 1;
                               } else {
                                   // White pixel
                                   if (current_state & 1) == 0 {
                                       // Counting black pixels
                                       if current_state == 4 {
                                           // A winner?
                                           if found_pattern_cross(&state_count) && handle_possible_center(&state_count, i, j) {
                                               // Yes
                                               // Clear state to start looking again
                                               current_state = 0;
                                               do_clear_counts(&state_count);
                                           } else {
                                               // No, shift counts back by two
                                               do_shift_counts2(&state_count);
                                               current_state = 3;
                                           }
                                       } else {
                                           state_count[current_state += 1] += 1;
                                       }
                                   } else {
                                       // Counting white pixels
                                       state_count[current_state] += 1;
                                   }
                               }
                           }
                           j += 1;
                        }
                    }

                   if found_pattern_cross(&state_count) {
                       handle_possible_center(&state_count, i, max_j);
                   }
               }
               i += i_skip;
            }
        }

       // for i=iSkip-1 ...
        let pattern_info: Vec<Vec<FinderPattern>> = self.select_multiple_best_patterns();
        let result: List<FinderPatternInfo> = ArrayList<>::new();
       for  let pattern: Vec<FinderPattern> in pattern_info {
           ResultPoint::order_best_patterns(pattern);
           result.add(FinderPatternInfo::new(pattern));
       }
       if result.is_empty() {
           return Ok(EMPTY_RESULT_ARRAY);
       } else {
           return Ok(result.to_array(EMPTY_RESULT_ARRAY));
       }
   }
}

