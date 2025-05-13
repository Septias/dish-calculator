use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum DishPlanError {
    #[error("The plan does not exist at the given location.")]
    #[from(tokio::fs::Error)]
    PlanDoesNotExist,
    #[error("The Dish does not exist at the given location.")]
    DishDoesNotExist,
    #[error("Can't parse the plan markdown")]
    MarkdownError,
}
