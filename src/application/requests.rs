pub struct GetMonthRequest<'a> {
    pub key: &'a i8
}

impl<'a> GetMonthRequest<'a> {
    pub fn new(key: &'a i8) -> GetMonthRequest {
        return GetMonthRequest {
            key
        }
    }
}