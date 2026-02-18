/**
 * @file Menu grammar for tree-sitter
 * @author Sebastian Klähn <info@sebastian-klaehn.de>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "menu",

  rules: {
    // TODO: add the actual grammar rules
    source_file: $ => "hello"
  }
});
