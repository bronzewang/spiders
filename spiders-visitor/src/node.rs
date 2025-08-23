use leptos::*;
use leptos_struct_table::*;
use sea_orm::{entity::prelude::*};
// use ::chrono::NaiveDate;

// This generates the component NodeTable
#[derive(TableRow)]
#[table(sortable, impl_vec_data_provider)]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "node")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub name: String,
    pub addr: String,

    // pub logtime: NaiveDate,

    // #[table(skip)]
    // pub author: Author,

    // // specified that there is a getter method `author_name()`
    // pub author_name: FieldGetter<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        Ok(self)
    }
}

// impl Node {
//     // if no otherwise specified the getter method should have the same name as the `FieldGetter` field
//     pub fn author_name(&self) -> String {
//         format!("{} {}", self.author.first_name, self.author.last_name)
//     }

//     // getter for publish date
//     pub fn get_publish_date(&self) -> NaiveDate {
//         // do sth...
//         self.publish_date
//     }
// }

#[component]
pub fn Node() -> impl IntoView {

    let rows = vec![
        Model {
            id: 1,
            name: "nodeaio".to_string(),
            addr: "192.168.3.200".to_string(),
            // author: Author {
            //     first_name: "F. Scott".to_string(),
            //     last_name: "Fitzgerald".to_string(),
            // },
            // publish_date: NaiveDate::from_ymd_opt(1925, 4, 10).unwrap(),
            // author_name: Default::default(),
        },
        Model {
            id: 2,
            name: "nodew01".to_string(),
            addr: "192.168.3.201".to_string(),
            // author: Author {
            //     first_name: "John".to_string(),
            //     last_name: "Steinbeck".to_string(),
            // },
            // publish_date: NaiveDate::from_ymd_opt(1939, 4, 14).unwrap(),
            // author_name: Default::default(),
        },
    ];

    view! {
        <table>
            <TableContent rows/>
        </table>
    }
}
