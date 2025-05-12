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

pub(crate) struct IngredientList(Ingredients);

impl IngredientList {
    /// Accumulate ingredients.
    fn accumulate(&mut self) {
        self.0 = self
            .0
            .into_iter()
            .fold(
                HashMap::new(),
                |mut acc: HashMap<String, Vec<Ingredient>>, elem| {
                    if let Some(amounts) = acc.get_mut(&elem.name) {
                        amounts.push(elem)
                    } else {
                        acc.insert(elem.name, vec![elem]);
                    }

                    acc
                },
            )
            .iter()
            .collect()
    }
    /// Generate md shopping list.
    pub(crate) fn as_md_list(&self) -> String {
        let mut all = vec![];

        for (name, amount) in self.0 {
            all.push(format!(
                "- {name} [{}] ({})",
                amount
                    .iter()
                    .map(|amount| format!("{}{}", amount.amount, amount.measure))
                    .collect::<Vec<_>>()
                    .join(", "),
                amount
                    .iter()
                    .map(|amount| amount.dish.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        all.sort();
        all.join("\n")
    }

    /// Generate clustered md shopping list with AI.
    pub(crate) fn as_clustered_md_list(&self) -> String {
        todo!()
    }
}
