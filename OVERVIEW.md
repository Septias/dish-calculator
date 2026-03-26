# Dish Calculator - Project Overview

## What It Does Currently

### Core Functionality
The program is a **meal planning and shopping list generator** for group cooking events. It:

1. **Parses Meal Plans** (`plan.rs`)
   - Reads a weekly menu file in a simple line-based format
   - Extracts dishes for each day with person counts
   - Format: `Day: [[Dish1]], [[Dish2]]`
   - Uses tree-sitter-menu parser for structured parsing

2. **Parses Recipe Files** (`dish.rs`)
   - Reads individual recipe markdown files from a cookbook directory
   - Extracts ingredients with amounts, units, and names
   - Supports flexible formats: "Personen" or "Portionen"
   - Uses tree-sitter-dish parser for reliable parsing

3. **Scales Recipes** (`dish.rs`)
   - Automatically scales ingredient amounts based on:
     - Recipe's base person count
     - Actual number of people to feed
   - Formula: `scaled_amount = original_amount × (target_people / recipe_people)`

4. **Generates Shopping Lists** (`types.rs`)
   - Aggregates ingredients across all dishes in the meal plan
   - Groups by ingredient name and unit
   - Outputs two formats:
     - Simple alphabetical list (`shopping-list.md`)
     - "Clustered" list (currently identical, meant for categorization)



### Example Usage
```bash
cargo run -- --plan gk26.menu --dish-root Rezepte/
# Outputs: shopping-list.md, shopping-list-clustered.md
```

## Areas for Improvement


### 🟡 Important Enhancements

#### 4. Better Error Handling
**Current:** Parse errors crash the program
**Needed:**
- Graceful degradation for missing recipes
- Clear error messages showing which dish failed
- Validation before processing (check all dishes exist)

#### 5. Recipe Format Validation
**Problem:** Some recipes have inconsistent formats that may not parse
**Solution:**
- Add validation command: `cargo run -- --validate Rezepte/`
- Report format issues before they cause runtime errors
- Auto-fix common issues (normalize whitespace, fix units)

#### 6. Unit Normalization
**Problem:** Same units in different formats aren't consolidated:
- `200 g` and `0.2 kg` treated as different units
- `1 EL` and `1 el` might not group correctly

**Solution:**
- Normalize units during parsing (g → kg for large amounts)
- Case-insensitive unit matching
- Unit conversion table

#### 7. Shopping List Improvements
**Current output:**
```markdown
- Butter: 200.0 g (Dish1, Dish2)
- Mehl: 500.0 g (Dish1, Dish3)
```

**Enhancements:**
- Group by category (Dairy, Produce, Dry Goods, etc.)
- Add checkboxes: `- [ ] Butter: 200g`
- Smart rounding (200.0 g → 200 g, 2.5 kg → 2½ kg)
- Highlight specialty items vs. pantry staples
- Warn about unusual amounts (20kg of something?)

#### 9. Shopping Markers Not Used
**Problem:** `⟨Einkauf⟩` markers are parsed but ignored

**Potential uses:**
- Split shopping list into multiple trips
- Mark items to buy fresh vs. in advance
- Generate per-day shopping lists

### 🟢 Nice-to-Have Features

#### 10. PDF Generation
**Note:** `markdown2pdf` dependency exists but unused

**Implementation:**
- Generate formatted PDF shopping lists
- Include meal plan calendar
- Print-friendly formatting

#### 11. Nutritional Information
**Addition:** Track calories, macros per dish
**Output:** Nutritional summary per meal/day

#### 12. Cost Estimation
**Addition:** Price database per ingredient
**Output:** Estimated total shopping cost

#### 13. Dietary Filters
**Addition:** Tag recipes (vegan, vegetarian, gluten-free, etc.)
**Feature:** Filter meal plans by dietary restrictions

#### 14. Inventory Management
**Addition:** Track what's already in pantry
**Feature:** Subtract pantry items from shopping list
**Output:** Only show what needs to be purchased


