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

    if zorder::bmi2::has_hardware_support() {
        println!();

        println!("BMI2 implementation:");
        for coord in list {
            let idx = unsafe { zorder::bmi2::index_of_unchecked(coord) };
            let new_coord: [u8; 3] = unsafe { zorder::bmi2::coord_of_unchecked(idx) };

            println!("{:?} => {:032b} => {:?}", coord, idx, new_coord);
        }
    }
}