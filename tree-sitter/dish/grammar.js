module.exports = grammar({
  name: "dish",

  // Keep newlines significant; ignore only spaces and tabs
  extras: _ => [/[\t ]+/],

  rules: {
    source_file: $ =>
      seq(
        repeat("\n"),
        $.persons_line,
        repeat1("\n"),
        repeat(seq($.preamble_line, "\n")),
        $.ingredients_section,
        repeat("\n"),
        optional($.preparation_section),
      ),

    persons_line: $ =>
      seq(
        field("count", $.integer),
        /[\t ]+/,
        choice("Personen", "Portionen")
      ),

    ingredients_section: $ =>
      prec.right(1, seq("## Zutaten", repeat1("\n"), repeat1(seq($.ingredient_line, "\n")))),


    ingredient_line: $ =>
      seq(
        "-",
        /[\t ]+/,
        optional(
          seq(
            field("quantity", $.quantity),
            /[\t ]+/
          )
        ),
        optional(
          seq(
            field("unit", $.unit),
            /[\t ]+/
          )
        ),
        field("name", $.ingredient_name),
        optional(/[\t ]+/)  // Allow trailing spaces
      ),

    preparation_section: $ =>
      prec.right(1, seq("## Zubereitung", repeat1("\n"), repeat(seq($.step_line, "\n")), repeat("\n"))),


    step_line: $ =>
        seq(field("text", $.text))
      ,

    // Lines of free text that must not start with a section header (## ...)
    preamble_line: _ => token(prec(1, /([^#][^\n\r]*|#[^#][^\n\r]*)/)),

    // Tokens
    quantity: $ => choice($.float, $.integer),
    integer: _ => /\d+/,
    float: _ => /\d+\.\d+/,
    unit: _ => /[^\s]+/,
    ingredient_name: _ => /[^\n\r]*/,
    text: _ => /[^\n\r]*/
  }
});
