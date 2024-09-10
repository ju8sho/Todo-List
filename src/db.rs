use rusqlite::{Connection, Result};

use crate::struktura::Vazifa;

//tablitsa yaratamiz
pub fn initialize_db() -> Result<Connection> {
    let ulanish = Connection::open("todo.db")?;
    ulanish.execute(
        "CREATE TABLE IF NOT EXISTS vazifalar (
                id INTEGER PRIMARY KEY,
                tavsifi TEXT NOT NULL,
                bajarilgan BOOLEAN NOT NULL DEFAULT 0
        )",
        [],
    )?;
    Ok(ulanish)

}

pub fn add_data(ulanish: &Connection, tavsifi: &str) -> Result<()> {
    ulanish.execute(
        "INSERT OR IGNORE INTO vazifalar (tavsifi) VALUES (?1)",
        [tavsifi]

    )?;
    Ok(())
}

pub fn get_vazifa(ulanish: &Connection) -> Result<Vec<Vazifa>> {
    let mut stmt = ulanish.prepare("SELECT id, tavsifi, bajarilgan FROM vazifalar")?;
    let vazifa_iter = stmt.query_map([],|row| {
        Ok(Vazifa {
            id: row.get(0).unwrap_or(0),
            tavsifi: row.get(1).unwrap_or_else(|_| "Nomalum vazifa".to_string()),
            bajarilgan: row.get(2).unwrap_or(false) // Null bo'lsa, bajarilmagan (false) deb hisoblash
        })
        
    })?;

    let mut vazifalar = Vec::new();
    for vazifa in vazifa_iter {
        vazifalar.push(vazifa?);
       
    }
    Ok(vazifalar)
}