#[derive(Clone, Debug, Default, Copy)]
pub struct Agent {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub shoot_cooldown: u32,
    pub optimal_range: u32,
    pub soaking_power: u32,
    pub splash_bombs: u32,
    pub cooldown: u32,
    pub wetness: u32,
    pub team: Team,
    pub is_dead: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Copy)]
pub enum Team {
    #[default]
    Me,
    Enemy,
}