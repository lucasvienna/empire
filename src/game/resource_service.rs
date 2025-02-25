use std::fmt;

use crate::db::resources::ResourcesRepository;
use crate::db::DbConn;

pub struct ResourceService {
    connection: DbConn,
    res_repo: ResourcesRepository,
}

impl fmt::Debug for ResourceService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ResourceService")
    }
}

impl ResourceService {
    pub fn new(connection: DbConn) -> ResourceService {
        ResourceService {
            connection,
            res_repo: ResourcesRepository {},
        }
    }
}
