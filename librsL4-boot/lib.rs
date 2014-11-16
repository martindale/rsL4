#![crate_name = "rsL4-boot"]
#![crate_type = "rlib"]
#![no_std]
#![feature(lang_items)]

extern crate core;

#[no_mangle]
#[allow(dead_code)]
pub extern fn main() -> ! {
  let thr: * mut u8 = 0x44E09000 as * mut u8;
  loop {
    unsafe {
      *thr = 'a' as u8;
    }
  }
}

// stack_exhausted won't ever be called since we implement our own __morestack. Even so, the
// compiler still expects to find the function.
#[lang = "stack_exhausted"] extern fn stack_exhausted() {}

#[lang = "eh_personality"]  extern fn eh_personality() {}

// This function will only be called if a call to panic! is made.
#[lang = "panic_fmt"]              fn panic_fmt() -> ! { loop {} }

// LLVM checks every function entry to make sure enough stack is allocated, if it isn't this
// function is called. This is normally used in a split stack situation to allocate more stack
// but Rust uses a call to this function to determine when the stack overflows (Rust uses a large
// stack instead of a split-stack). We also disable stack checking on __morestack so we don't try
// calling __morestack from running out of stack while executing __morestack.
#[no_mangle]
#[no_stack_check]
extern fn __morestack() {
  let thr: * mut u8 = 0x44E09000 as * mut u8;
  loop {
    unsafe {
      *thr = 'm' as u8;
    }
  }
}

// These functions are expected by something for stack unwinding.
#[no_mangle]
extern fn __aeabi_unwind_cpp_pr0() {}
#[no_mangle]
extern fn __aeabi_unwind_cpp_pr1() {}
