// a smart pointer type that reads from its pointer location without needing to copy it first
//Cow stands for copy on write
use std::borrow::Cow;
//is a C-like string type that allows read in zero-terminated strings
use std::ffi::CStr;
// c_char, a type alias for i8 type, presents the possibility of a platform-specific nuances
use std::os::raw::c_char;

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];
fn main() {
    let a = 42;
    //String is a smart pointer that holds a pointer to a backing array and a field to store its size
    let b : String;
    //accepts a type parameter for the data it points to; str is the type returned by CStr.to_string_lossy(),
    // so it is appropriate here
    let c: Cow<str>; 

    unsafe {
        //refs can't cast directly to *mut T, the type required by String::from_raw_parts()
        //But *const T can be cast to *mut T, leading to this double cast syntax 
        let b_ptr = &B as *const u8 as *mut u8;
        // String::from_raw_parts accepts a pointer (*mut T)
        // to an array of bytes, a size, and a capacity parameter 
         b = String::from_raw_parts(b_ptr, 10, 10);

        //Converts a *const u8 to a *const i8, aliased to c_char.
        //The conversion to i8 works because we remain under 128, following the ASCii standard.
        let c_ptr = &C as *const u8 as *const c_char;

        //takes reponsability for reading the pointer until it reaches 0;
        // then it generates Cow<str>from the result;
        c = CStr::from_ptr(c_ptr).to_string_lossy();
    }

    println!("a: {}, b: {}, c: {}", a, b, c);
}
