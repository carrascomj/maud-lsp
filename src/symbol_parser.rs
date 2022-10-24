/// Parse a line and return the symbol under the cursor
pub fn extract_symbol<'a>(line: &str, column: usize) -> Option<&str> {
    let (before, after) = line.split_at(column);
    if let (Some(start), Some(end)) = (before.chars().rev().position(|c| c == '"'), after.find('"'))
    {
        line.get((column - start)..(column + end))
    } else if let (Some(start), Some(end)) = (
        before
            .chars()
            .rev()
            .position(|c| c == ' ' || c == ',' || c == '{'),
        after.find(|c| c == ' ' || c == ',' || c == '}'),
    ) {
        let words = line.get((column - start)..(column + end))?;
        let mut len_acc = column - start;
        for word in words.split("_") {
            len_acc += word.len();
            if len_acc > column {
                return Some(word);
            }
        }
        None
    } else {
        None
    }
}
