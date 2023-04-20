use rusqlite::{Connection, Result, params};

/// Setup rusqlite connection.
///
/// # Returns
/// - Result<Connection, rusqlite::Error>
pub fn setup_connection() -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open("since_when.db")?;
    Ok(conn)
}

/// Setup the data_base tables.
///
/// # Arguments
/// - &Connection
///
/// # Returns
/// - ()
pub fn setup_tables(conn: &Connection) {
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS events (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL UNIQUE
                  );",
        params![],
    ) {
        Ok(_) => {
            println!("Created table events.");
        }
        Err(e) => {
            println!("Error creating table: {}", e);
        }
    }
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS occurrences (
                  event_id        INTEGER,
                  date            TEXT NOT NULL,
                  FOREIGN KEY(event_id) REFERENCES events(id)
                  );",
        params![],
    ) {
        Ok(_) => {
            println!("Created table occurrences.");
        }
        Err(e) => {
            println!("Error creating table: {}", e);
        }
    }
}

/// Insert test data into the data_base.
///
/// # Arguments
/// - &Connection
///
/// # Returns
/// - ()
pub fn insert_test_event(conn: &Connection) {
    match conn.execute(
        "INSERT INTO events (name) VALUES (?1), (?2);",
        params!["Pooper empty", "Propane tank full"],
    ) {
        Ok(inserted) => {
            println!("Record inserted: {}", inserted);
            // Insert test occurrence.
            match conn.execute(
                "INSERT INTO occurrences (event_id, date) VALUES (?1, ?2), (?3, ?4), (?5, ?6), (?7, ?8);",
                params![
                    1i32,
                    "2023-04-01".to_string(),
                    2i32,
                    "2023-04-12".to_string(),
                    1i32,
                    "2023-04-06".to_string(),
                    1i32,
                    "2023-04-11".to_string(),
                ],
            ) {
                Ok(inserted) => println!("Record inserted: {}", inserted),
                Err(e) => println!("Error inserting record: {}", e),
            }
        }
        Err(e) => println!("Error inserting record: {}", e),
    }
}
