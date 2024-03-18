pub fn manhattan_distance(a: (u8, u8), b: (u8, u8)) -> u8 {
    ((a.0 as i16 - b.0 as i16).abs() + (a.1 as i16 - b.1 as i16).abs()) as u8
}
