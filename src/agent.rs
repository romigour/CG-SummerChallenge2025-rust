#[derive(Clone, Debug, Default, Copy)]
pub struct Agent {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub shoot_cooldown: i32,
    pub optimal_range: i32,
    pub soaking_power: i32,
    pub splash_bombs: i32,
    pub cooldown: i32,
    pub wetness: i32,
    pub team: Team,
    pub hunker_down: bool,
    pub is_dead: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Copy)]
pub enum Team {
    #[default]
    Me,
    Enemy,
}