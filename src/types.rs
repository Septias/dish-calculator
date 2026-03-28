use std::collections::HashMap;

pub(crate) type Ingredients = Vec<Ingredient>;

/// A single ingredient
#[derive(Debug)]
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
    pub(crate) fn as_md_list(&mut self) -> String {
        self.accumulate();

        let mut items: Vec<String> = self
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
    pub(crate) fn as_clustered_md_list(&mut self) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accumulate_same_ingredient_same_measure() {
        let mut list = IngredientList::from(vec![
            Ingredient {
                amount: 100.0,
                measure: "g".to_string(),
                name: "flour".to_string(),
                dish: "Dish A".to_string(),
            },
            Ingredient {
                amount: 200.0,
                measure: "g".to_string(),
                name: "flour".to_string(),
                dish: "Dish B".to_string(),
            },
            Ingredient {
                amount: 50.0,
                measure: "g".to_string(),
                name: "flour".to_string(),
                dish: "Dish C".to_string(),
            },
        ]);

        list.accumulate();

        assert_eq!(list.0.len(), 1);
        assert_eq!(list.0[0].amount, 350.0);
        assert_eq!(list.0[0].measure, "g");
        assert_eq!(list.0[0].name, "flour");
        assert!(list.0[0].dish.contains("Dish A"));
        assert!(list.0[0].dish.contains("Dish B"));
        assert!(list.0[0].dish.contains("Dish C"));
    }

    #[test]
    fn test_accumulate_different_measures_kept_separate() {
        let mut list = IngredientList::from(vec![
            Ingredient {
                amount: 2.0,
                measure: "cups".to_string(),
                name: "sugar".to_string(),
                dish: "Dish A".to_string(),
            },
            Ingredient {
                amount: 100.0,
                measure: "g".to_string(),
                name: "sugar".to_string(),
                dish: "Dish B".to_string(),
            },
            Ingredient {
                amount: 1.0,
                measure: "cups".to_string(),
                name: "sugar".to_string(),
                dish: "Dish C".to_string(),
            },
        ]);

        list.accumulate();

        assert_eq!(list.0.len(), 2);

        let cups_ingredient = list.0.iter().find(|i| i.measure == "cups").unwrap();
        assert_eq!(cups_ingredient.amount, 3.0);
        assert_eq!(cups_ingredient.name, "sugar");
        assert!(cups_ingredient.dish.contains("Dish A"));
        assert!(cups_ingredient.dish.contains("Dish C"));

        let grams_ingredient = list.0.iter().find(|i| i.measure == "g").unwrap();
        assert_eq!(grams_ingredient.amount, 100.0);
        assert_eq!(grams_ingredient.name, "sugar");
        assert!(grams_ingredient.dish.contains("Dish B"));
    }

    #[test]
    fn test_accumulate_different_ingredients_kept_separate() {
        let mut list = IngredientList::from(vec![
            Ingredient {
                amount: 200.0,
                measure: "g".to_string(),
                name: "flour".to_string(),
                dish: "Dish A".to_string(),
            },
            Ingredient {
                amount: 150.0,
                measure: "g".to_string(),
                name: "sugar".to_string(),
                dish: "Dish A".to_string(),
            },
            Ingredient {
                amount: 100.0,
                measure: "g".to_string(),
                name: "butter".to_string(),
                dish: "Dish B".to_string(),
            },
            Ingredient {
                amount: 50.0,
                measure: "g".to_string(),
                name: "flour".to_string(),
                dish: "Dish C".to_string(),
            },
        ]);

        list.accumulate();

        assert_eq!(list.0.len(), 3);

        let flour = list.0.iter().find(|i| i.name == "flour").unwrap();
        assert_eq!(flour.amount, 250.0);
        assert_eq!(flour.measure, "g");
        assert!(flour.dish.contains("Dish A"));
        assert!(flour.dish.contains("Dish C"));

        let sugar = list.0.iter().find(|i| i.name == "sugar").unwrap();
        assert_eq!(sugar.amount, 150.0);
        assert_eq!(sugar.measure, "g");
        assert_eq!(sugar.dish, "Dish A");

        let butter = list.0.iter().find(|i| i.name == "butter").unwrap();
        assert_eq!(butter.amount, 100.0);
        assert_eq!(butter.measure, "g");
        assert_eq!(butter.dish, "Dish B");
    }
}
