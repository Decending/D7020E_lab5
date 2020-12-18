// get_sign.rs
// Showcase how we automatically can interface Rust to KLEE
//

#![no_std]
#![no_main]

use klee_sys::klee_make_symbolic;
use panic_klee as _;

fn sum_first_elements(arr: &[u8], index: usize) -> u16 {
    let mut acc: u16 = 0;
    for i in 0..index {
    	if index < arr.len() {
        	acc += arr[i] as u16;
        } else {
        break;
        }
    }
    return acc;
}

#[no_mangle]
fn main() {
    let mut arr = [0u8; 8];
    for i in 0..arr.len() {
    	let mut myVar: u8 = 0;
        klee_make_symbolic!(&mut myVar, "myArrVar");
        arr[i] = myVar;
    }
    let mut i: usize = 0;
    klee_make_symbolic!(&mut i, "i");
    let b = sum_first_elements(&arr, i);
}

// A) Array indexing is tricky to analyse at compile time.
// Thus Rust (rustc) will inject code for run-time verification
// `panic`ing on index out of range.
//
// (Compare to C/C++, where a "buffer overflow" might pass unnoticed
// causing all sorts of problems.)
//
// Compare the test generated in release `--release` (optimized) to
// test generated in debug/dev mode (un-optimized).
//
// Try to explain in your own words the difference and why?
// (Hint, even if we don't use the result `b`, Rust don't optimize out the call, why?)
//
// [My answer here]
// During debug/dev mode we get 10 test cases, 8 for the possible indecies
// 0-7, 1 test case for when index goes out of bounds and one when
// an overflow happens. During --release the two test cases correspond to
// when the index is 0 and one where it is 8.
// Because it is not seen as "inconsequential code" to the behaviour.
//
// B) Fix the code so that you don't get an error.
// (It should still compute the sum of the n first elements
// and return the sum of the whole array if index larger than size/length).
// The fix should be in the function (not on the caller side).
//
// [Git commit "B"]
// Fixed! Index out of bounds error!
//
// C) In the example, the array is holding only zeroes.
// Figure out a way to make the content symbolic.
// (Hint, declare as mutable, iterate and set each element symbolic)
//
// [Git commit "C"]
// Fixed! Create the arr as before, but create variables and insert them in
// the array.
//
//
// D) Analyze the example using KLEE. Now a new (maybe unexpected) error should occur!
//
// Explain what caused the error.
//
// [My answer here]
// It's an overflow error, we need to make sure the sumation of the array
// can't overflow.
//
// E) Make a sensible fix to the code.
// Motivate your choice.
//
// [My answer here]
// By simply increasing the size of the "containers" to u16 we can store a
// value 255 times larger than what can be contained within u8
//
// [Git commit "D"]
// Fixed!
//
// F) Learning outcome.
// 70% of Microsoft security updates over the last decade is directly related to
// memory safety.
//
// Explain in your own words what Microsoft would gain by using Rust.
//
// [My answer here]
// One of the major issues Microsoft is working with in regards to windows
// is memory leaks, which would be handled by rust directly through the
// priority system and the compiler. Integrating rust might yield results.
//
// Explain in your own words what Microsoft would gain by using `cargo
// klee` on their Rust code.
// 
// [My answer here]
// The workflow would be stream lined, as finding errors / incorrect
// behaviours in the code will be easier to find and fix.
//
// And YES, Microsoft is rewriting core system functionality in Rust as we speak!
