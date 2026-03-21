module.exports = grammar({
  name: "dish",

  // Keep newlines significant; ignore only spaces and tabs
  extras: $ => [/[\t ]/],

  rules: {

    source_file: $ =>
      seq(
        repeat("\n"),
        $.persons_line,
        "\n",
        repeat("\n"),
        $.ingredients_section,
        repeat1("\n"),
        $.preparation_section,
        repeat("\n")
      ),

    persons_line: $ => seq(field("count", $.integer), " Personen"),

    ingredients_section: $ =>
      seq($.ingredients_header, "\n", repeat(seq($.ingredient_line, "\n"))),

    ingredients_header: _ => "## Zutaten",

    ingredient_line: $ =>
      seq(
        "- ",
        field("quantity", $.quantity),
        " ",
        field("unit", $.unit),
        " ",
        field("name", $.ingredient_name)
      ),

    preparation_section: $ =>
      seq($.preparation_header, "\n", repeat(seq($.step_line, "\n"))),

    preparation_header: _ => "## Zubereitung",

    step_line: $ =>
      seq(field("number", $.integer), ". ", field("text", $.text)),

    // Tokens
    quantity: $ => choice($.float, $.integer),
    integer: _ => /\d+/,
    float: _ => /\d+\.\d+/,
    unit: _ => /[^\s]+/,
    ingredient_name: _ => /[^\n]*/,
    text: _ => /[^\n]*/
  }
});
