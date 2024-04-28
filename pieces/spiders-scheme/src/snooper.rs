// use sea_orm::entity::preclude::*;
use crate::Toolkit;

pub struct Snooper {
    pub name: String,
    pub toolkits: Vec<Toolkit>,
}
