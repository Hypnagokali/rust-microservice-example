pub trait BasicRepository<T, ID> {
    fn find_by_id(&self, id: ID) -> Result<T, String>;
    fn save(&self, data: &T) -> Result<T, String>;
    fn delete(&self, data: &T) -> Result<(), String>;
    fn delete_by_id(&self, id: ID) -> Result<(), String>;
}