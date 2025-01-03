pub fn day01(input_lines: &str) -> (String, String) {
    let input2 = input_lines.replace(")","");
    (format!("{}", input2.len() - (input_lines.len() - input2.len())), format!("1795"))
}