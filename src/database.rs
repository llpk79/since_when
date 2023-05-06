use crate::events::EventOccurrence;
use log::{error, info};
use rusqlite::{params, Connection, Result};

/// Setup rusqlite connection.
///
/// # Returns
/// - `rusqlite::Connection`
pub fn setup_connection() -> Connection {
    match Connection::open("since_when.db") {
        Ok(conn) => conn,
        Err(e) => {
            panic!("Error opening data_base {}", e);
        }
    }
}

/// Prepare a SQL statement.
///
/// # Arguments
/// - conn: `&Connection`
/// - stmt: `&str`
///
/// # Returns
/// - `rusqlite::Statement`
pub fn prepare_stmt<'a>(conn: &'a Connection, stmt: &'a str) -> rusqlite::Statement<'a> {
    match conn.prepare(stmt) {
        Ok(statement) => statement,
        Err(e) => {
            panic!("Error preparing statement: {}", e);
        }
    }
}

/// Setup the data_base tables.
///
/// # Arguments
/// - `&Connection`
///
/// # Returns
/// - `()`
pub fn setup_tables(conn: &Connection) {
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS events (
              id              INTEGER PRIMARY KEY,
              name            TEXT NOT NULL UNIQUE
              );",
        params![],
    ) {
        Ok(_) => {
            info!("Created table events.");
        }
        Err(e) => {
            error!("Error creating table: {}", e);
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
            info!("Created table occurrences.");
        }
        Err(e) => {
            error!("Error creating table: {}", e);
        }
    }
}

/// Insert test data into the data_base.
///
/// # Arguments
/// - `&Connection`
///
/// # Returns
/// - `()`
pub fn insert_test_event(conn: &Connection) {
    match conn.execute(
        "INSERT INTO events (name) VALUES (?1), (?2);",
        params!["Pooper empty", "Propane tank full"],
    ) {
        Ok(inserted) => {
            info!("Record inserted: {}", inserted);
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
            Ok(inserted) => info!("Record inserted: {}", inserted),
            Err(e) => error!("Error inserting record: {}", e),
        }
        }
        Err(e) => error!("Error inserting record: {}", e),
    }
}

/// Get events and occurrences from the data_base.
///
/// # Arguments
/// - conn - `&Connection`
///
/// # Returns
/// - `Result<Vec<EventOccurrence>>`
pub fn get_events(conn: &Connection) -> Result<Vec<EventOccurrence>> {
    info!("Retrieving Records.");
    // Get all events and occurrences.
    let mut stmt = prepare_stmt(
        conn,
        "\
    SELECT name, date \
    FROM events \
    JOIN occurrences \
    ON events.id = occurrences.event_id \
    ORDER BY date DESC;",
    );
    let event_iter = stmt.query_map([], |row| {
        Ok(EventOccurrence {
            name: row.get(0)?,
            date: row.get(1)?,
        })
    })?;
    let mut events = Vec::new();
    for event in event_iter {
        events.push(match event {
            Ok(event) => event,
            Err(e) => {
                error!("Error retrieving record: {}", e);
                EventOccurrence {
                    name: "".to_string(),
                    date: "".to_string(),
                }
            }
        });
    }
    Ok(events)
}

/// Perform a SQL insert with variable parameters.
///
/// ### Arguments
/// - conn: `&rusqlite::Connection` - The data_base connection.
/// - id: `(i32, bool)` - The id of the event to insert.
/// - date: `(&str, bool)` - The date of the occurrence to insert.
/// - event: `(&str, bool)` - The name of the event to insert.
///     - The bool portion of the tuple is a flag to determine if the parameter should be used.
/// - sql: `&str` - The SQL statement to execute.
///
/// ### Returns
/// - `Result<i32, rusqlite::Error>` - bool success flag.
pub fn sql_insert(
    conn: &Connection,
    id: (i32, bool),
    date: (&str, bool),
    event: (&str, bool),
    sql: &str,
) -> Result<i32, rusqlite::Error> {
    let mut stmt = prepare_stmt(conn, sql);
    // Match on the flags to determine which parameters to use.
    match (id.1, date.1, event.1) {
        // Update event with a new occurrence.
        (true, true, false) => match stmt.execute(params![id.0, date.0]) {
            Ok(success) => success,
            Err(e) => {
                error!("Error: {:?}", e);
                0
            }
        },
        // Add event/delete occurrence.
        (false, false, true) => match stmt.execute(params![event.0]) {
            Ok(success) => success,
            Err(e) => {
                error!("Error: {:?}", e);
                0
            }
        },
        // Delete event.
        (true, false, false) => match stmt.execute(params![id.0]) {
            Ok(success) => success,
            Err(e) => {
                error!("Error: {:?}", e);
                0
            }
        },
        _ => 0, // This should never happen.
    };
    Ok(id.0)
}

/// Get the id of the event.
///
/// # Arguments
/// - conn: `&rusqlite::Connection` - The data_base connection.
///
/// # Returns
/// - id: `i32` - The id of the event.
pub fn get_event_id(conn: &Connection, event: &str) -> i32 {
    struct ID {
        id: i32,
    }
    info!("Getting event id for {:?}", event);
    let mut id_stmt = prepare_stmt(conn, "SELECT id FROM events WHERE name = ?1;");
    let ID { id } = match id_stmt.query_row(params![event], |row| Ok(ID { id: row.get(0)? })) {
        Ok(id) => id,
        Err(e) => {
            error!("Error: {:?}", e);
            ID { id: 0 }
        }
    };
    id
}

/// Add an event to the data_base.
///
/// ### Arguments
/// - event: `&str` - The name of the event to add.
/// - date: `&str` - The date of the occurrence to add.
///
/// ### Returns
/// - `()`
pub fn add_event(event: &str, date: &str) {
    let conn = setup_connection();
    match sql_insert(
        &conn,
        (0, false),
        ("", false),
        (event, true),
        "INSERT INTO events (name) VALUES (?1);",
    ) {
        Ok(_) => {
            info!("Event added: {:?}", event);
            let id = get_event_id(&conn, event);
            // Add the occurrence to the data_base.
            match sql_insert(
                &conn,
                (id, true),
                (date, true),
                ("", false),
                "INSERT INTO occurrences (event_id, date) VALUES (?1, ?2);",
            ) {
                Ok(_) => {
                    info!("Occurrence added: {}, {}", event, date);
                }
                Err(e) => {
                    error!("Error: {:?}", e);
                }
            };
        }
        // If the event already exists, do not add the occurrence.
        Err(e) => {
            error!("Error: {:?}", e);
        }
    }
}

/// Delete an event from the data_base.
///
/// ### Arguments
/// - event: `&str` - The name of the event to delete.

/// ### Returns
/// - `()`
pub fn delete_event(event: &str) {
    let conn = setup_connection();
    let id = get_event_id(&conn, event);
    // Delete occurrence.
    match sql_insert(
        &conn,
        (id, true),
        ("", false),
        ("", false),
        "DELETE FROM occurrences WHERE event_id = ?1;",
    ) {
        Ok(_) => {
            info!("Occurrences deleted.");
            // Delete event.
            match sql_insert(
                &conn,
                (0, false),
                ("", false),
                (event, true),
                "DELETE FROM events WHERE name = ?1;",
            ) {
                Ok(_) => {
                    info!("Event deleted: {}", event);
                }
                Err(e) => {
                    error!("Error: {:?}", e);
                }
            }
            id
        }
        Err(e) => {
            error!("Error: {:?}", e);
            0
        }
    };
}

/// Update an event in the data_base.
///
/// ### Arguments
/// - event: `&str` - The name of the event to update.
/// - date: `&str` - The date of the occurrence to update.
///
/// ### Returns
/// - `()`
pub fn update_event(event: &str, date: &str) {
    let conn = setup_connection();
    let id = get_event_id(&conn, event);
    // Add the occurrence to the data_base.
    match sql_insert(
        &conn,
        (id, true),
        (date, true),
        ("", false),
        "INSERT INTO occurrences (event_id, date) VALUES (?1, ?2);",
    ) {
        Ok(_) => {
            info!("Occurrence added: {} on {}", event, date);
        }
        Err(e) => {
            error!("Error: {:?}", e);
        }
    };
}
