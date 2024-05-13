// extern crate opencv;
// use opencv::core::Rect;

use std::ffi::{CStr, CString};
fn main() {
    println!("Hello, world!");
}

// // write main structs

// // /**
// //  * @struct DetectReadResult
// //  * @brief Represents the result of a combined text detection and reading operation.
// //  *
// //  * @details This structure is used to store the result of operations where text is both detected and read,
// //  * storing the text and its location within the image.
// //  */
// #[repr(C)]
// pub struct DetectReadResult {
//     //   /**
//     //    * @brief The text detected and read from the image.
//     //    */
//     pub text_: std::ffi::CString,

//     //   /**
//     //    * @brief The bounding box of the detected and read text within the image.
//     //    */
//     pub bounding_box_: Rect,
// }

// // write functions

// #[no_mangle]
// pub extern "C" fn detect_read_text(
//     image: &Mat,
//     model_path: *const libc::c_char,
//     display: bool,
//     results: *mut DetectReadResult,
//     results_len: *mut libc::size_t,
// ) -> libc::c_int {
//     let model_path = unsafe {
//         if model_path.is_null() {
//             CString::new("frozen_east_text_detection.pb").unwrap()
//         } else {
//             CStr::from_ptr(model_path).to_owned()
//         }
//     };

//     let detection_results = DetectReadText(image, model_path.to_str().unwrap(), display);

//     let results_slice = unsafe { std::slice::from_raw_parts_mut(results, *results_len as usize) };
//     for (i, result) in detection_results.into_iter().enumerate() {
//         if i >= *results_len as usize {
//             break;
//         }
//         results_slice[i] = DetectReadResult {
//             text: CString::new(result.text_).unwrap(),
//             bounding_box: result.bounding_box_,
//         };
//     }

//     unsafe {
//         *results_len = detection_results.len() as libc::size_t;
//     }

//     0 // Return success
// }
