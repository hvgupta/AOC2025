pub fn part_output<T, U: std::fmt::Debug>(
    function: fn(&T) -> Result<U, ()>,
    part: u8,
    part_input: &T,
) -> String {
    match function(&part_input) {
        Ok(result) => format!("Part {}: {:?}", part, result),
        Err(_) => format!("Error in Part {}", part),
    }
}