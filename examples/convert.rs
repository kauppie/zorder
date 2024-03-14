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

    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("bmi2") {
            println!();

            println!("BMI2 implementation:");
            for coord in list {
                let idx = unsafe { zorder::bmi2::index_of(coord) };
                let new_coord: [u8; 3] = unsafe { zorder::bmi2::coord_of(idx) };

                println!("{:?} => {:032b} => {:?}", coord, idx, new_coord);
            }
        }
    }
}
