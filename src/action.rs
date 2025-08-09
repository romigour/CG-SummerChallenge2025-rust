#[derive(Clone, Debug)]
pub enum TypeAction {
    Throw,
    Shoot,
    HunkerDown,
}
#[derive(Debug)]
pub struct Action {
   id: i32,
   mx: i32,
   my: i32,
   type_action: TypeAction,
   x: i32,
   y: i32,
   ennemy_id: i32,
}

impl Action {
    pub fn new(id: i32, mx: i32, my: i32, type_action: TypeAction, x: i32, y: i32, ennemy_id: i32) -> Self {
        Action { id, mx, my, type_action, x, y, ennemy_id }
    }

    pub fn shoot(id: i32, mx: i32, my: i32, ennemy_id: i32) -> Self {
        Self::new(id, mx, my, TypeAction::Shoot, 0, 0, ennemy_id)
    }

    pub fn throw(id: i32, mx: i32, my: i32, x: i32, y: i32) -> Self {
        Self::new(id, mx, my, TypeAction::Throw, x, y, 0)
    }

    pub fn hunker_down(id: i32, mx: i32, my: i32) -> Self {
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