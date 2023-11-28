use crate::{Service, ServiceResult};
use rc_storage::chrono::NaiveDate;

pub struct Link {}

pub struct Project {
    pub id: i32,
    pub name: String,
    pub background: Option<String>,
    pub create_at: NaiveDate,
    pub update_at: NaiveDate,
    pub end_at: Option<NaiveDate>,
    pub link_list: Vec<Link>,
}

pub struct ProjectService<'a> {
    service: &'a Service,
}

impl<'a> ProjectService<'a> {
    pub fn create_project(&self) -> ServiceResult<Project> {
        todo!()
    }
}
