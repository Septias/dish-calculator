#[derive(Debug)]
struct Amount {
    /// Amount of ingredient.
    amount: f32,
    /// Measure of the ingredient.
    measure: String,
    /// Name of the ingredient.
    name: String,
    /// The dish this ingredient is from.
    dish: String,
}

struct Dish {
    /// The amount of people to feed.
    people: Option<usize>,
    /// For how many people the recipe is scaled.
    recepie_people: usize,
    /// List of recipe ingredients.
    ingredients: Vec<Amount>,
    /// Other text
    blocks: Vec<String>,
}

/// A single day
struct Day {
    /// List of dishes.
    dishes: Vec,
    /// The amount of people to feed.
    people: Option<usize>,
}

/// The week structure of a meal plan.
struct Week {
    /// Start date used for PDF export.
    start: chrono::Dateu<_>,
    /// Consecutive list of days.
    days: Vec<Day>,
    /// The amount of people to feed.
    people: usize,
}
