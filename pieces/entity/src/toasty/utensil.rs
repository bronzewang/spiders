use toasty::codegen_support::*;
#[derive(Debug)]
pub struct Utensil {
    pub id: Id<Utensil>,
    pub name: String,
    pub tooklit_id: Id<super::toolkit::Toolkit>,
    tooklit: BelongsTo<super::toolkit::Toolkit>,
}
impl Utensil {
    pub const ID: Path<Id<Utensil>> = Path::from_field_index::<Self>(0);
    pub const NAME: Path<String> = Path::from_field_index::<Self>(1);
    pub const TOOKLIT_ID: Path<Id<super::toolkit::Toolkit>> = Path::from_field_index::<Self>(2);
    pub const TOOKLIT: self::fields::Tooklit =
        self::fields::Tooklit::from_path(Path::from_field_index::<Self>(3));
    pub fn create<'a>() -> CreateUtensil<'a> {
        CreateUtensil::default()
    }
    pub fn create_many<'a>() -> CreateMany<'a, Utensil> {
        CreateMany::default()
    }
    pub fn filter<'a>(expr: stmt::Expr<'a, bool>) -> Query<'a> {
        Query::from_stmt(stmt::Select::from_expr(expr))
    }
    pub fn update<'a>(&'a mut self) -> UpdateUtensil<'a> {
        UpdateUtensil {
            model: self,
            query: UpdateQuery {
                stmt: stmt::Update::default(),
            },
        }
    }
    pub async fn delete(self, db: &Db) -> Result<()> {
        let stmt = self.into_select().delete();
        db.exec(stmt).await?;
        Ok(())
    }
}
impl Model for Utensil {
    const ID: ModelId = ModelId(2);
    const FIELD_COUNT: usize = 4;
    type Key = Id<Utensil>;
    fn load(mut record: Record<'_>) -> Result<Self, Error> {
        Ok(Utensil {
            id: Id::from_untyped(record[0].take().to_id()?),
            name: record[1].take().to_string()?,
            tooklit_id: Id::from_untyped(record[2].take().to_id()?),
            tooklit: BelongsTo::load(record[3].take())?,
        })
    }
}
impl<'a> stmt::IntoSelect<'a> for &'a Utensil {
    type Model = Utensil;
    fn into_select(self) -> stmt::Select<'a, Self::Model> {
        Utensil::find_by_id(&self.id).into_select()
    }
}
impl stmt::AsSelect for Utensil {
    type Model = Utensil;
    fn as_select(&self) -> stmt::Select<'_, Self::Model> {
        Utensil::find_by_id(&self.id).into_select()
    }
}
impl stmt::IntoSelect<'static> for Utensil {
    type Model = Utensil;
    fn into_select(self) -> stmt::Select<'static, Self::Model> {
        Utensil::find_by_id(self.id).into_select()
    }
}
impl<'a> stmt::IntoExpr<'a, Utensil> for &'a Utensil {
    fn into_expr(self) -> stmt::Expr<'a, Utensil> {
        stmt::Key::from_expr(&self.id).into()
    }
}
impl<'a> stmt::IntoExpr<'a, [Utensil]> for &'a Utensil {
    fn into_expr(self) -> stmt::Expr<'a, [Utensil]> {
        stmt::Key::from_expr(&self.id).into()
    }
}
#[derive(Debug)]
pub struct Query<'a> {
    stmt: stmt::Select<'a, Utensil>,
}
impl<'a> Query<'a> {
    pub const fn from_stmt(stmt: stmt::Select<'a, Utensil>) -> Query<'a> {
        Query { stmt }
    }
    pub async fn all(self, db: &'a Db) -> Result<Cursor<'a, Utensil>> {
        db.all(self.stmt).await
    }
    pub async fn first(self, db: &Db) -> Result<Option<Utensil>> {
        db.first(self.stmt).await
    }
    pub async fn get(self, db: &Db) -> Result<Utensil> {
        db.get(self.stmt).await
    }
    pub fn update(self) -> UpdateQuery<'a> {
        UpdateQuery::from(self)
    }
    pub async fn delete(self, db: &Db) -> Result<()> {
        db.exec(self.stmt.delete()).await?;
        Ok(())
    }
    pub async fn collect<A>(self, db: &'a Db) -> Result<A>
    where
        A: FromCursor<Utensil>,
    {
        self.all(db).await?.collect().await
    }
    pub fn filter(self, expr: stmt::Expr<'a, bool>) -> Query<'a> {
        Query {
            stmt: self.stmt.and(expr),
        }
    }
    pub fn tooklit(mut self) -> super::toolkit::Query<'a> {
        todo!()
    }
}
impl<'a> stmt::IntoSelect<'a> for Query<'a> {
    type Model = Utensil;
    fn into_select(self) -> stmt::Select<'a, Utensil> {
        self.stmt
    }
}
impl<'a> stmt::IntoSelect<'a> for &Query<'a> {
    type Model = Utensil;
    fn into_select(self) -> stmt::Select<'a, Utensil> {
        self.stmt.clone()
    }
}
impl Default for Query<'static> {
    fn default() -> Query<'static> {
        Query {
            stmt: stmt::Select::all(),
        }
    }
}
#[derive(Debug)]
pub struct CreateUtensil<'a> {
    pub(super) stmt: stmt::Insert<'a, Utensil>,
}
impl<'a> CreateUtensil<'a> {
    pub fn id(mut self, id: impl Into<Id<Utensil>>) -> Self {
        self.stmt.set_value(0, id.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.stmt.set_value(1, name.into());
        self
    }
    pub fn tooklit_id(mut self, tooklit_id: impl Into<Id<super::toolkit::Toolkit>>) -> Self {
        self.stmt.set_value(2, tooklit_id.into());
        self
    }
    pub fn tooklit<'b>(mut self, tooklit: impl IntoExpr<'a, self::relation::Tooklit<'b>>) -> Self {
        self.stmt.set_expr(3, tooklit.into_expr());
        self
    }
    pub async fn exec(self, db: &'a Db) -> Result<Utensil> {
        db.exec_insert_one::<Utensil>(self.stmt).await
    }
}
impl<'a> IntoInsert<'a> for CreateUtensil<'a> {
    type Model = Utensil;
    fn into_insert(self) -> stmt::Insert<'a, Utensil> {
        self.stmt
    }
}
impl<'a> IntoExpr<'a, Utensil> for CreateUtensil<'a> {
    fn into_expr(self) -> stmt::Expr<'a, Utensil> {
        self.stmt.into()
    }
}
impl<'a> IntoExpr<'a, [Utensil]> for CreateUtensil<'a> {
    fn into_expr(self) -> stmt::Expr<'a, [Utensil]> {
        self.stmt.into_list_expr()
    }
}
impl<'a> Default for CreateUtensil<'a> {
    fn default() -> CreateUtensil<'a> {
        CreateUtensil {
            stmt: stmt::Insert::blank(),
        }
    }
}
#[derive(Debug)]
pub struct UpdateUtensil<'a> {
    model: &'a mut Utensil,
    query: UpdateQuery<'a>,
}
#[derive(Debug)]
pub struct UpdateQuery<'a> {
    stmt: stmt::Update<'a, Utensil>,
}
impl<'a> UpdateUtensil<'a> {
    pub fn id(mut self, id: impl Into<Id<Utensil>>) -> Self {
        self.query.set_id(id);
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.query.set_name(name);
        self
    }
    pub fn tooklit_id(mut self, tooklit_id: impl Into<Id<super::toolkit::Toolkit>>) -> Self {
        self.query.set_tooklit_id(tooklit_id);
        self
    }
    pub fn tooklit<'b>(mut self, tooklit: impl IntoExpr<'a, self::relation::Tooklit<'b>>) -> Self {
        self.query.set_tooklit(tooklit);
        self
    }
    pub async fn exec(self, db: &Db) -> Result<()> {
        let fields;
        let mut into_iter;
        {
            let mut stmt = self.query.stmt;
            fields = stmt.fields().clone();
            stmt.set_selection(&*self.model);
            let mut records = db.exec::<Utensil>(stmt.into()).await?;
            into_iter = records
                .next()
                .await
                .unwrap()?
                .into_record()
                .into_owned()
                .into_iter();
        }
        for field in fields.iter() {
            match field.into_usize() {
                0 => self.model.id = stmt::Id::from_untyped(into_iter.next().unwrap().to_id()?),
                1 => self.model.name = into_iter.next().unwrap().to_string()?,
                2 => {
                    self.model.tooklit_id =
                        stmt::Id::from_untyped(into_iter.next().unwrap().to_id()?)
                }
                3 => {
                    self.model.tooklit_id =
                        stmt::Id::from_untyped(into_iter.next().unwrap().to_id()?)
                }
                _ => todo!("handle unknown field id in reload after update"),
            }
        }
        Ok(())
    }
}
impl<'a> UpdateQuery<'a> {
    pub fn id(mut self, id: impl Into<Id<Utensil>>) -> Self {
        self.set_id(id);
        self
    }
    pub fn set_id(&mut self, id: impl Into<Id<Utensil>>) -> &mut Self {
        self.stmt.set_expr(0, id.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.set_name(name);
        self
    }
    pub fn set_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.stmt.set_expr(1, name.into());
        self
    }
    pub fn tooklit_id(mut self, tooklit_id: impl Into<Id<super::toolkit::Toolkit>>) -> Self {
        self.set_tooklit_id(tooklit_id);
        self
    }
    pub fn set_tooklit_id(
        &mut self,
        tooklit_id: impl Into<Id<super::toolkit::Toolkit>>,
    ) -> &mut Self {
        self.stmt.set_expr(2, tooklit_id.into());
        self
    }
    pub fn tooklit<'b>(mut self, tooklit: impl IntoExpr<'a, self::relation::Tooklit<'b>>) -> Self {
        self.set_tooklit(tooklit);
        self
    }
    pub fn set_tooklit<'b>(
        &mut self,
        tooklit: impl IntoExpr<'a, self::relation::Tooklit<'b>>,
    ) -> &mut Self {
        self.stmt.set_expr(3, tooklit.into_expr());
        self
    }
    pub async fn exec(self, db: &Db) -> Result<()> {
        let stmt = self.stmt;
        let mut cursor = db.exec(stmt.into()).await?;
        Ok(())
    }
}
impl<'a> From<Query<'a>> for UpdateQuery<'a> {
    fn from(value: Query<'a>) -> UpdateQuery<'a> {
        UpdateQuery {
            stmt: stmt::Update::new(value),
        }
    }
}
impl<'a> From<stmt::Select<'a, Utensil>> for UpdateQuery<'a> {
    fn from(src: stmt::Select<'a, Utensil>) -> UpdateQuery<'a> {
        UpdateQuery {
            stmt: stmt::Update::new(src),
        }
    }
}
pub mod fields {
    use super::*;
    pub struct Tooklit {
        pub(super) path: Path<super::super::toolkit::Toolkit>,
    }
    impl Tooklit {
        pub const fn from_path(path: Path<super::super::toolkit::Toolkit>) -> Tooklit {
            Tooklit { path }
        }
        pub fn id(mut self) -> Path<Id<super::super::toolkit::Toolkit>> {
            self.path.chain(super::super::toolkit::Toolkit::ID)
        }
        pub fn name(mut self) -> Path<String> {
            self.path.chain(super::super::toolkit::Toolkit::NAME)
        }
        pub fn utensils(mut self) -> super::super::toolkit::fields::Utensils {
            let path = self.path.chain(super::super::toolkit::Toolkit::UTENSILS);
            super::super::toolkit::fields::Utensils::from_path(path)
        }
        pub fn snooper_id(mut self) -> Path<Id<super::super::snooper::Snooper>> {
            self.path.chain(super::super::toolkit::Toolkit::SNOOPER_ID)
        }
        pub fn snooper(mut self) -> super::super::toolkit::fields::Snooper {
            let path = self.path.chain(super::super::toolkit::Toolkit::SNOOPER);
            super::super::toolkit::fields::Snooper::from_path(path)
        }
        pub fn eq<'a, 'b, T>(self, rhs: T) -> stmt::Expr<'a, bool>
        where
            T: toasty::stmt::IntoExpr<'a, super::relation::tooklit::Tooklit<'b>>,
        {
            self.path.eq(rhs.into_expr().cast())
        }
        pub fn in_query<'a, Q>(self, rhs: Q) -> toasty::stmt::Expr<'a, bool>
        where
            Q: stmt::IntoSelect<'a, Model = super::super::toolkit::Toolkit>,
        {
            self.path.in_query(rhs)
        }
    }
    impl From<Tooklit> for Path<super::super::toolkit::Toolkit> {
        fn from(val: Tooklit) -> Path<super::super::toolkit::Toolkit> {
            val.path
        }
    }
    impl<'stmt> stmt::IntoExpr<'stmt, super::relation::tooklit::Tooklit<'stmt>> for Tooklit {
        fn into_expr(self) -> stmt::Expr<'stmt, super::relation::tooklit::Tooklit<'stmt>> {
            todo!("into_expr for {} (field path struct)", stringify!(Tooklit));
        }
    }
}
pub mod relation {
    use super::*;
    use toasty::Cursor;
    pub mod tooklit {
        use super::*;
        #[derive(Debug)]
        pub struct Tooklit<'a> {
            scope: &'a Utensil,
        }
        impl super::Utensil {
            pub fn tooklit(&self) -> Tooklit<'_> {
                Tooklit { scope: self }
            }
        }
        impl<'a> Tooklit<'a> {
            pub fn get(&self) -> &super::super::super::toolkit::Toolkit {
                self.scope.tooklit.get()
            }
        }
        impl<'a> stmt::IntoSelect<'a> for &'a Tooklit<'_> {
            type Model = super::super::super::toolkit::Toolkit;
            fn into_select(self) -> stmt::Select<'a, Self::Model> {
                super::super::super::toolkit::Toolkit::find_by_id(&self.scope.tooklit_id)
                    .into_select()
            }
        }
        impl<'stmt, 'a> stmt::IntoExpr<'stmt, Tooklit<'a>> for Tooklit<'a> {
            fn into_expr(self) -> stmt::Expr<'stmt, Tooklit<'a>> {
                todo!(
                    "stmt::IntoExpr for {} (belongs_to Fk struct) - self = {:#?}",
                    stringify!(Tooklit),
                    self
                );
            }
        }
        impl<'stmt, 'a> stmt::IntoExpr<'stmt, Tooklit<'a>> for &'stmt Tooklit<'a> {
            fn into_expr(self) -> stmt::Expr<'stmt, Tooklit<'a>> {
                todo!(
                    "stmt::IntoExpr for &'a {} (belongs_to Fk struct) - self = {:#?}",
                    stringify!(Tooklit),
                    self
                );
            }
        }
        impl<'stmt, 'a> stmt::IntoExpr<'stmt, Tooklit<'a>>
            for &'stmt super::super::super::toolkit::Toolkit
        {
            fn into_expr(self) -> stmt::Expr<'stmt, Tooklit<'a>> {
                stmt::Expr::from_untyped(&self.id)
            }
        }
        impl<'stmt, 'a> stmt::IntoExpr<'stmt, Tooklit<'a>>
            for super::super::super::toolkit::CreateToolkit<'stmt>
        {
            fn into_expr(self) -> stmt::Expr<'stmt, Tooklit<'a>> {
                let expr: stmt::Expr<'stmt, super::super::super::toolkit::Toolkit> =
                    self.stmt.into();
                expr.cast()
            }
        }
        impl<'a> Tooklit<'a> {
            pub async fn find(&self, db: &Db) -> Result<super::super::super::toolkit::Toolkit> {
                db.get(self.into_select()).await
            }
        }
    }
    pub use tooklit::Tooklit;
}
pub mod queries {
    use super::*;
    impl super::Utensil {
        pub fn find_by_id<'a>(id: impl stmt::IntoExpr<'a, Id<Utensil>>) -> FindById<'a> {
            FindById {
                query: Query::from_stmt(stmt::Select::from_expr(Utensil::ID.eq(id))),
            }
        }
    }
    pub struct FindById<'a> {
        query: Query<'a>,
    }
    impl<'a> FindById<'a> {
        pub async fn all(self, db: &'a Db) -> Result<Cursor<'a, super::Utensil>> {
            self.query.all(db).await
        }
        pub async fn first(self, db: &Db) -> Result<Option<super::Utensil>> {
            self.query.first(db).await
        }
        pub async fn get(self, db: &Db) -> Result<super::Utensil> {
            self.query.get(db).await
        }
        pub fn update(self) -> super::UpdateQuery<'a> {
            super::UpdateQuery::from(self.query)
        }
        pub async fn delete(self, db: &Db) -> Result<()> {
            self.query.delete(db).await
        }
        pub fn include<T: ?Sized>(mut self, path: impl Into<Path<T>>) -> FindById<'a> {
            let path = path.into();
            self.query.stmt.include(path);
            self
        }
        pub fn filter(self, filter: stmt::Expr<'a, bool>) -> Query<'a> {
            let stmt = self.into_select();
            Query::from_stmt(stmt.and(filter))
        }
        pub async fn collect<A>(self, db: &'a Db) -> Result<A>
        where
            A: FromCursor<super::Utensil>,
        {
            self.all(db).await?.collect().await
        }
    }
    impl<'a> stmt::IntoSelect<'a> for FindById<'a> {
        type Model = super::Utensil;
        fn into_select(self) -> stmt::Select<'a, Self::Model> {
            self.query.into_select()
        }
    }
    impl super::Utensil {
        pub fn find_many_by_id<'a>() -> FindManyById<'a> {
            FindManyById { items: vec![] }
        }
    }
    pub struct FindManyById<'a> {
        items: Vec<stmt::Expr<'a, Id<Utensil>>>,
    }
    impl<'a> FindManyById<'a> {
        pub fn item(mut self, id: impl stmt::IntoExpr<'a, Id<Utensil>>) -> Self {
            self.items.push(id.into_expr());
            self
        }
        pub async fn all(self, db: &'a Db) -> Result<Cursor<'a, super::Utensil>> {
            db.all(self.into_select()).await
        }
        pub async fn first(self, db: &Db) -> Result<Option<super::Utensil>> {
            db.first(self.into_select()).await
        }
        pub async fn get(self, db: &Db) -> Result<super::Utensil> {
            db.get(self.into_select()).await
        }
        pub fn update(self) -> super::UpdateQuery<'a> {
            super::UpdateQuery::from(self.into_select())
        }
        pub async fn delete(self, db: &Db) -> Result<()> {
            db.delete(self.into_select()).await
        }
        pub fn filter(self, filter: stmt::Expr<'a, bool>) -> Query<'a> {
            let stmt = self.into_select();
            Query::from_stmt(stmt.and(filter))
        }
        pub async fn collect<A>(self, db: &'a Db) -> Result<A>
        where
            A: FromCursor<super::Utensil>,
        {
            self.all(db).await?.collect().await
        }
    }
    impl<'a> stmt::IntoSelect<'a> for FindManyById<'a> {
        type Model = super::Utensil;
        fn into_select(self) -> stmt::Select<'a, Self::Model> {
            stmt::Select::from_expr(stmt::in_set(Utensil::ID, self.items))
        }
    }
    impl super::Utensil {
        pub fn find_by_tooklit_id<'a>(
            tooklit_id: impl stmt::IntoExpr<'a, Id<super::super::toolkit::Toolkit>>,
        ) -> FindByTooklitId<'a> {
            FindByTooklitId {
                query: Query::from_stmt(stmt::Select::from_expr(
                    Utensil::TOOKLIT_ID.eq(tooklit_id),
                )),
            }
        }
    }
    pub struct FindByTooklitId<'a> {
        query: Query<'a>,
    }
    impl<'a> FindByTooklitId<'a> {
        pub async fn all(self, db: &'a Db) -> Result<Cursor<'a, super::Utensil>> {
            self.query.all(db).await
        }
        pub async fn first(self, db: &Db) -> Result<Option<super::Utensil>> {
            self.query.first(db).await
        }
        pub async fn get(self, db: &Db) -> Result<super::Utensil> {
            self.query.get(db).await
        }
        pub fn update(self) -> super::UpdateQuery<'a> {
            super::UpdateQuery::from(self.query)
        }
        pub async fn delete(self, db: &Db) -> Result<()> {
            self.query.delete(db).await
        }
        pub fn include<T: ?Sized>(mut self, path: impl Into<Path<T>>) -> FindByTooklitId<'a> {
            let path = path.into();
            self.query.stmt.include(path);
            self
        }
        pub fn filter(self, filter: stmt::Expr<'a, bool>) -> Query<'a> {
            let stmt = self.into_select();
            Query::from_stmt(stmt.and(filter))
        }
        pub async fn collect<A>(self, db: &'a Db) -> Result<A>
        where
            A: FromCursor<super::Utensil>,
        {
            self.all(db).await?.collect().await
        }
    }
    impl<'a> stmt::IntoSelect<'a> for FindByTooklitId<'a> {
        type Model = super::Utensil;
        fn into_select(self) -> stmt::Select<'a, Self::Model> {
            self.query.into_select()
        }
    }
    impl super::Utensil {
        pub fn find_many_by_tooklit_id<'a>() -> FindManyByTooklitId<'a> {
            FindManyByTooklitId { items: vec![] }
        }
    }
    pub struct FindManyByTooklitId<'a> {
        items: Vec<stmt::Expr<'a, Id<super::super::toolkit::Toolkit>>>,
    }
    impl<'a> FindManyByTooklitId<'a> {
        pub fn item(
            mut self,
            tooklit_id: impl stmt::IntoExpr<'a, Id<super::super::toolkit::Toolkit>>,
        ) -> Self {
            self.items.push(tooklit_id.into_expr());
            self
        }
        pub async fn all(self, db: &'a Db) -> Result<Cursor<'a, super::Utensil>> {
            db.all(self.into_select()).await
        }
        pub async fn first(self, db: &Db) -> Result<Option<super::Utensil>> {
            db.first(self.into_select()).await
        }
        pub async fn get(self, db: &Db) -> Result<super::Utensil> {
            db.get(self.into_select()).await
        }
        pub fn update(self) -> super::UpdateQuery<'a> {
            super::UpdateQuery::from(self.into_select())
        }
        pub async fn delete(self, db: &Db) -> Result<()> {
            db.delete(self.into_select()).await
        }
        pub fn filter(self, filter: stmt::Expr<'a, bool>) -> Query<'a> {
            let stmt = self.into_select();
            Query::from_stmt(stmt.and(filter))
        }
        pub async fn collect<A>(self, db: &'a Db) -> Result<A>
        where
            A: FromCursor<super::Utensil>,
        {
            self.all(db).await?.collect().await
        }
    }
    impl<'a> stmt::IntoSelect<'a> for FindManyByTooklitId<'a> {
        type Model = super::Utensil;
        fn into_select(self) -> stmt::Select<'a, Self::Model> {
            stmt::Select::from_expr(stmt::in_set(Utensil::TOOKLIT_ID, self.items))
        }
    }
}
