use sea_orm::ConnectionTrait;

pub struct Repository<'a, C: ConnectionTrait> {
    pub db: &'a C,
}

impl<'a, C: ConnectionTrait> Repository<'a, C> {
    pub fn new(db: &'a C) -> Self {
        Self { db }
    }
}
