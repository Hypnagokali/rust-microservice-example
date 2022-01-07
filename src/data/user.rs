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
    fn find_by_id(&self, id: u64) -> Result<T, String>;
    fn save(&self, data: &T) -> Result<T, String>;
}


impl BasicRepository<UserData> for InMemoryDataSource<UserData> {
    fn find_by_id(&self, id: u64) -> Result<UserData, String> {
        let user_data = self.data_source.lock().unwrap();

        if id < 1 {
            return Err(String::from("ID < 1"));  
        }

        let id_in_slab = id - 1;

        if user_data.contains(id_in_slab as usize) {
            Ok(user_data[id_in_slab as usize])
        } else {
            Err(format!("User with ID = {} not found", id))
        }
    }

    fn save(&self, user_data: &UserData) -> Result<UserData, String> {
        let mut saved_user = user_data.clone();

        let mut locked_data_source = self.data_source.lock().unwrap();

        if saved_user.id > 0 {
            // update user
            let update_user = locked_data_source.get_mut((user_data.id - 1) as usize);

            if let Some(user) = update_user {
                *user = saved_user;
    
                Ok(saved_user)
            } else {
                Err(format!("User not found ID = {}", saved_user.id))
            }
        } else {
            // new user
            let user_id = locked_data_source.insert(saved_user);

            let updated_saved_user = locked_data_source.get_mut(user_id).unwrap();
            updated_saved_user.id = (user_id + 1) as u64;

            Ok(*updated_saved_user)
        }
    }
}

impl fmt::Display for UserData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let user_obj_str = format!("{{ \"id\":{} }}\n", self.id);

        f.write_str(&user_obj_str)
    }
}