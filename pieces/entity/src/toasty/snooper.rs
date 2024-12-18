use toasty::codegen_support::*;
#[derive(Debug)]
pub struct Snooper {
    pub id: Id<Snooper>,
    pub name: String,
    toolkits: HasMany<super::toolkit::Toolkit>,
}
impl Snooper {
    pub const ID: Path<Id<Snooper>> = Path::from_field_index::<Self>(0);
    pub const NAME: Path<String> = Path::from_field_index::<Self>(1);
    pub const TOOLKITS: self::fields::Toolkits =
        self::fields::Toolkits::from_path(Path::from_field_index::<Self>(2));
    pub fn create<'a>() -> CreateSnooper<'a> {
        CreateSnooper::default()
    }
    pub fn create_many<'a>() -> CreateMany<'a, Snooper> {
        CreateMany::default()
    }
    pub fn filter<'a>(expr: stmt::Expr<'a, bool>) -> Query<'a> {
        Query::from_stmt(stmt::Select::from_expr(expr))
    }
    pub fn update<'a>(&'a mut self) -> UpdateSnooper<'a> {
        UpdateSnooper {
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
impl Model for Snooper {
    const ID: ModelId = ModelId(0);
    const FIELD_COUNT: usize = 3;
    type Key = Id<Snooper>;
    fn load(mut record: Record<'_>) -> Result<Self, Error> {
        Ok(Snooper {
            id: Id::from_untyped(record[0].take().to_id()?),
            name: record[1].take().to_string()?,
            toolkits: HasMany::load(record[2].take())?,
        })
    }
}
impl<'a> stmt::IntoSelect<'a> for &'a Snooper {
    type Model = Snooper;
    fn into_select(self) -> stmt::Select<'a, Self::Model> {
        Snooper::find_by_id(&self.id).into_select()
    }
}
impl stmt::AsSelect for Snooper {
    type Model = Snooper;
    fn as_select(&self) -> stmt::Select<'_, Self::Model> {
        Snooper::find_by_id(&self.id).into_select()
    }
}
impl stmt::IntoSelect<'static> for Snooper {
    type Model = Snooper;
    fn into_select(self) -> stmt::Select<'static, Self::Model> {
        Snooper::find_by_id(self.id).into_select()
    }
}
impl<'a> stmt::IntoExpr<'a, Snooper> for &'a Snooper {
    fn into_expr(self) -> stmt::Expr<'a, Snooper> {
        stmt::Key::from_expr(&self.id).into()
    }
}
impl<'a> stmt::IntoExpr<'a, [Snooper]> for &'a Snooper {
    fn into_expr(self) -> stmt::Expr<'a, [Snooper]> {
        stmt::Key::from_expr(&self.id).into()
    }
}
#[derive(Debug)]
pub struct Query<'a> {
    stmt: stmt::Select<'a, Snooper>,
}
impl<'a> Query<'a> {
    pub const fn from_stmt(stmt: stmt::Select<'a, Snooper>) -> Query<'a> {
        Query { stmt }
    }
    pub async fn all(self, db: &'a Db) -> Result<Cursor<'a, Snooper>> {
        db.all(self.stmt).await
    }
    pub async fn first(self, db: &Db) -> Result<Option<Snooper>> {
        db.first(self.stmt).await
    }
    pub async fn get(self, db: &Db) -> Result<Snooper> {
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
        A: FromCursor<Snooper>,
    {
        self.all(db).await?.collect().await
    }
    pub fn filter(self, expr: stmt::Expr<'a, bool>) -> Query<'a> {
        Query {
            stmt: self.stmt.and(expr),
        }
    }
}
impl<'a> stmt::IntoSelect<'a> for Query<'a> {
    type Model = Snooper;
    fn into_select(self) -> stmt::Select<'a, Snooper> {
        self.stmt
    }
}
impl<'a> stmt::IntoSelect<'a> for &Query<'a> {
    type Model = Snooper;
    fn into_select(self) -> stmt::Select<'a, Snooper> {
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
pub struct CreateSnooper<'a> {
    pub(super) stmt: stmt::Insert<'a, Snooper>,
}
impl<'a> CreateSnooper<'a> {
    pub fn id(mut self, id: impl Into<Id<Snooper>>) -> Self {
        self.stmt.set_value(0, id.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.stmt.set_value(1, name.into());
        self
    }
    pub fn toolkit(mut self, toolkit: impl IntoExpr<'a, super::toolkit::Toolkit>) -> Self {
        self.stmt.push_expr(2, toolkit.into_expr());
        self
    }
    pub async fn exec(self, db: &'a Db) -> Result<Snooper> {
        db.exec_insert_one::<Snooper>(self.stmt).await
    }
}
impl<'a> IntoInsert<'a> for CreateSnooper<'a> {
    type Model = Snooper;
    fn into_insert(self) -> stmt::Insert<'a, Snooper> {
        self.stmt
    }
}
impl<'a> IntoExpr<'a, Snooper> for CreateSnooper<'a> {
    fn into_expr(self) -> stmt::Expr<'a, Snooper> {
        self.stmt.into()
    }
}
impl<'a> IntoExpr<'a, [Snooper]> for CreateSnooper<'a> {
    fn into_expr(self) -> stmt::Expr<'a, [Snooper]> {
        self.stmt.into_list_expr()
    }
}
impl<'a> Default for CreateSnooper<'a> {
    fn default() -> CreateSnooper<'a> {
        CreateSnooper {
            stmt: stmt::Insert::blank(),
        }
    }
}
#[derive(Debug)]
pub struct UpdateSnooper<'a> {
    model: &'a mut Snooper,
    query: UpdateQuery<'a>,
}
#[derive(Debug)]
pub struct UpdateQuery<'a> {
    stmt: stmt::Update<'a, Snooper>,
}
impl<'a> UpdateSnooper<'a> {
    pub fn id(mut self, id: impl Into<Id<Snooper>>) -> Self {
        self.query.set_id(id);
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.query.set_name(name);
        self
    }
    pub fn toolkit(mut self, toolkit: impl IntoExpr<'a, super::toolkit::Toolkit>) -> Self {
        self.query.add_toolkit(toolkit);
        self
    }
    pub async fn exec(self, db: &Db) -> Result<()> {
        let fields;
        let mut into_iter;
        {
            let mut stmt = self.query.stmt;
            fields = stmt.fields().clone();
            stmt.set_selection(&*self.model);
            let mut records = db.exec::<Snooper>(stmt.into()).await?;
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
                _ => todo!("handle unknown field id in reload after update"),
            }
        }
        Ok(())
    }
}
impl<'a> UpdateQuery<'a> {
    pub fn id(mut self, id: impl Into<Id<Snooper>>) -> Self {
        self.set_id(id);
        self
    }
    pub fn set_id(&mut self, id: impl Into<Id<Snooper>>) -> &mut Self {
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
    pub fn toolkit(mut self, toolkit: impl IntoExpr<'a, super::toolkit::Toolkit>) -> Self {
        self.add_toolkit(toolkit);
        self
    }
    pub fn add_toolkit(
        &mut self,
        toolkit: impl IntoExpr<'a, super::toolkit::Toolkit>,
    ) -> &mut Self {
        self.stmt.push_expr(2, toolkit.into_expr());
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
impl<'a> From<stmt::Select<'a, Snooper>> for UpdateQuery<'a> {
    fn from(src: stmt::Select<'a, Snooper>) -> UpdateQuery<'a> {
        UpdateQuery {
            stmt: stmt::Update::new(src),
        }
    }
}
pub mod fields {
    use super::*;
    pub struct Toolkits {
        pub(super) path: Path<[super::super::toolkit::Toolkit]>,
    }
    impl Toolkits {
        pub const fn from_path(path: Path<[super::super::toolkit::Toolkit]>) -> Toolkits {
            Toolkits { path }
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
        pub fn snooper_id(mut self) -> Path<Id<Snooper>> {
            self.path.chain(super::super::toolkit::Toolkit::SNOOPER_ID)
        }
        pub fn snooper(mut self) -> super::super::toolkit::fields::Snooper {
            let path = self.path.chain(super::super::toolkit::Toolkit::SNOOPER);
            super::super::toolkit::fields::Snooper::from_path(path)
        }
    }
    impl From<Toolkits> for Path<[super::super::toolkit::Toolkit]> {
        fn from(val: Toolkits) -> Path<[super::super::toolkit::Toolkit]> {
            val.path
        }
    }
    impl<'stmt> stmt::IntoExpr<'stmt, super::relation::toolkits::Toolkits<'stmt>> for Toolkits {
        fn into_expr(self) -> stmt::Expr<'stmt, super::relation::toolkits::Toolkits<'stmt>> {
            todo!("into_expr for {} (field path struct)", stringify!(Toolkits));
        }
    }
}
pub mod relation {
    use super::*;
    use toasty::Cursor;
    pub mod toolkits {
        use super::*;
        #[derive(Debug)]
        pub struct Toolkits<'a> {
            scope: &'a Snooper,
        }
        #[derive(Debug)]
        pub struct Query<'a> {
            pub(super) scope: super::Query<'a>,
        }
        #[derive(Debug)]
        pub struct Remove<'a> {
            stmt: stmt::Unlink<'a, super::Snooper>,
        }
        #[derive(Debug)]
        pub struct Add<'a> {
            stmt: stmt::Link<'a, super::Snooper>,
        }
        impl super::Snooper {
            pub fn toolkits(&self) -> Toolkits<'_> {
                Toolkits { scope: self }
            }
        }
        impl<'a> super::Query<'a> {
            pub fn toolkits(self) -> Query<'a> {
                Query::with_scope(self)
            }
        }
        impl<'a> Toolkits<'a> {
            pub fn get(&self) -> &[super::super::super::toolkit::Toolkit] {
                self.scope.toolkits.get()
            }
            #[doc = r" Iterate all entries in the relation"]
            pub async fn all(
                self,
                db: &'a Db,
            ) -> Result<Cursor<'a, super::super::super::toolkit::Toolkit>> {
                db.all(self.into_select()).await
            }
            pub async fn collect<A>(self, db: &'a Db) -> Result<A>
            where
                A: FromCursor<super::super::super::toolkit::Toolkit>,
            {
                self.all(db).await?.collect().await
            }
            #[doc = r" Create a new associated record"]
            pub fn create(self) -> super::super::super::toolkit::CreateToolkit<'a> {
                let mut builder = super::super::super::toolkit::CreateToolkit::default();
                builder.stmt.set_scope(self);
                builder
            }
            pub fn query(
                self,
                filter: stmt::Expr<'a, bool>,
            ) -> super::super::super::toolkit::Query<'a> {
                let query = self.into_select();
                super::super::super::toolkit::Query::from_stmt(query.and(filter))
            }
            #[doc = r" Add an item to the association"]
            pub fn add(
                self,
                toolkits: impl IntoSelect<'a, Model = super::super::super::toolkit::Toolkit>,
            ) -> Add<'a> {
                Add {
                    stmt: stmt::Link::new(self.scope, super::Snooper::TOOLKITS, toolkits),
                }
            }
            #[doc = r" Remove items from the association"]
            pub fn remove(
                self,
                toolkits: impl IntoSelect<'a, Model = super::super::super::toolkit::Toolkit>,
            ) -> Remove<'a> {
                Remove {
                    stmt: stmt::Unlink::new(self.scope, super::Snooper::TOOLKITS, toolkits),
                }
            }
            pub fn find_by_id(
                self,
                id: impl stmt::IntoExpr<'a, Id<super::super::super::toolkit::Toolkit>>,
            ) -> FindBySnooperAndId<'a> {
                FindBySnooperAndId {
                    stmt: stmt::Select::from_expr(
                        super::super::super::toolkit::Toolkit::SNOOPER
                            .in_query(self.scope)
                            .and(super::super::super::toolkit::Toolkit::ID.eq(id)),
                    ),
                }
            }
        }
        impl<'a> stmt::IntoSelect<'a> for Toolkits<'a> {
            type Model = super::super::super::toolkit::Toolkit;
            fn into_select(self) -> stmt::Select<'a, super::super::super::toolkit::Toolkit> {
                super::super::super::toolkit::Toolkit::filter(
                    super::super::super::toolkit::Toolkit::SNOOPER.in_query(self.scope),
                )
                .into_select()
            }
        }
        impl<'a> Query<'a> {
            pub fn with_scope<S>(scope: S) -> Query<'a>
            where
                S: IntoSelect<'a, Model = Snooper>,
            {
                Query {
                    scope: super::Query::from_stmt(scope.into_select()),
                }
            }
            pub fn find_by_id(
                self,
                id: impl stmt::IntoExpr<'a, Id<super::super::super::toolkit::Toolkit>>,
            ) -> FindBySnooperAndId<'a> {
                FindBySnooperAndId {
                    stmt: stmt::Select::from_expr(
                        super::super::super::toolkit::Toolkit::SNOOPER
                            .in_query(self.scope)
                            .and(super::super::super::toolkit::Toolkit::ID.eq(id)),
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
        pub struct FindBySnooperAndId<'a> {
            stmt: stmt::Select<'a, super::super::super::toolkit::Toolkit>,
        }
        impl<'a> FindBySnooperAndId<'a> {
            pub async fn all(
                self,
                db: &'a Db,
            ) -> Result<Cursor<'a, super::super::super::toolkit::Toolkit>> {
                db.all(self.stmt).await
            }
            pub async fn first(
                self,
                db: &Db,
            ) -> Result<Option<super::super::super::toolkit::Toolkit>> {
                db.first(self.stmt).await
            }
            pub async fn get(self, db: &Db) -> Result<super::super::super::toolkit::Toolkit> {
                db.get(self.stmt).await
            }
            pub fn update(self) -> super::super::super::toolkit::UpdateQuery<'a> {
                super::super::super::toolkit::UpdateQuery::from(self.stmt)
            }
            pub async fn delete(self, db: &Db) -> Result<()> {
                db.exec(self.stmt.delete()).await?;
                Ok(())
            }
        }
        impl<'a> stmt::IntoSelect<'a> for FindBySnooperAndId<'a> {
            type Model = super::super::super::toolkit::Toolkit;
            fn into_select(self) -> stmt::Select<'a, Self::Model> {
                self.stmt
            }
        }
    }
    pub use toolkits::Toolkits;
}
pub mod queries {
    use super::*;
    impl super::Snooper {
        pub fn find_by_id<'a>(id: impl stmt::IntoExpr<'a, Id<Snooper>>) -> FindById<'a> {
            FindById {
                query: Query::from_stmt(stmt::Select::from_expr(Snooper::ID.eq(id))),
            }
        }
    }
    pub struct FindById<'a> {
        query: Query<'a>,
    }
    impl<'a> FindById<'a> {
        pub async fn all(self, db: &'a Db) -> Result<Cursor<'a, super::Snooper>> {
            self.query.all(db).await
        }
        pub async fn first(self, db: &Db) -> Result<Option<super::Snooper>> {
            self.query.first(db).await
        }
        pub async fn get(self, db: &Db) -> Result<super::Snooper> {
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
            A: FromCursor<super::Snooper>,
        {
            self.all(db).await?.collect().await
        }
        pub fn toolkits(mut self) -> self::relation::toolkits::Query<'a> {
            self::relation::toolkits::Query::with_scope(self)
        }
    }
    impl<'a> stmt::IntoSelect<'a> for FindById<'a> {
        type Model = super::Snooper;
        fn into_select(self) -> stmt::Select<'a, Self::Model> {
            self.query.into_select()
        }
    }
    impl super::Snooper {
        pub fn find_many_by_id<'a>() -> FindManyById<'a> {
            FindManyById { items: vec![] }
        }
    }
    pub struct FindManyById<'a> {
        items: Vec<stmt::Expr<'a, Id<Snooper>>>,
    }
    impl<'a> FindManyById<'a> {
        pub fn item(mut self, id: impl stmt::IntoExpr<'a, Id<Snooper>>) -> Self {
            self.items.push(id.into_expr());
            self
        }
        pub async fn all(self, db: &'a Db) -> Result<Cursor<'a, super::Snooper>> {
            db.all(self.into_select()).await
        }
        pub async fn first(self, db: &Db) -> Result<Option<super::Snooper>> {
            db.first(self.into_select()).await
        }
        pub async fn get(self, db: &Db) -> Result<super::Snooper> {
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
            A: FromCursor<super::Snooper>,
        {
            self.all(db).await?.collect().await
        }
    }
    impl<'a> stmt::IntoSelect<'a> for FindManyById<'a> {
        type Model = super::Snooper;
        fn into_select(self) -> stmt::Select<'a, Self::Model> {
            stmt::Select::from_expr(stmt::in_set(Snooper::ID, self.items))
        }
    }
}
