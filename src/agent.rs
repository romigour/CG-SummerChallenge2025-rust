#[derive(Clone, Debug)]
pub struct Agent {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub shoot_cooldown: u32,
    pub optimal_range: u32,
    pub soaking_power: u32,
    pub splash_bombs: u32,
    pub team: Team,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Team {
    Me,
    Enemy,
}