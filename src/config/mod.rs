pub mod global_config {
    pub const USER_PATH: &str = "/user/";
}

pub mod server_config {
    use std::net::SocketAddr;

    use crate::data::user::{UserDb};

    pub struct DataBaseServer {
        pub addr: SocketAddr,
        pub user_db: UserDb,
    }

}