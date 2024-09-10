use rusqlite::{Connection, Result};

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

pub fn get_vazifa(ulanish: &Connection) -> Result<Vec<(i32, String, bool)>> {
    let mut stmt = ulanish.prepare("SELECT id, tavsifi, bajarilgan FROM vazifalar")?;
    let vazifa_iter = stmt.query_map([],|row| {
        Ok((
            row.get(0)?, //id
            row.get(1)?, //tavsifi
            row.get(2)?, // vazifalarni bajarilganlik haqida
        ))
        
    })?;

    let mut vazifalar = Vec::new();
    for i in vazifa_iter {
        vazifalar.push(i?);
    }
    Ok(vazifalar)
}