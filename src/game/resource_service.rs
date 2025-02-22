use std::fmt;

use crate::db::resources::ResourcesRepository;
use crate::db::DbConn;

pub struct ResourceService<'a> {
    connection: &'a mut DbConn,
    res_repo: ResourcesRepository,
}

impl fmt::Debug for ResourceService<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ResourceService")
    }
}

impl ResourceService<'_> {
    pub fn new(connection: &mut DbConn) -> ResourceService {
        ResourceService {
            connection,
            res_repo: ResourcesRepository {},
        }
    }
}
