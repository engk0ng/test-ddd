use crate::domain::repositories::MothRepository;

use super::requests::GetMonthRequest;

pub struct GetMonthHandler<'a> {
    month_repository: &'a dyn MothRepository
}

impl<'a> GetMonthHandler<'a> {
    pub fn new(month_repository: &dyn MothRepository) -> GetMonthHandler {
        return GetMonthHandler {
            month_repository
        }
    }

    pub fn execute(&self, request: GetMonthRequest) -> Result<&'static str, String> {
        return self.month_repository.by_key(*request.key);
    }
}