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

    //////////////////////////////
    // Top-level Structure
    //////////////////////////////

    // A complete meal plan file.
    pub rule mdplan_file() -> ()
        = people_line() date_line() essensplan_header() table() { () }

    // People line, e.g. "12 People"
    rule people_line() -> ()
        = n:$(number_rule()) ws() "People" newline() { () }

    // A number: one or more digits.
    rule number_rule() -> &'input str
        = n:$(['0'..='9']+) { n }

    // Date line: any text until a newline.
    rule date_line() -> String
        = d:$( (!newline() [_])+ ) newline() { d.to_string() }

    // Essensplan header: the literal "## Essensplan"
    rule essensplan_header() -> ()
        = "## Essensplan" newline() { () }

    //////////////////////////////
    // Table Structure
    //////////////////////////////

    // Table: header row, divider row, and one or more meal rows.
    pub rule table() -> ()
        = header_row() divider_row() meal_row()+ { () }

    // Header row: starts with a cell then one or more day header cells.
    rule header_row() -> ()
        = "|" header_cell() ( "|" day_header() )+ "|" newline() { () }
    // Any text until a pipe.
    rule header_cell() -> String
        = s:$( (!"|" [_])* ) { s.to_string() }

    // Day header: a day name that may optionally be followed by a people count in parentheses.
    rule day_header() -> (String, Option<String>)
        = d:day_name() ws_opt() opt:(
              "(" ws_opt() n:$(number_rule()) ws_opt() ")" { n }
          )? {
              (d, opt.map(|s| s.to_string()))
          }
    // Day name: any text until an opening parenthesis.
    rule day_name() -> String
        = s:$( (!"(" [_])+ ) { s.to_string() }

    // Divider row: each cell is made of dashes, colons or spaces.
    rule divider_row() -> ()
        = "|" divider_cell() ( "|" divider_cell() )+ "|" newline() { () }
    rule divider_cell() -> String
        = s:$( ([' ' | '-' | ':'])+ ) { s.to_string() }

    //////////////////////////////
    // Meal Rows
    //////////////////////////////

    // A meal row starts with a time cell and then zero or more day cells.
    rule meal_row() -> ()
        = "|" time_cell() ( "|" day_cell() )* "|" newline() { () }

    // A time cell is either bold (enclosed in "**") or plain text.
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
        = s:$((!("<br>" / "#Einkauf" / "[[") [_])+) { s.to_string() }
}}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let table = include_str!("test-data/table.md");
        mdplan_parser::table(table).unwrap();
    }
}
