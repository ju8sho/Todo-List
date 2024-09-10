use crate::db::initialize_db;

mod db;
mod struktura;
mod ui;


fn main() {
    // SQLite ma'lumotlar bazasiga ulanish
    let ulanish = initialize_db().expect("Malumotlar bazasiga ulanishda xatolik");

    //Foydalanuvchi interfeysini ishga tushirish
    ui::run(&ulanish);
}
