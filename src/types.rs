type Ingredients = Vec<Ingredient>;

trait IngredientAccumulator {
    fn accumulate(&self) -> Ingredients;
}

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

/// A single dish.
pub(crate) struct Dish {
    /// The amount of people to feed.
    pub(crate) people: Option<usize>,
    /// For how many people the recipe is scaled.
    pub(crate) recepie_people: usize,
    /// List of recipe ingredients.
    pub(crate) ingredients: Vec<Ingredient>,
    /// Other text
    pub(crate) blocks: Vec<String>,
}

impl IngredientAccumulator for Dish {
    fn accumulate(&self) -> Ingredients {
        todo!()
    }
}

/// A single day with multiple dishes.
pub(crate) struct Day {
    /// List of dishes.
    pub(crate) dishes: Vec<Dish>,
    /// The amount of people to feed.
    pub(crate) people: Option<usize>,
}

impl IngredientAccumulator for Day {
    fn accumulate(&self) -> Ingredients {
        todo!()
    }
}

/// The week structure of a meal plan.
pub(crate) struct WeekPlan {
    /// Start date used for PDF export.
    pub(crate) start: chrono::NaiveDate,
    /// Consecutive list of days.
    pub(crate) days: Vec<Day>,
    /// The amount of people to feed.
    pub(crate) people: usize,
}

impl IngredientAccumulator for WeekPlan {
    fn accumulate(&self) -> Ingredients {
        todo!()
    }
}
