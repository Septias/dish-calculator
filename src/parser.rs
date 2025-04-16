peg::parser! {
    grammar mdplan_parser() for str {

    //////////////////////////////
    // Basic Helpers
    //////////////////////////////

    // One or more whitespace characters.
    rule ws() = [' ' | '\t']+
    // Zero or more whitespace characters.
    rule ws_opt() = [' ' | '\t']*

    // Newline.
    rule newline() = "\n"

    // End-of-input marker.
    rule EOI() -> ()
        = ![_]

    //////////////////////////////
    // Top-level Structure
    //////////////////////////////

    // A complete meal plan file.
    // It consists of: People line, date line, header, table, markdown text, then the end-of-input.
    pub rule mdplan_file() -> ()
        = people_line() date_line() essensplan_header() table() newline() { () }

    // People line, e.g.
    rule people_line() -> usize
        = "People: " n:$(number_rule()) newline() {? n.parse().or(Err("Expected number after `People:`")) }
        / expected!("People")

    // A number: one or more digits.
    rule number_rule() -> &'input str
        = n:$(['0'..='9']+) { n }
        / expected!("Number")

    // Date line: any text until a newline.
    rule date_line() -> String
        = "Start: " m:$("['0'..='9']{2}") "-" d:$("['0'..='9']{2}") "-" y:$("['0'..='9']{4}") newline() { d.to_string() }
        / expected!("Date")

    // Essensplan header: the literal "## .*"
    rule essensplan_header() -> ()
        = "## " (!"\n" [_])  newline() { () }

    //////////////////////////////
    // Table Structure
    //////////////////////////////

    // Table: header row, divider row, and one or more meal rows.
    pub rule table() -> ()
        = header_row() divider_row() meal_row()+ { () }

    pub rule header_row() -> Vec<Option<usize>>
        =  s:(!"|\n" "|" a:header() {a})* "|\n" { (s) }
        / expected!("header row")

    pub rule header() -> Option<usize>
        = (a:header_amount() {a}) / (header_no_amount() { None })
        / expected!("header")

    rule header_amount() -> Option<usize>
        = (!"(" [_])  a:amount() (!"|" [_]) { Some(a) }
        / expected!("Header with count")

    rule amount() -> usize
       = "(" n:$(number_rule()) ")" {? n.parse().or(Err("Expected positive number"))}

    rule header_no_amount()
        = (!"|" [_])+
        / expected!("Header no count")

    // Divider row: each cell is made of dashes, colons, or spaces.
    rule divider_row() -> ()
        = "|" divider_cell() ( "|" divider_cell() )+ / ("|" newline()) { () }
    rule divider_cell() -> String
        = s:$( ([' ' | '-' | ':'])+ ) { s.to_string() }

    //////////////////////////////
    // Meal Rows
    //////////////////////////////

    // A meal row starts with a time cell and then zero or more day cells.
    rule meal_row() -> ()
        = "|" time_cell() ( "|" day_cell() )* / ("|" newline()) { () }

    // A time cell is either bold (wrapped in "**") or plain text.
    rule time_cell() -> String
        = bold() / plain_time()

    // Bold text: text wrapped by "**".
    rule bold() -> String
        = "**" s:bold_text() "**" { s }
    rule bold_text() -> String
        = s:$( (!"**" [_])+ ) { s.to_string() }

    // Plain time: any text until a pipe.
    rule plain_time() -> String
        = s:$( (!"|" [_])+ ) { s.to_string() }

    //////////////////////////////
    // Day Cell Content
    //////////////////////////////

    // A day cell may be empty or contain multiple items.
    rule day_cell() -> String
        = s:$(
            ( shopping()
            / meal_ref()
            / html_br()
            / plain_text() )*
          ) { s.to_string() }

    // Shopping marker: exactly "#Einkauf"
    rule shopping() -> ()
        = "#Einkauf" { () }

    // Meal reference: text enclosed in "[[" and "]]".
    rule meal_ref() -> String
        = "[[" s:meal_name() "]]" { s }

    rule meal_name() -> String
        = s:$( (!"]]" [_])+) { s.to_string() }

    // HTML line break, as in "<br>"
    rule html_br() -> ()
        = "<br>" { () }

    // Plain text: any text that does not begin with "<br>", "#Einkauf", or "[[".
    rule plain_text() -> String
        = s:$( (!("<br>" / "#Einkauf" / "[[") [_])+) { s.to_string() }

    //////////////////////////////
    // Markdown Text After the Table
    //////////////////////////////

    // Any additional markdown text until the end-of-input.
    rule markdown_text() -> String
        = s:$((!EOI() [_])*) { s.to_string() }
}}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_table() {
        let table = include_str!("test-data/table.md");
        mdplan_parser::table(table).unwrap();
    }

    #[test]
    fn test_parse_table_header_row() {
        let header = "| Donnerstag | Montag |\n";
        assert_eq!(mdplan_parser::header_row(header).unwrap(), vec![None, None]);
        let header = "| Donnerstag | Montag (12) |\n";
        assert_eq!(
            mdplan_parser::header_row(header).unwrap(),
            vec![None, Some(12)]
        );
    }

    #[test]
    fn test_parse_table_header_cell() {
        let header = "Montag";
        mdplan_parser::header(header).unwrap();
    }
}
