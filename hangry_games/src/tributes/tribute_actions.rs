#[derive(Clone, Debug, Default, PartialEq)]
pub enum TributeActions {
    #[default]
    Idle,
    Move,
    Rest,
    UseItem,
    Attack,
    Hide,
}
