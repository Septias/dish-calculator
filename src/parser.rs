use pest::Parser;
use pest_derive::Parser;
#[derive(Parser)]
#[grammar = "../pest.grammar"]
struct MdPlanParser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let table = include_str!("../test_data/table.md");
        MdPlanParser::parse(Rule::table, table).unwrap();
    }
}
