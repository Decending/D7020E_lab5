// get_sign.rs
// Showcase how we automatically can interface Rust to KLEE
//

#![no_std]
#![no_main]

use klee_sys::klee_make_symbolic;
use panic_klee as _;

fn sum_first_elements(arr: &[u8], index: usize) -> u8 {
    let mut acc = 0;
    if index > arr.len() {
    	for i in 0..index {
       	acc += &arr[i];
    	}
    }
    else{
    for i in 0..arr.len() {
    	acc += &arr[i];
    	}
    }
    return acc;
}

#[no_mangle]
fn main() {
    let mut arr = [0u8; 8];
    for i in 0..arr.len() {
       klee_make_symbolic![&mut arr[i], "arr[i]"];
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
// Because it is not seen as "inconsequential code" to the behaviour.
// As to what "inconsequential code" is... I've been unable to find a sufficiently
// well put together explaination of what it is exactly.
//
// B) Fix the code so that you don't get an error.
// (It should still compute the sum of the n first elements
// and return the sum of the whole array if index larger than size/length).
// The fix should be in the function (not on the caller side).
//
// [Git commit "B"]
// Still an error... Can't find the source as of yet. Fixed errors
// regarding indexing.
//
// C) In the example, the array is holding only zeroes.
// Figure out a way to make the content symbolic.
// (Hint, declare as mutable, iterate and set each element symbolic)
//
// [Git commit "C"]
// Fixed! 
//
//
// D) Analyze the example using KLEE. Now a new (maybe unexpected) error should occur!
//
// Explain what caused the error.
//
// [your answer here]
//
// E) Make a sensible fix to the code.
// Motivate your choice.
//
// [your answer here]
//
// [Git commit "D"]
//
// F) Learning outcome.
// 70% of Microsoft security updates over the last decade is directly related to
// memory safety.
//
// Explain in your own words what Microsoft would gain by using Rust.
//
// [your answer here]
//
// Explain in your own words what Microsoft would gain by using `cargo klee`
// on their Rust code.
//
// And YES, Microsoft is rewriting core system functionality in Rust as we speak!
