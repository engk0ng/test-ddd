
pub trait MothRepository {
    fn by_key(&self, key: i8) -> Result<&'static str, String>;
}