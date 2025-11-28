use sea_orm::ConnectionTrait;

use crate::{
    domain::organization::repositories::{CompanyRepository, DepartmentRepository},
    infrastructure::db::Repository,
};

pub struct RepositoryProvider<'a, C: ConnectionTrait> {
    pub c: &'a C,
}

impl<'a, C: ConnectionTrait> RepositoryProvider<'a, C> {
    pub fn new(c: &'a C) -> Self {
        Self { c }
    }

    pub fn company_repo(&self) -> impl CompanyRepository {
        Repository::new(self.c)
    }

    pub fn department_repo(&self) -> impl DepartmentRepository {
        Repository::new(self.c)
    }
}
