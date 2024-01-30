#![cfg(feature = "image")]

/**
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*/
use rxing::{oned::Code39Reader, BarcodeFormat, MultiFormatReader};

mod common;

/**
 * @author Sean Owen
 */
#[cfg(feature = "image_formats")]
#[test]
fn code39_black_box1_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/code39-1",
        MultiFormatReader::default(),
        BarcodeFormat::CODE_39,
    );

    // 2 failes on row 169

    // super("src/test/resources/blackbox/code39-1", new MultiFormatReader(), BarcodeFormat.CODE_39);
    tester.add_test(4, 4, 0.0);
    tester.add_test(4, 4, 180.0);

    tester.test_black_box();
}

/**
 * @author dswitkin@google.com (Daniel Switkin)
 */
#[cfg(feature = "image_formats")]
#[test]
fn code39_black_box3_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/code39-3",
        MultiFormatReader::default(),
        BarcodeFormat::CODE_39,
    );

    // super("src/test/resources/blackbox/code39-3", new MultiFormatReader(), BarcodeFormat.CODE_39);
    tester.add_test(17, 17, 0.0);
    tester.add_test(17, 17, 180.0);

    tester.test_black_box();
}

/**
 * @author Sean Owen
 */
#[cfg(feature = "image_formats")]
#[test]
fn code39_extended_black_box2_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/code39-2",
        Code39Reader::with_all_config(false, true),
        BarcodeFormat::CODE_39,
    );

    // super("src/test/resources/blackbox/code39-2", new Code39Reader(false, true), BarcodeFormat.CODE_39);
    tester.add_test(2, 2, 0.0);
    tester.add_test(2, 2, 180.0);

    tester.test_black_box();
}
