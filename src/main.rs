use clap::{App, Arg, SubCommand};
use rusqlite::{Connection, Result, NO_PARAMS};

struct Todo {
    id: i32,
    text: String,
    completed: bool,
}

fn add_item(text: &str, conn: &Connection) -> Result<()> {
    conn.execute(
        "INSERT INTO Todo (text, completed) VALUES (?1, 0)",
        &[text.to_string()],
    )?;
    Ok(())
}

fn show_items(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM Todo")?;
    let todo_iter = stmt.query_map(NO_PARAMS, |row| {
        Ok(Todo {
            id: row.get(0)?,
            text: row.get(1)?,
            completed: row.get(2)?,
        })
    })?;

    for todo in todo_iter {
        let todo = todo?;
        println!(
            "id: {}, text: {}, completed: {}",
            todo.id, todo.text, todo.completed
        );
    }

    Ok(())
}

fn main() -> Result<()> {
    // prepare
    let conn = Connection::open("todo.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Todo (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            text TEXT NOT NULL,
            completed BOOL NOT NULL
        )",
        NO_PARAMS,
    )?;

    let matches = App::new("myapp")
        .subcommand(
            SubCommand::with_name("add")
                .about("Add todo.")
                .arg(Arg::with_name("text").index(1).required(true)),
        )
        .subcommand(SubCommand::with_name("show"))
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("add") {
        let todo_text = matches.value_of("text").unwrap();
        match add_item(todo_text, &conn) {
            Ok(_) => println!("Item added: {}", todo_text),
            Err(_) => println!("Something went wrong. please try again."),
        }
    }
    if let Some(..) = matches.subcommand_matches("show") {
        show_items(&conn)?;
    }

    Ok(())
}
