use sea_orm::entity::prelude::*;

// Utensil entity
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "utensil")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
	pub config: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

// impl RelationTrait for Relation {
//     fn def(&self) -> RelationDef {
//         // match self {
//         //     Self::Fruit => Entity::has_many(_)
//         // }
//         None
//     }
// }

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        Ok(self)
    }
}
