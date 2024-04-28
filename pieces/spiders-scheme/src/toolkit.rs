// use sea_orm::entity::preclude::*;
use crate::Utensil;

pub struct Toolkit {
    pub name: String,
    // device: Device,
    pub calibers: Vec<Utensil>,
    pub owner: bool,
}
