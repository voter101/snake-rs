pub fn manhattan_distance(a: (u16, u16), b: (u16, u16)) -> u16 {
    ((a.0 as i16 - b.0 as i16).abs() + (a.1 as i16 - b.1 as i16).abs()) as u16
}
