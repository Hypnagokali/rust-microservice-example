use core::fmt;
use std::{sync::{Arc, Mutex}};
use slab::Slab;

pub type UserId = u64;

#[derive(Debug, Copy, Clone)]
pub struct UserData {
    pub id: UserId
}

pub type UserDb = Arc<Mutex<Slab<UserData>>>;

/// InMemoryDataSource<T>
/// Umgesetzt mit einem Slab Array
pub struct InMemoryDataSource<T> {
    pub data_source: Arc<Mutex<Slab<T>>>,
}

pub trait BasicRepository<T> {
    fn find_by_id(&self, id: u64) -> Result<T, &'static str>;
    fn save(&self, data: &T) -> Result<T, &'static str>;
}


impl BasicRepository<UserData> for InMemoryDataSource<UserData> {
    fn find_by_id(&self, id: u64) -> Result<UserData, &'static str> {
        let user_data = self.data_source.lock().unwrap();

        if user_data.contains(id as usize) {
            Ok(user_data[id as usize])
        } else {
            Err("User not found")
        }
    }

    fn save(&self, user_data: &UserData) -> Result<UserData, &'static str> {
        let mut saved_user = user_data.clone();
        let user_id = self.data_source.lock().unwrap().insert(saved_user);
        saved_user.id = user_id as u64;

        Ok(saved_user)
    }
}

impl fmt::Display for UserData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let user_obj_str = format!("{{ \"id\":{} }}\n", self.id);

        f.write_str(&user_obj_str)
    }
}