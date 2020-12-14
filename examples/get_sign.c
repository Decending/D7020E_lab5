/*
 * First KLEE tutorial: testing a small function
 * https://klee.github.io/tutorials/testing-function/
 */

#include <klee/klee.h>

int get_sign(int x)
{
    if (x == 0)
        return 0;

    if (x < 0)
        return -1;
    else
        return 1;
}

int main()
{
    int a;
    klee_make_symbolic(&a, sizeof(a), "a");
    return get_sign(a);
}

// A) Compiling into LLVM bitcode
// > clang -emit-llvm -c get_sign.c
//
// Now you can run Klee on your generated bitcode.
//
// > klee get_sign.bc
//
// [My answer here]
// KLEE: output directory is "/home/kalle/Desktop/klee_tutorial-master/examples/klee-out-1"
// KLEE: Using Z3 solver backend
// KLEE: done: total instructions = 31
// KLEE: done: completed paths = 3
// KLEE: done: generated tests = 3
//
// B) Inspecting the output
//
// > ls klee-last/
//
// [My answer here]
// assembly.ll  messages.txt  run.stats  test000002.ktest  warnings.txt
// info         run.istats    test000001.ktest  test000003.ktest
//
// C) Inspecting the generated test cases
//
// > ktest-tool klee-last/test000001.ktest
//
// What path in the code does this test represent?
//
// [My answer here]
// This test represents the path taken when x = 0.
//
// > ktest-tool klee-last/test000002.ktest
//
// What path in the code does this test represent?
//
// [My answer here]
// This test represents the path taken when x < 0
// the int (255) represents -1
//
// > ktest-tool klee-last/test000003.ktest
//
// What path in the code does this test represent?
//
// [My answer here]
// This test represents the path taken when x > 0
// 
// D) Replaying a test case
//
// Fist check that includes were installed:
// > ls /usr/local/include
// klee
//
// > ls /usr/local/lib
// klee  libkleeRuntest.so  libkleeRuntest.so.1.0
//
// If those are ok, then you can compile for replay:
//
// > clang -I /usr/local/include/ -L /usr/local/lib get_sign.c -l kleeRuntest
//
// To replay the first test:
//
// We need to add the libary path so it can be dynamically loaded:
// Depending on shell this might look different:
//
// Under `bash` (and `bash` like shells)
// > export LD_LIBRARY_PATH=/usr/local/lib/:$LD_LIBRARY_PATH
//
// Under `fish`
// > set -x LD_LIBRARY_PATH /usr/local/lib/:$LD_LIBRARY_PATH
//
// > KTEST_FILE=klee-last/test000001.ktest ./a.out
//
// Now let's inspect the status (return code), in `bash`:
// $? is the return value (error code) as seen by the shell.
//
// > echo $?
//
// In `fish` you would do
//
// > echo $status
//
// Did the result correspond to the expected path for the test?
//
// [My answer here]
// Expected result: 0
// Actual result: 0
// Perfect match!
//
// > KTEST_FILE=klee-last/test000002.ktest ./a.out
//
// Inspect the return code:
//
// Did the result correspond to the expected path for the test?
//
// [My answer here]
// Expected result: 1
// Actual result: 1
// Perfect match!
//
// > KTEST_FILE=klee-last/test000003.ktest ./a.out
//
// Inspect the return code:
//
// Did the result correspond to the expected path for the test?
//
// [My answer here]
// Yes, as the returned result (255) in binary is 11111111
// This is interpreted as an unsigned 8 bit int, as opposed to an 8 bit 
// signed int.
// 2s complement of 255 is -1, which is the expected result.
//
// Why not? Confir to shell error codes:
//
// [My answer here]
// We got the expected output, just interpreted as an unsigned int as
// opposed to a signed one.
//
// D) Debugging
//
// In the above example its kind of hard to see exactly
// what happens. Using `gdb` you single step the program.
//
// First build it with debug symbols (`-g`).
// > clang -g -I /usr/local/include/ -L /usr/local/lib get_sign.c -l kleeRuntest
//
// Then start `gdb`:
// > KTEST_FILE=klee-last/test000001.ktest gdb ./a.out
// (gdb) break get_sign
//
// (gdb) run
//
// Now we can inspect the `x` argument by:
// (gdb) print x
//
// What value do you get, and why?
//
// [My answer here]
// We get x = 0, as it is the value for x during test 1.
//
// Step the code
// > (gdb) next
//
// What path did it take, and why?
//
// [My answer here]
// It took the path corresponding to x = 0, because x = 0 is being
// interpreted as 0.
//
// Now we can try with another test:
//
// (gdb) set environment KTEST_FILE=klee-last/test000002.ktest
//
// And (re-start) the debug session:
// (gdb) run
//
// Step through the code.
//
// Which path did it take, and why?
//
// [My answer here]
// The path corresponding to x > 0, as x = -1 (11111111) is being 
// interpreted as 255.
//
// And finally:
//
// (gdb) set environment KTEST_FILE=klee-last/test000003.ktest
//
// Which path did it take, and why?
//
// [My answer here]
// The path taken corresponded to x < 0, as to why I can't answer yet.
//
// E) Under the hood.
//
// Explain in your own words how
// `klee_make_symbolic(&a, sizeof(a), "a");`
// works when you run `klee` to generate test cases:
//
// [My answer here]
// A symbolic variable "a" is created, it has no value and instead works
// with constraints which are added to "a" when the path branch.
// (hint, mark memory region as symbolic)
//
// Explain in your own words how
// `klee_make_symbolic(&a, sizeof(a), "a");`
// works when you replay test cases:
//
// [My answer here]
// The variable "a" is created, with the constraints loaded from the
// beginning, and a value is generated for "a" to satisfy the constraints
// and hence the path.
// (hint, KTEST_FILE points to a concrete assignment
// of the memory region)
