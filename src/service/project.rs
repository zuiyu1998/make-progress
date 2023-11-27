use super::Service;

pub struct ProjectService<'a> {
    service: &'a Service,
}
