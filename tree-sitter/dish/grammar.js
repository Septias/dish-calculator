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
         seq("-", field("quantity", $.quantity), field("unit", $.unit), field("name", $.text)),
         seq("-", field("quantity", $.quantity), field("name", $.text)),
         seq("-", field("name", $.text)),
         $.preamble_line,
      ),

    preparation_section: $ => seq("## Zubereitung", repeat($.text)),

    // Tokens
    quantity: $ => choice($.float, $.integer),
    integer: _ => token(prec(2,/\d+/)),
    float: _ => token(prec(2,/\d+\.\d+/)),
    unit: _ => token(prec(3,/[^\s]+/)),
    text: _ => /[^\n\r]+/,
    preamble_line: _ => /[^#\-\n\r][^\n\r]*/,
  }
});
