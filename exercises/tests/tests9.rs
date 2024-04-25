extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    fn my_demo_function_alias(a: u32) -> u32;
}

mod foo {
    // No `extern` equals `extern "Rust"`.
    #[no_mangle]
    pub fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    // Import `foo` module to use its functions.
    use super::foo;
    
    #[test]
    fn test_success() {
        // The externally imported functions are UNSAFE by default
        // because of untrusted source of other languages. You may
        // wrap them in safe Rust APIs to ease the burden of callers.
        //
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.
        unsafe {
            // Use functions from the `foo` module.
            assert_eq!(foo::my_demo_function(5), super::my_demo_function(5));
        }
    }
}
