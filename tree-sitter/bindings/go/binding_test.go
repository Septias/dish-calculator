package tree_sitter_menu_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_menu "github.com/tree-sitter/tree-sitter-menu/bindings/go"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_menu.Language())
	if language == nil {
		t.Errorf("Error loading menu grammar")
	}
}
