use crate::maud_data::KineticModel;

/// Implementors are able to parse a line and return the symbol under the cursor
pub trait SymbolExtract {
    fn extract_symbol(line: &str, column: usize) -> Option<&str>;
}

impl SymbolExtract for KineticModel<'_> {
    fn extract_symbol<'a>(line: &str, column: usize) -> Option<&str> {
        let (before, after) = line.split_at(column);
        if let (Some(start), Some(end)) =
            (before.chars().rev().position(|c| c == '"'), after.find('"'))
        {
            line.get((column - start)..(column + end))
        } else {
            None
        }
    }
}
