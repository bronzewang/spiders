// use sea_orm::entity::preclude::*;
use crate::Caliber;

pub struct Utensil {
    pub calibers: Vec<Caliber>,
}
