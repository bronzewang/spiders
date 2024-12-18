use toasty::codegen_support::*;
#[derive(Debug)]
pub struct Toolkit {
    pub id: Id<Toolkit>,
    pub name: String,
    utensils: HasMany<super::utensil::Utensil>,
    pub snooper_id: Option<Id<super::snooper::Snooper>>,
    snooper: BelongsTo<super::snooper::Snooper>,
}
impl Toolkit {
    pub const ID: Path<Id<Toolkit>> = Path::from_field_index::<Self>(0);
    pub const NAME: Path<String> = Path::from_field_index::<Self>(1);
    pub const UTENSILS: self::fields::Utensils =
        self::fields::Utensils::from_path(Path::from_field_index::<Self>(2));
    pub const SNOOPER_ID: Path<Id<super::snooper::Snooper>> = Path::from_field_index::<Self>(3);
    pub const SNOOPER: self::fields::Snooper =
        self::fields::Snooper::from_path(Path::from_field_index::<Self>(4));
    pub fn create<'a>() -> CreateToolkit<'a> {
        CreateToolkit::default()
    }
    pub fn create_many<'a>() -> CreateMany<'a, Toolkit> {
        CreateMany::default()
    }
    pub fn filter<'a>(expr: stmt::Expr<'a, bool>) -> Query<'a> {
        Query::from_stmt(stmt::Select::from_expr(expr))
    }
    pub fn update<'a>(&'a mut self) -> UpdateToolkit<'a> {
        UpdateToolkit {
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
impl Model for Toolkit {
    const ID: ModelId = ModelId(1);
    const FIELD_COUNT: usize = 5;
    type Key = Id<Toolkit>;
    fn load(mut record: Record<'_>) -> Result<Self, Error> {
        Ok(Toolkit {
            id: Id::from_untyped(record[0].take().to_id()?),
            name: record[1].take().to_string()?,
            utensils: HasMany::load(record[2].take())?,
            snooper_id: record[3].take().to_option_id()?.map(Id::from_untyped),
            snooper: BelongsTo::load(record[4].take())?,
        })
    }
}
impl<'a> stmt::IntoSelect<'a> for &'a Toolkit {
    type Model = Toolkit;
    fn into_select(self) -> stmt::Select<'a, Self::Model> {
        Toolkit::find_by_id(&self.id).into_select()
    }
}
impl stmt::AsSelect for Toolkit {
    type Model = Toolkit;
    fn as_select(&self) -> stmt::Select<'_, Self::Model> {
        Toolkit::find_by_id(&self.id).into_select()
    }
}
impl stmt::IntoSelect<'static> for Toolkit {
    type Model = Toolkit;
    fn into_select(self) -> stmt::Select<'static, Self::Model> {
        Toolkit::find_by_id(self.id).into_select()
    }
}
impl<'a> stmt::IntoExpr<'a, Toolkit> for &'a Toolkit {
    fn into_expr(self) -> stmt::Expr<'a, Toolkit> {
        stmt::Key::from_expr(&self.id).into()
    }
}
impl<'a> stmt::IntoExpr<'a, [Toolkit]> for &'a Toolkit {
    fn into_expr(self) -> stmt::Expr<'a, [Toolkit]> {
        stmt::Key::from_expr(&self.id).into()
    }
}
#[derive(Debug)]
pub struct Query<'a> {
    stmt: stmt::Select<'a, Toolkit>,
}
impl<'a> Query<'a> {
    pub const fn from_stmt(stmt: stmt::Select<'a, Toolkit>) -> Query<'a> {
        Query { stmt }
    }
    pub async fn all(self, db: &'a Db) -> Result<Cursor<'a, Toolkit>> {
        db.all(self.stmt).await
    }
    pub async fn first(self, db: &Db) -> Result<Option<Toolkit>> {
        db.first(self.stmt).await
    }
    pub async fn get(self, db: &Db) -> Result<Toolkit> {
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
        A: FromCursor<Toolkit>,
    {
        self.all(db).await?.collect().await
    }
    pub fn filter(self, expr: stmt::Expr<'a, bool>) -> Query<'a> {
        Query {
            stmt: self.stmt.and(expr),
        }
    }
    pub fn snooper(mut self) -> super::snooper::Query<'a> {
        todo!()
    }
}
impl<'a> stmt::IntoSelect<'a> for Query<'a> {
    type Model = Toolkit;
    fn into_select(self) -> stmt::Select<'a, Toolkit> {
        self.stmt
    }
}
impl<'a> stmt::IntoSelect<'a> for &Query<'a> {
    type Model = Toolkit;
    fn into_select(self) -> stmt::Select<'a, Toolkit> {
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
pub struct CreateToolkit<'a> {
    pub(super) stmt: stmt::Insert<'a, Toolkit>,
}
impl<'a> CreateToolkit<'a> {
    pub fn id(mut self, id: impl Into<Id<Toolkit>>) -> Self {
        self.stmt.set_value(0, id.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.stmt.set_value(1, name.into());
        self
    }
    pub fn utensil(mut self, utensil: impl IntoExpr<'a, super::utensil::Utensil>) -> Self {
        self.stmt.push_expr(2, utensil.into_expr());
        self
    }
    pub fn snooper_id(mut self, snooper_id: impl Into<Id<super::snooper::Snooper>>) -> Self {
        self.stmt.set_value(3, snooper_id.into());
        self
    }
    pub fn snooper<'b>(mut self, snooper: impl IntoExpr<'a, self::relation::Snooper<'b>>) -> Self {
        self.stmt.set_expr(4, snooper.into_expr());
        self
    }
    pub async fn exec(self, db: &'a Db) -> Result<Toolkit> {
        db.exec_insert_one::<Toolkit>(self.stmt).await
    }
}
impl<'a> IntoInsert<'a> for CreateToolkit<'a> {
    type Model = Toolkit;
    fn into_insert(self) -> stmt::Insert<'a, Toolkit> {
        self.stmt
    }
}
impl<'a> IntoExpr<'a, Toolkit> for CreateToolkit<'a> {
    fn into_expr(self) -> stmt::Expr<'a, Toolkit> {
        self.stmt.into()
    }
}
impl<'a> IntoExpr<'a, [Toolkit]> for CreateToolkit<'a> {
    fn into_expr(self) -> stmt::Expr<'a, [Toolkit]> {
        self.stmt.into_list_expr()
    }
}
impl<'a> Default for CreateToolkit<'a> {
    fn default() -> CreateToolkit<'a> {
        CreateToolkit {
            stmt: stmt::Insert::blank(),
        }
    }
}
#[derive(Debug)]
pub struct UpdateToolkit<'a> {
    model: &'a mut Toolkit,
    query: UpdateQuery<'a>,
}
#[derive(Debug)]
pub struct UpdateQuery<'a> {
    stmt: stmt::Update<'a, Toolkit>,
}
impl<'a> UpdateToolkit<'a> {
    pub fn id(mut self, id: impl Into<Id<Toolkit>>) -> Self {
        self.query.set_id(id);
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.query.set_name(name);
        self
    }
    pub fn utensil(mut self, utensil: impl IntoExpr<'a, super::utensil::Utensil>) -> Self {
        self.query.add_utensil(utensil);
        self
    }
    pub fn snooper_id(mut self, snooper_id: impl Into<Id<super::snooper::Snooper>>) -> Self {
        self.query.set_snooper_id(snooper_id);
        self
    }
    pub fn unset_snooper_id(&mut self) -> &mut Self {
        self.query.unset_snooper_id();
        self
    }
    pub fn snooper<'b>(mut self, snooper: impl IntoExpr<'a, self::relation::Snooper<'b>>) -> Self {
        self.query.set_snooper(snooper);
        self
    }
    pub async fn exec(self, db: &Db) -> Result<()> {
        let fields;
        let mut into_iter;
        {
            let mut stmt = self.query.stmt;
            fields = stmt.fields().clone();
            stmt.set_selection(&*self.model);
            let mut records = db.exec::<Toolkit>(stmt.into()).await?;
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
                2 => {}
                3 => {
                    self.model.snooper_id = into_iter
                        .next()
                        .unwrap()
                        .to_option_id()?
                        .map(stmt::Id::from_untyped)
                }
                4 => {
                    self.model.snooper_id =
                        stmt::Id::from_untyped(into_iter.next().unwrap().to_option_id()?)
                }
                _ => todo!("handle unknown field id in reload after update"),
            }
        }
        Ok(())
    }
}
impl<'a> UpdateQuery<'a> {
    pub fn id(mut self, id: impl Into<Id<Toolkit>>) -> Self {
        self.set_id(id);
        self
    }
    pub fn set_id(&mut self, id: impl Into<Id<Toolkit>>) -> &mut Self {
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
    pub fn utensil(mut self, utensil: impl IntoExpr<'a, super::utensil::Utensil>) -> Self {
        self.add_utensil(utensil);
        self
    }
    pub fn add_utensil(
        &mut self,
        utensil: impl IntoExpr<'a, super::utensil::Utensil>,
    ) -> &mut Self {
        self.stmt.push_expr(2, utensil.into_expr());
        self
    }
    pub fn snooper_id(mut self, snooper_id: impl Into<Id<super::snooper::Snooper>>) -> Self {
        self.set_snooper_id(snooper_id);
        self
    }
    pub fn set_snooper_id(
        &mut self,
        snooper_id: impl Into<Id<super::snooper::Snooper>>,
    ) -> &mut Self {
        self.stmt.set_expr(3, snooper_id.into());
        self
    }
    pub fn unset_snooper_id(&mut self) -> &mut Self {
        self.stmt.set(3, Value::Null);
        self
    }
    pub fn snooper<'b>(mut self, snooper: impl IntoExpr<'a, self::relation::Snooper<'b>>) -> Self {
        self.set_snooper(snooper);
        self
    }
    pub fn set_snooper<'b>(
        &mut self,
        snooper: impl IntoExpr<'a, self::relation::Snooper<'b>>,
    ) -> &mut Self {
        self.stmt.set_expr(4, snooper.into_expr());
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
impl<'a> From<stmt::Select<'a, Toolkit>> for UpdateQuery<'a> {
    fn from(src: stmt::Select<'a, Toolkit>) -> UpdateQuery<'a> {
        UpdateQuery {
            stmt: stmt::Update::new(src),
        }
    }
}
pub mod fields {
    use super::*;
    pub struct Utensils {
        pub(super) path: Path<[super::super::utensil::Utensil]>,
    }
    impl Utensils {
        pub const fn from_path(path: Path<[super::super::utensil::Utensil]>) -> Utensils {
            Utensils { path }
        }
        pub fn id(mut self) -> Path<Id<super::super::utensil::Utensil>> {
            self.path.chain(super::super::utensil::Utensil::ID)
        }
        pub fn name(mut self) -> Path<String> {
            self.path.chain(super::super::utensil::Utensil::NAME)
        }
        pub fn tooklit_id(mut self) -> Path<Id<Toolkit>> {
            self.path.chain(super::super::utensil::Utensil::TOOKLIT_ID)
        }
        pub fn tooklit(mut self) -> super::super::utensil::fields::Tooklit {
            let path = self.path.chain(super::super::utensil::Utensil::TOOKLIT);
            super::super::utensil::fields::Tooklit::from_path(path)
        }
    }
    impl From<Utensils> for Path<[super::super::utensil::Utensil]> {
        fn from(val: Utensils) -> Path<[super::super::utensil::Utensil]> {
            val.path
        }
    }
    impl<'stmt> stmt::IntoExpr<'stmt, super::relation::utensils::Utensils<'stmt>> for Utensils {
        fn into_expr(self) -> stmt::Expr<'stmt, super::relation::utensils::Utensils<'stmt>> {
            todo!("into_expr for {} (field path struct)", stringify!(Utensils));
        }
    }
    pub struct Snooper {
        pub(super) path: Path<super::super::snooper::Snooper>,
    }
    impl Snooper {
        pub const fn from_path(path: Path<super::super::snooper::Snooper>) -> Snooper {
            Snooper { path }
        }
        pub fn id(mut self) -> Path<Id<super::super::snooper::Snooper>> {
            self.path.chain(super::super::snooper::Snooper::ID)
        }
        pub fn name(mut self) -> Path<String> {
            self.path.chain(super::super::snooper::Snooper::NAME)
        }
        pub fn toolkits(mut self) -> super::super::snooper::fields::Toolkits {
            let path = self.path.chain(super::super::snooper::Snooper::TOOLKITS);
            super::super::snooper::fields::Toolkits::from_path(path)
        }
        pub fn eq<'a, 'b, T>(self, rhs: T) -> stmt::Expr<'a, bool>
        where
            T: toasty::stmt::IntoExpr<'a, super::relation::snooper::Snooper<'b>>,
        {
            self.path.eq(rhs.into_expr().cast())
        }
        pub fn in_query<'a, Q>(self, rhs: Q) -> toasty::stmt::Expr<'a, bool>
        where
            Q: stmt::IntoSelect<'a, Model = super::super::snooper::Snooper>,
        {
            self.path.in_query(rhs)
        }
    }
    impl From<Snooper> for Path<super::super::snooper::Snooper> {
        fn from(val: Snooper) -> Path<super::super::snooper::Snooper> {
            val.path
        }
    }
    impl<'stmt> stmt::IntoExpr<'stmt, super::relation::snooper::Snooper<'stmt>> for Snooper {
        fn into_expr(self) -> stmt::Expr<'stmt, super::relation::snooper::Snooper<'stmt>> {
            todo!("into_expr for {} (field path struct)", stringify!(Snooper));
        }
    }
}
pub mod relation {
    use super::*;
    use toasty::Cursor;
    pub mod utensils {
        use super::*;
        #[derive(Debug)]
        pub struct Utensils<'a> {
            scope: &'a Toolkit,
        }
        #[derive(Debug)]
        pub struct Query<'a> {
            pub(super) scope: super::Query<'a>,
        }
        #[derive(Debug)]
        pub struct Remove<'a> {
            stmt: stmt::Unlink<'a, super::Toolkit>,
        }
        #[derive(Debug)]
        pub struct Add<'a> {
            stmt: stmt::Link<'a, super::Toolkit>,
        }
        impl super::Toolkit {
            pub fn utensils(&self) -> Utensils<'_> {
                Utensils { scope: self }
            }
        }
        impl<'a> super::Query<'a> {
            pub fn utensils(self) -> Query<'a> {
                Query::with_scope(self)
            }
        }
        impl<'a> Utensils<'a> {
            pub fn get(&self) -> &[super::super::super::utensil::Utensil] {
                self.scope.utensils.get()
            }
            #[doc = r" Iterate all entries in the relation"]
            pub async fn all(
                self,
                db: &'a Db,
            ) -> Result<Cursor<'a, super::super::super::utensil::Utensil>> {
                db.all(self.into_select()).await
            }
            pub async fn collect<A>(self, db: &'a Db) -> Result<A>
            where
                A: FromCursor<super::super::super::utensil::Utensil>,
            {
                self.all(db).await?.collect().await
            }
            #[doc = r" Create a new associated record"]
            pub fn create(self) -> super::super::super::utensil::CreateUtensil<'a> {
                let mut builder = super::super::super::utensil::CreateUtensil::default();
                builder.stmt.set_scope(self);
                builder
            }
            pub fn query(
                self,
                filter: stmt::Expr<'a, bool>,
            ) -> super::super::super::utensil::Query<'a> {
                let query = self.into_select();
                super::super::super::utensil::Query::from_stmt(query.and(filter))
            }
            #[doc = r" Add an item to the association"]
            pub fn add(
                self,
                utensils: impl IntoSelect<'a, Model = super::super::super::utensil::Utensil>,
            ) -> Add<'a> {
                Add {
                    stmt: stmt::Link::new(self.scope, super::Toolkit::UTENSILS, utensils),
                }
            }
            #[doc = r" Remove items from the association"]
            pub fn remove(
                self,
                utensils: impl IntoSelect<'a, Model = super::super::super::utensil::Utensil>,
            ) -> Remove<'a> {
                Remove {
                    stmt: stmt::Unlink::new(self.scope, super::Toolkit::UTENSILS, utensils),
                }
            }
            pub fn find_by_id(
                self,
                id: impl stmt::IntoExpr<'a, Id<super::super::super::utensil::Utensil>>,
            ) -> FindByTooklitAndId<'a> {
                FindByTooklitAndId {
                    stmt: stmt::Select::from_expr(
                        super::super::super::utensil::Utensil::TOOKLIT
                            .in_query(self.scope)
                            .and(super::super::super::utensil::Utensil::ID.eq(id)),
                    ),
                }
            }
        }
        impl<'a> stmt::IntoSelect<'a> for Utensils<'a> {
            type Model = super::super::super::utensil::Utensil;
            fn into_select(self) -> stmt::Select<'a, super::super::super::utensil::Utensil> {
                super::super::super::utensil::Utensil::filter(
                    super::super::super::utensil::Utensil::TOOKLIT.in_query(self.scope),
                )
                .into_select()
            }
        }
        impl<'a> Query<'a> {
            pub fn with_scope<S>(scope: S) -> Query<'a>
            where
                S: IntoSelect<'a, Model = Toolkit>,
            {
                Query {
                    scope: super::Query::from_stmt(scope.into_select()),
                }
            }
            pub fn find_by_id(
                self,
                id: impl stmt::IntoExpr<'a, Id<super::super::super::utensil::Utensil>>,
            ) -> FindByTooklitAndId<'a> {
                FindByTooklitAndId {
                    stmt: stmt::Select::from_expr(
                        super::super::super::utensil::Utensil::TOOKLIT
                            .in_query(self.scope)
                            .and(super::super::super::utensil::Utensil::ID.eq(id)),
                    ),
                }
            }
        }
        impl<'a> Add<'a> {
            pub async fn exec(self, db: &'a Db) -> Result<()> {
                let mut cursor = db.exec(self.stmt.into()).await?;
                assert!(cursor.next().await.is_none());
                Ok(())
            }
        }
        impl<'a> Remove<'a> {
            pub async fn exec(self, db: &'a Db) -> Result<()> {
                let mut cursor = db.exec(self.stmt.into()).await?;
                assert!(cursor.next().await.is_none());
                Ok(())
            }
        }
        pub struct FindByTooklitAndId<'a> {
            stmt: stmt::Select<'a, super::super::super::utensil::Utensil>,
        }
        impl<'a> FindByTooklitAndId<'a> {
            pub async fn all(
                self,
                db: &'a Db,
            ) -> Result<Cursor<'a, super::super::super::utensil::Utensil>> {
                db.all(self.stmt).await
            }
            pub async fn first(
                self,
                db: &Db,
            ) -> Result<Option<super::super::super::utensil::Utensil>> {
                db.first(self.stmt).await
            }
            pub async fn get(self, db: &Db) -> Result<super::super::super::utensil::Utensil> {
                db.get(self.stmt).await
            }
            pub fn update(self) -> super::super::super::utensil::UpdateQuery<'a> {
                super::super::super::utensil::UpdateQuery::from(self.stmt)
            }
            pub async fn delete(self, db: &Db) -> Result<()> {
                db.exec(self.stmt.delete()).await?;
                Ok(())
            }
        }
        impl<'a> stmt::IntoSelect<'a> for FindByTooklitAndId<'a> {
            type Model = super::super::super::utensil::Utensil;
            fn into_select(self) -> stmt::Select<'a, Self::Model> {
                self.stmt
            }
        }
    }
    pub use utensils::Utensils;
    pub mod snooper {
        use super::*;
        #[derive(Debug)]
        pub struct Snooper<'a> {
            scope: &'a Toolkit,
        }
        impl super::Toolkit {
            pub fn snooper(&self) -> Snooper<'_> {
                Snooper { scope: self }
            }
        }
        impl<'a> Snooper<'a> {
            pub fn get(&self) -> &super::super::super::snooper::Snooper {
                self.scope.snooper.get()
            }
        }
        impl<'a> stmt::IntoSelect<'a> for &'a Snooper<'_> {
            type Model = super::super::super::snooper::Snooper;
            fn into_select(self) -> stmt::Select<'a, Self::Model> {
                super::super::super::snooper::Snooper::find_by_id(
                    self.scope
                        .snooper_id
                        .as_ref()
                        .expect("TODO: handle null fk fields"),
                )
                .into_select()
            }
        }
        impl<'stmt, 'a> stmt::IntoExpr<'stmt, Snooper<'a>> for Snooper<'a> {
            fn into_expr(self) -> stmt::Expr<'stmt, Snooper<'a>> {
                todo!(
                    "stmt::IntoExpr for {} (belongs_to Fk struct) - self = {:#?}",
                    stringify!(Snooper),
                    self
                );
            }
        }
        impl<'stmt, 'a> stmt::IntoExpr<'stmt, Snooper<'a>> for &'stmt Snooper<'a> {
            fn into_expr(self) -> stmt::Expr<'stmt, Snooper<'a>> {
                todo!(
                    "stmt::IntoExpr for &'a {} (belongs_to Fk struct) - self = {:#?}",
                    stringify!(Snooper),
                    self
                );
            }
        }
        impl<'stmt, 'a> stmt::IntoExpr<'stmt, Snooper<'a>>
            for &'stmt super::super::super::snooper::Snooper
        {
            fn into_expr(self) -> stmt::Expr<'stmt, Snooper<'a>> {
                stmt::Expr::from_untyped(&self.id)
            }
        }
        impl<'stmt, 'a> stmt::IntoExpr<'stmt, Snooper<'a>>
            for super::super::super::snooper::CreateSnooper<'stmt>
        {
            fn into_expr(self) -> stmt::Expr<'stmt, Snooper<'a>> {
                let expr: stmt::Expr<'stmt, super::super::super::snooper::Snooper> =
                    self.stmt.into();
                expr.cast()
            }
        }
        impl<'a> Snooper<'a> {
            pub async fn find(&self, db: &Db) -> Result<super::super::super::snooper::Snooper> {
                db.get(self.into_select()).await
            }
        }
    }
    pub use snooper::Snooper;
}
pub mod queries {
    use super::*;
    impl super::Toolkit {
        pub fn find_by_id<'a>(id: impl stmt::IntoExpr<'a, Id<Toolkit>>) -> FindById<'a> {
            FindById {
                query: Query::from_stmt(stmt::Select::from_expr(Toolkit::ID.eq(id))),
            }
        }
    }
    pub struct FindById<'a> {
        query: Query<'a>,
    }
    impl<'a> FindById<'a> {
        pub async fn all(self, db: &'a Db) -> Result<Cursor<'a, super::Toolkit>> {
            self.query.all(db).await
        }
        pub async fn first(self, db: &Db) -> Result<Option<super::Toolkit>> {
            self.query.first(db).await
        }
        pub async fn get(self, db: &Db) -> Result<super::Toolkit> {
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
            A: FromCursor<super::Toolkit>,
        {
            self.all(db).await?.collect().await
        }
        pub fn utensils(mut self) -> self::relation::utensils::Query<'a> {
            self::relation::utensils::Query::with_scope(self)
        }
    }
    impl<'a> stmt::IntoSelect<'a> for FindById<'a> {
        type Model = super::Toolkit;
        fn into_select(self) -> stmt::Select<'a, Self::Model> {
            self.query.into_select()
        }
    }
    impl super::Toolkit {
        pub fn find_many_by_id<'a>() -> FindManyById<'a> {
            FindManyById { items: vec![] }
        }
    }
    pub struct FindManyById<'a> {
        items: Vec<stmt::Expr<'a, Id<Toolkit>>>,
    }
    impl<'a> FindManyById<'a> {
        pub fn item(mut self, id: impl stmt::IntoExpr<'a, Id<Toolkit>>) -> Self {
            self.items.push(id.into_expr());
            self
        }
        pub async fn all(self, db: &'a Db) -> Result<Cursor<'a, super::Toolkit>> {
            db.all(self.into_select()).await
        }
        pub async fn first(self, db: &Db) -> Result<Option<super::Toolkit>> {
            db.first(self.into_select()).await
        }
        pub async fn get(self, db: &Db) -> Result<super::Toolkit> {
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
            A: FromCursor<super::Toolkit>,
        {
            self.all(db).await?.collect().await
        }
    }
    impl<'a> stmt::IntoSelect<'a> for FindManyById<'a> {
        type Model = super::Toolkit;
        fn into_select(self) -> stmt::Select<'a, Self::Model> {
            stmt::Select::from_expr(stmt::in_set(Toolkit::ID, self.items))
        }
    }
    impl super::Toolkit {
        pub fn find_by_snooper_id<'a>(
            snooper_id: impl stmt::IntoExpr<'a, Id<super::super::snooper::Snooper>>,
        ) -> FindBySnooperId<'a> {
            FindBySnooperId {
                query: Query::from_stmt(stmt::Select::from_expr(
                    Toolkit::SNOOPER_ID.eq(snooper_id),
                )),
            }
        }
    }
    pub struct FindBySnooperId<'a> {
        query: Query<'a>,
    }
    impl<'a> FindBySnooperId<'a> {
        pub async fn all(self, db: &'a Db) -> Result<Cursor<'a, super::Toolkit>> {
            self.query.all(db).await
        }
        pub async fn first(self, db: &Db) -> Result<Option<super::Toolkit>> {
            self.query.first(db).await
        }
        pub async fn get(self, db: &Db) -> Result<super::Toolkit> {
            self.query.get(db).await
        }
        pub fn update(self) -> super::UpdateQuery<'a> {
            super::UpdateQuery::from(self.query)
        }
        pub async fn delete(self, db: &Db) -> Result<()> {
            self.query.delete(db).await
        }
        pub fn include<T: ?Sized>(mut self, path: impl Into<Path<T>>) -> FindBySnooperId<'a> {
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
            A: FromCursor<super::Toolkit>,
        {
            self.all(db).await?.collect().await
        }
        pub fn utensils(mut self) -> self::relation::utensils::Query<'a> {
            self::relation::utensils::Query::with_scope(self)
        }
    }
    impl<'a> stmt::IntoSelect<'a> for FindBySnooperId<'a> {
        type Model = super::Toolkit;
        fn into_select(self) -> stmt::Select<'a, Self::Model> {
            self.query.into_select()
        }
    }
    impl super::Toolkit {
        pub fn find_many_by_snooper_id<'a>() -> FindManyBySnooperId<'a> {
            FindManyBySnooperId { items: vec![] }
        }
    }
    pub struct FindManyBySnooperId<'a> {
        items: Vec<stmt::Expr<'a, Id<super::super::snooper::Snooper>>>,
    }
    impl<'a> FindManyBySnooperId<'a> {
        pub fn item(
            mut self,
            snooper_id: impl stmt::IntoExpr<'a, Id<super::super::snooper::Snooper>>,
        ) -> Self {
            self.items.push(snooper_id.into_expr());
            self
        }
        pub async fn all(self, db: &'a Db) -> Result<Cursor<'a, super::Toolkit>> {
            db.all(self.into_select()).await
        }
        pub async fn first(self, db: &Db) -> Result<Option<super::Toolkit>> {
            db.first(self.into_select()).await
        }
        pub async fn get(self, db: &Db) -> Result<super::Toolkit> {
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
            A: FromCursor<super::Toolkit>,
        {
            self.all(db).await?.collect().await
        }
    }
    impl<'a> stmt::IntoSelect<'a> for FindManyBySnooperId<'a> {
        type Model = super::Toolkit;
        fn into_select(self) -> stmt::Select<'a, Self::Model> {
            stmt::Select::from_expr(stmt::in_set(Toolkit::SNOOPER_ID, self.items))
        }
    }
}
