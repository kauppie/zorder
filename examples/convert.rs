//! Example to show conversions from 3D coordinates to Z-order index and back.
//!
//! The example uses both software and BMI2 hardware implementations, but the BMI2
//! implementation only runs if the hardware supports it.

fn main() {
    let list = [
        [0u8, 0u8, 0u8],
        [1u8, 0u8, 1u8],
        [2u8, 2u8, 2u8],
        [3u8, 16u8, 128u8],
        [255u8, 255u8, 255u8],
    ];

    println!("Software implementation:");
    for coord in list {
        let idx = zorder::index_of(coord);
        let new_coord: [u8; 3] = zorder::coord_of(idx);

        println!("{:?} => {:032b} => {:?}", coord, idx, new_coord);
    }

    if let Some(support_token) = zorder::bmi2::HardwareSupportToken::new() {
        println!();

        println!("BMI2 implementation:");
        for coord in list {
            let idx = zorder::bmi2::index_of(coord, support_token);
            let new_coord: [u8; 3] = zorder::bmi2::coord_of(idx, support_token);

            println!("{:?} => {:032b} => {:?}", coord, idx, new_coord);
        }
    }
}
