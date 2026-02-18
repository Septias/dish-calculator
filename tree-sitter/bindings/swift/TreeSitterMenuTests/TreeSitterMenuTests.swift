import XCTest
import SwiftTreeSitter
import TreeSitterMenu

final class TreeSitterMenuTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_menu())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading menu grammar")
    }
}
