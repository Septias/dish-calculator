# Grammar Updates Summary

## Changes Made to grammar.js

### 1. Support Multiple Person/Portion Formats
**Before:**
```javascript
persons_line: $ => seq(field("count", $.integer), " Personen"),
```

**After:**
```javascript
persons_line: $ =>
  seq(
    field("count", $.integer),
    /[\t ]+/,
    choice("Personen", "Portionen", "Portion")
  ),
```

Now supports:
- `4 Personen` ✓
- `4 Portionen` ✓
- `4 Portion` ✓

### 2. Flexible Whitespace Handling
**Before:**
```javascript
extras: $ => [/[\t ]/],  // Single space or tab
```

**After:**
```javascript
extras: $ => [/[\t ]+/],  // One or more spaces/tabs
```

### 3. Tab Support in Ingredient Lines
**Before:**
```javascript
ingredient_line: $ =>
  seq(
    "- ",
    field("quantity", $.quantity),
    " ",
    field("unit", $.unit),
    " ",
    field("name", $.ingredient_name)
  ),
```

**After:**
```javascript
ingredient_line: $ =>
  seq(
    "-",
    /[\t ]+/,
    field("quantity", $.quantity),
    /[\t ]+/,
    field("unit", $.unit),
    /[\t ]+/,
    field("name", $.ingredient_name)
  ),
```

Now handles:
- `- 100 g Butter` (spaces) ✓
- `- 5	EL	Rum` (tabs) ✓
- `- 100  g  Butter` (multiple spaces) ✓

## Test Coverage

### New Tests Added (src/dish.rs)
1. **test_parse_portionen_format** - Verifies "Portionen" format works
2. **test_parse_with_tabs** - Verifies tab-separated ingredients work

### All Tests Passing (10/10) ✓
1. test_parse_simple_dish
2. test_parse_with_scaling
3. test_parse_with_float_quantity
4. test_parse_auberginen_corpus
5. test_parse_invalid_format_fails
6. test_parse_missing_persons_line
7. test_parse_empty_ingredients_section
8. test_scaling_with_fractional_people
9. **test_parse_portionen_format** (new)
10. **test_parse_with_tabs** (new)

## Test Corpus Files Added

From the Rezepte directory:
- ✓ `curry.md` - 16 Personen
- ✓ `brownies.md` - 3 Personen
- ✓ `kaiserschmarn.md` - **4 Portionen** (tests new format)
- ✓ `mousse.md` - 36 Personen
- ✓ `kaesespaetzle.md` - 1 Personen

Existing:
- ✓ `auberginen.md`
- ✓ `brokkoli.md`

## Remaining Format Variations (Not Yet Handled)

These variations exist in the Rezepte but don't cause parsing errors:

1. **Unit variations** - Grammar accepts any non-whitespace as unit:
   - `stk`, `Stk.`, `Stk` (all work)
   - `EL`, `el` (both work)
   - Case-insensitive matching not needed for parsing

2. **Ingredient names with special characters**:
   - Commas: `Rum, Cognac oder Wasser` ✓ (works)
   - Parentheses: `Zitrone(n)` ✓ (works)
   - All characters except newline accepted ✓

3. **Header text before persons_line**:
   - Currently not supported in grammar
   - Not needed for GK26 recipes
   - Can be added if needed

## Impact

All recipes from the GK26 menu that exist in the Rezepte directory can now be parsed correctly!

**Found dishes:** 18/25
**Missing dishes:** 7 (Gebratene Maultaschen, Bohneneintopf mit Kartoffeln, Hotdogs, Brunch, Kuchenbuffet, Wraps mit Gyros, Abschlussbuffet)
