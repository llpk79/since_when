use crate::events::EventOccurrence;
use log::{error, info};
use rusqlite::{params, Connection, Result, Statement};
use std::collections::HashMap;

/// Setup rusqlite connection.
///
/// ### Returns
/// - `Connection` - The connection to the data_base.
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
/// ### Arguments
/// - conn: `&'a Connection` - The connection to the data_base.
/// - stmt: `&'a str` - The SQL statement to prepare.
///
/// ### Returns
/// - `Statement<'a>`
pub fn prepare_stmt<'a>(conn: &'a Connection, stmt: &'a str) -> Statement<'a> {
    match conn.prepare(stmt) {
        Ok(statement) => statement,
        Err(e) => {
            panic!("Error preparing statement: {}", e);
        }
    }
}

/// Setup the data_base tables.
///
/// ### Arguments
/// - `&Connection` - The connection to the data_base.
///
/// ### Returns
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
              year            INTEGER NOT NULL,
              month           INTEGER NOT NULL,
              day             INTEGER NOT NULL,
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
/// ### Arguments
/// - `&Connection` - The connection to the data_base.
///
/// ### Returns
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
            "INSERT INTO occurrences (event_id, year, month, day) VALUES (?1, ?2, ?3, ?4), (?5, ?6, ?7, ?8), (?9, ?10, ?11, ?12), (?13, ?14, ?15, ?16);",
            params![
                1i32,
                2023i32,
                4i32,
                1i32,
                2i32,
                2023i32,
                4i32,
                12i32,
                1i32,
                2023i32,
                4i32,
                6i32,
                1i32,
                2023i32,
                4i32,
                11i32,
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
/// ### Arguments
/// - conn - `&Connection` - The connection to the data_base.
///
/// ### Returns
/// - `Result<Vec<EventOccurrence>>` - The event occurrences.
pub fn get_events(conn: &Connection) -> Result<Vec<EventOccurrence>> {
    info!("Retrieving Records.");
    // Get all events and occurrences.
    let mut stmt = prepare_stmt(
        conn,
        "\
    SELECT name, year, month, day \
    FROM events \
    JOIN occurrences \
    ON events.id = occurrences.event_id \
    ORDER BY year, month, day DESC;",
    );
    let event_iter = stmt.query_map([], |row| {
        Ok(EventOccurrence {
            name: row.get(0)?,
            year: row.get(1)?,
            month: row.get(2)?,
            day: row.get(3)?,
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
                    year: 0,
                    month: 0,
                    day: 0,
                }
            }
        });
    }
    Ok(events)
}

/// Perform a SQL insert with variable parameters.
///
/// ### Arguments
/// - conn: `&Connection` - The data_base connection.
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
    date: (i32, u32, u32, bool),
    event: (&str, bool),
    sql: &str,
) -> Result<i32, rusqlite::Error> {
    let mut stmt = prepare_stmt(conn, sql);
    // Match on the flags to determine which parameters to use.
    match (id.1, date.3, event.1) {
        // Update event with a new occurrence.
        (true, true, false) => match stmt.execute(params![id.0, date.0, date.1, date.2]) {
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
/// ### Arguments
/// - conn: `&Connection` - The data_base connection.
/// - event: `&str` - The name of the event.
///
/// ### Returns
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
pub fn add_event(event: &str, year: i32, month: u32, day: u32) {
    let conn = setup_connection();
    match sql_insert(
        &conn,
        (0, false),
        (year, month, day, false),
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
                (year, month, day, true),
                ("", false),
                "INSERT INTO occurrences (event_id, year, month, day) VALUES (?1, ?2, ?3, ?4);",
            ) {
                Ok(_) => {
                    info!("Occurrence added: {}, {}-{}-{}", event, year, month, day);
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
///
/// ### Returns
/// - `()`
pub fn delete_event(event: &str) {
    let conn = setup_connection();
    let id = get_event_id(&conn, event);
    // Delete occurrence.
    match sql_insert(
        &conn,
        (id, true),
        (0, 0, 0, false),
        ("", false),
        "DELETE FROM occurrences WHERE event_id = ?1;",
    ) {
        Ok(_) => {
            info!("Occurrences deleted.");
            // Delete event.
            match sql_insert(
                &conn,
                (0, false),
                (0, 0, 0, false),
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
pub fn update_event(event: &str, year: i32, month: u32, day: u32) {
    let conn = setup_connection();
    let id = get_event_id(&conn, event);
    // Add the occurrence to the data_base.
    match sql_insert(
        &conn,
        (id, true),
        (year, month, day, true),
        ("", false),
        "INSERT INTO occurrences (event_id, year, month, day) VALUES (?1, ?2, ?3, ?4);",
    ) {
        Ok(_) => {
            info!("Occurrence added: {} on {}-{}-{}", event, year, month, day);
        }
        Err(e) => {
            error!("Error: {:?}", e);
        }
    };
}

/// Get events by year and month.
///
/// ### Returns
/// - `Result<HashMap<i32, Vec<String>>>` `{day: [event,...]}`
pub fn events_by_year_month(year: i32, month: u32) -> Result<HashMap<u32, Vec<String>>> {
    let conn = setup_connection();
    let mut stmt = prepare_stmt(
        &conn,
        "\
        SELECT e.name, o.day \
        FROM events e \
        JOIN occurrences o \
        ON e.id = o.event_id \
        WHERE o.year = ? and o.month = ?;",
    );
    struct EventDay {
        name: String,
        day: u32,
    }
    let event_iter = stmt.query_map(params![year, month as i32], |row| {
        Ok(EventDay {
            name: row.get(0)?,
            day: row.get(1)?,
        })
    })?;
    let mut events_by_year_month: HashMap<u32, Vec<String>> = HashMap::new();
    for event_result in event_iter {
        let event = match event_result {
            Ok(event) => event,
            Err(e) => {
                error!("Error getting record {}", e);
                EventDay {
                    name: "".to_string(),
                    day: 0,
                }
            }
        };
        if events_by_year_month.contains_key(&event.day) {
            let event_vec = match events_by_year_month.get_mut(&event.day) {
                Some(event_vec) => event_vec,
                None => {
                    error!("Error getting event vector");
                    continue;
                }
            };
            event_vec.push(event.name);
        } else {
            events_by_year_month.insert(event.day.clone(), vec![event.name.clone()]);
        }
    }
    Ok(events_by_year_month)
}
