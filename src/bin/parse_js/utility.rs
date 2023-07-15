/// `start` should look something like `"var ClassName = function () {"`.
/// The opening `{` is very important!
pub fn get_definition<'a>(start: &'a str, content: &'a str) -> Option<&'a str> {
    let start_index = content.find(start)?;
    let mut brackets = 1;
    let mut end_index = start_index + start.len();
    for c in content.chars().skip(end_index) {
        brackets += match c {
            '{' => 1,
            '}' => -1,
            _ => 0,
        };
        end_index += 1;
        if brackets == 0 {
            break;
        }
    }
    Some(&content[start_index..end_index])
}
