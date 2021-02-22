use valkyrie_core::ecs::Component;

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Debug;
impl Component for Debug {}

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Player;
impl Component for Player {}

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
impl Component for Position {}

// TODO: make an enum?
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Collidable {
    pub radius: u32,
}
impl Component for Collidable {}
