
pub struct Vazifa {
    pub id: i32,
    pub tavsifi: String,
    pub bajarilgan: bool,
}

impl Vazifa {
    pub fn new(id: i32, tavsifi: String, bajarilgan: bool) -> Vazifa {
        Vazifa {id, tavsifi, bajarilgan}
    }

    pub fn bajarildi_deb_belgila(&mut self) {
        self.bajarilgan = true;
    }
}