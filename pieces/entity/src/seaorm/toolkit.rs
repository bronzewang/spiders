use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "toolkit"
    }
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub id: i32,
    pub name: String,
    pub owner: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    Name,
    Owner,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Column::Id => ColumnType::Integer.def(),
            Column::Name => ColumnType::String(StringLen::None).def(),
            Column::Owner => ColumnType::Integer.def(),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}
impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Snooper,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::Snooper => Entity::belongs_to(super::snooper::Entity)
                .from(Column::Owner)
                .to(super::snooper::Column::Id)
                .into(),
        }
    }
}

impl Related<super::snooper::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Snooper.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
