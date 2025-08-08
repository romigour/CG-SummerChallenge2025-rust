#[derive(Clone, Debug)]
pub enum TypeAction {
    Throw,
    Shoot,
    HunkerDown,
}
#[derive(Debug)]
pub struct Action {
   id: u32,
   mx: u32,
   my: u32,
   type_action: TypeAction,
   x: u32,
   y: u32,
   ennemy_id: u32,
}

impl Action {
    pub fn new(id: u32, mx: u32, my: u32, type_action: TypeAction, x: u32, y: u32, ennemy_id: u32) -> Self {
        Action { id, mx, my, type_action, x, y, ennemy_id }
    }

    pub fn shoot(id: u32, mx: u32, my: u32, ennemy_id: u32) -> Self {
        Self::new(id, mx, my, TypeAction::Shoot, 0, 0, ennemy_id)
    }

    pub fn throw(id: u32, mx: u32, my: u32, x: u32, y: u32) -> Self {
        Self::new(id, mx, my, TypeAction::Throw, x, y, 0)
    }

    pub fn hunker_down(id: u32, mx: u32, my: u32) -> Self {
        Self::new(id, mx, my, TypeAction::HunkerDown, 0, 0, 0)
    }

    pub fn display(&self) -> String {
        match self.type_action {
            TypeAction::Throw => format!("{};MOVE {} {};THROW {} {}", self.id, self.mx, self.my, self.x, self.y),
            TypeAction::Shoot => format!("{};MOVE {} {};SHOOT {}", self.id, self.mx, self.my, self.ennemy_id),
            TypeAction::HunkerDown => format!("{};MOVE {} {};HUNKER_DOWN", self.id, self.mx, self.my),
        }
    }
}