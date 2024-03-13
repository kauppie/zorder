//! This example can be used to validate if the current CPU supports the BMI2 instruction set.
//!
//! # Example
//!
//! ```sh
//! cargo run --example bmi2_support
//! ```

fn main() {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("bmi2") {
            println!("bmi2 is supported");
        } else {
            println!("bmi2 is not supported");
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        println!("Not x86_64 arch; bmi2 is not supported")
    }
}
