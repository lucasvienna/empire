use crate::db::DbConn;
use crate::db::resources::ResourcesRepository;

pub struct ResourceService<'a> {
    connection: &'a mut DbConn,
    res_repo: ResourcesRepository,
}

impl ResourceService<'_> {
    pub fn new(connection: &mut DbConn) -> ResourceService {
        ResourceService {
            connection,
            res_repo: ResourcesRepository {},
        }
    }
}
