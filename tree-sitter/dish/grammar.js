module.exports = grammar({
  name: "dish",

  extras: _ => [/[\t \n]+/],

  rules: {
    source_file: $ =>
      seq(
        $.persons_line,
        repeat($.preamble_line),
        $.ingredients_section,
        optional($.preparation_section),
      ),

    persons_line: $ =>
      seq(
        field("count", $.integer),
        /[\t ]+/,
        choice("Personen", "Portionen")
      ),

    ingredients_section: $ => seq("## Zutaten", repeat1($.ingredient_line)),

    ingredient_line: $ =>
      choice(
        prec(4, seq("-", field("quantity", $.quantity), field("unit", $.unit), field("name", $.text))),
        prec(3, seq("-", field("quantity", $.quantity), field("name", $.text))),
        prec(2, seq("-", field("name", $.text))),
        prec(1,$.preamble_line),
      ),

    preparation_section: $ => seq("## Zubereitung", repeat($.text)),

    // Tokens
    quantity: $ => choice($.float, $.integer),
    integer: _ => /\d+/,
    float: _ => /\d+\.\d+/,
    unit: _ => /[^\s]+/,
    text: _ => /[^\n\r]+/,
    preamble_line: _ => /[^#\-\n\r][^\n\r]*/,
  }
});
