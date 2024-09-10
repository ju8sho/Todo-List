use rusqlite::Connection;
use std::io::{self, Write};

use crate::db;

pub fn run(ulanish: &Connection) {

    loop {
       println!("1. Vazifa qo'shish");
       println!("2. Vazifalarni ko'rish"); 
       println!("3. Chiqish");
       print!("Tanlovni kiriting: ");
       io::stdout().flush().unwrap();  

       let mut tanlash = String::new();
       io::stdin().read_line(&mut tanlash).expect("Vazifalarni kiritishda xatolik");
       let tanlash = tanlash.trim();

       match tanlash {
            "1" => add_vazifa_ui(ulanish),
            "2" => vazifalar_toplami_ui(ulanish),
            "3" => break,
            _ => println!("Notug'ri tanlov qayta urinib ko'ring")
       }
    }
}

fn add_vazifa_ui(ulanish: &Connection) {
    println!("Vazifalarni kiriting: ");
    let mut tavsifi = String::new();
    io::stdin().read_line(&mut tavsifi).unwrap();
    let tavsifi = tavsifi.trim();

    db::add_data(ulanish, tavsifi).expect("Vazifa qo'shishda xatolik yuz berdi");
    println!("Vazifa qo'shildi");
}

fn vazifalar_toplami_ui(ulanish: &Connection) {
    let vazifalar = db::get_vazifa(ulanish).expect("Vazifalarni olishda xatolk");
    for vazifa in vazifalar {
        let status = if vazifa.bajarilgan {"Bajarilgan"} else {"Bajarilmagan"};
        println!("{}: {} [{}]", vazifa.id, vazifa.tavsifi, status);
    }
    
} 