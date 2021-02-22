use std::cell::RefCell;

use crate::domain::repositories::MothRepository;

pub struct InMemoryMonthsRepository {
    months: Vec<&'static str>,
    current_key: RefCell<i8>,
    current_select: RefCell<i8>,
}

impl InMemoryMonthsRepository {
    pub fn new() -> InMemoryMonthsRepository {
        let mut _months: Vec<&'static str> = vec![
            "Januari",
            "Februari",
            "Maret",
            "April",
            "Mei",
            "Juni",
            "Juli",
            "Agustus",
            "September",
            "Oktober",
            "November",
            "Desember"
        ];

        return InMemoryMonthsRepository {
            months: _months,
            current_key: RefCell::new(-1),
            current_select: RefCell::new(0),
        }
    }

    #[warn(unused_assignments)]
    fn convert_key(&self, key: i8) -> i8 {
        let result: i8;
        let cur_key = *self.current_key.borrow();
        let cur_select = *self.current_select.borrow();

        if cur_key == key {
            if cur_select == 0 && (key == 3 || key == 7) {
                result = key / 3;
            }
            else {
                result = 0;
            }
        }
        else  {
            result = key / 3;
        }
        *self.current_key.borrow_mut() = key;
        *self.current_select.borrow_mut() = result;
        result
    }
}

impl MothRepository for InMemoryMonthsRepository {
    fn by_key(&self, key: i8) -> Result<&'static str, String> {
        let idx = self.convert_key(key);

        let val = self.months[idx as usize];
        Ok(val)
    }
}
