use std::collections::HashMap;

pub(crate) type Ingredients = Vec<Ingredient>;

/// A single ingredient
pub(crate) struct Ingredient {
    /// Amount of ingredient.
    pub(crate) amount: f32,
    /// Measure of the ingredient.
    pub(crate) measure: String,
    /// Name of the ingredient.
    pub(crate) name: String,
    /// The dish this ingredient is from.
    pub(crate) dish: String,
}

pub(crate) struct IngredientList(pub(crate) Ingredients);

impl IngredientList {
    pub(crate) fn new() -> Self {
        Self(Vec::new())
    }

    pub(crate) fn from(ingredients: Ingredients) -> Self {
        Self(ingredients)
    }

    /// Accumulate ingredients by name and unit.
    pub(crate) fn accumulate(&mut self) {
        let mut grouped: HashMap<(String, String), Vec<Ingredient>> = HashMap::new();

        for ingredient in self.0.drain(..) {
            let key = (ingredient.name.clone(), ingredient.measure.clone());
            grouped.entry(key).or_insert_with(Vec::new).push(ingredient);
        }

        self.0 = grouped
            .into_iter()
            .map(|((name, measure), ingredients)| {
                let total_amount: f32 = ingredients.iter().map(|i| i.amount).sum();
                let dishes: Vec<String> = ingredients.iter().map(|i| i.dish.clone()).collect();

                Ingredient {
                    amount: total_amount,
                    measure: measure.clone(),
                    name: name.clone(),
                    dish: dishes.join(", "),
                }
            })
            .collect();
    }

    /// Generate md shopping list.
    pub(crate) fn as_md_list(&self) -> String {
        let mut accumulated = self.clone();
        accumulated.accumulate();

        let mut items: Vec<String> = accumulated
            .0
            .iter()
            .map(|ingredient| {
                let amount_str = if ingredient.measure.is_empty() {
                    format!("{:.1}", ingredient.amount)
                } else {
                    format!("{:.1} {}", ingredient.amount, ingredient.measure)
                };

                format!(
                    "- {}: {} ({})",
                    ingredient.name, amount_str, ingredient.dish
                )
            })
            .collect();

        items.sort();
        items.join("\n")
    }

    /// Generate clustered md shopping list with AI.
    pub(crate) fn as_clustered_md_list(&self) -> String {
        // For now, just use the same implementation as as_md_list
        self.as_md_list()
    }
}

impl Clone for Ingredient {
    fn clone(&self) -> Self {
        Self {
            amount: self.amount,
            measure: self.measure.clone(),
            name: self.name.clone(),
            dish: self.dish.clone(),
        }
    }
}

impl Clone for IngredientList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
