# Format Variations Found in Rezepte

## Issues to Address in Grammar

### 1. Persons Line Variations
- ✓ `2 Personen` (current grammar supports)
- ✗ `4 Portionen` (kaiserschmarn.md - NOT supported)
- ✗ `4 Portion` (some files have singular)
- ✗ `3 Portionen ` (trailing spaces)
- ✗ Optional header text before persons line (auberginen, brokkoli)

### 2. Whitespace Variations
- ✗ Tabs instead of spaces in ingredient lines
  - Example: `5 EL	Rum, Cognac oder Wasser` (kaiserschmarn.md)
- ✗ Multiple spaces between fields
- ✗ Trailing spaces at end of lines
- ✗ Optional blank lines before `## Zutaten`

### 3. Unit Variations
- `g`, `kg`, `mg` (weight)
- `ml`, `l`, `Liter` (volume)
- `EL`, `el` (tablespoon - case varies)
- `TL`, `tl` (teaspoon - case varies)
- `Stk.`, `stk`, `Stk` (pieces - with/without period)
- `Pck.` (package)
- No unit for items like "Eier" (eggs)

### 4. Ingredient Name Variations
- Simple: `Butter`
- With commas: `Rum, Cognac oder Wasser`
- With special chars: `Zitrone(n), unbehandelt, abgeriebene Schale`
- Multi-word: `gemahlene Leinsamen`

### 5. Number Format
- ✓ Integer: `100`
- ✓ Float: `0.5`
- ? Large numbers with dot separator: `1.6` (kg - could be confused with decimal)

## Test Files Added
- `curry.md` - 16 Personen, lowercase "stk" units
- `brownies.md` - 3 Personen, standard format
- `kaiserschmarn.md` - **4 Portionen** (needs grammar update), tabs in ingredient lines
- `mousse.md` - 36 Personen, "Stk." with period
- `kaesespaetzle.md` - 1 Personen, simple format
- `auberginen.md` - (already existed) header text before persons line
- `brokkoli.md` - (already existed) header text before persons line

## Recommended Grammar Changes

1. Update `persons_line` to accept "Portionen" and "Portion"
2. Make whitespace more flexible (tabs + spaces)
3. Update `extras` to handle more whitespace variations
4. Consider making header text before persons_line optional
5. Make "Personen" vs "Portionen" both acceptable
