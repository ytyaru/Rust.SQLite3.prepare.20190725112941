fn main() {
    let connection = sqlite3::open("/tmp/work/db.sqlite3").unwrap();

    if let Some(row) = connection.prepare("select count(*) from sqlite_master where type='table' and name='users';").unwrap().cursor().next().unwrap() {
        if 0 == row[0].as_integer().unwrap() {
            connection
                .execute(
                    "
                    CREATE TABLE users (name TEXT, age INTEGER);
                    INSERT INTO users (name, age) VALUES ('Alice', 42);
                    INSERT INTO users (name, age) VALUES ('Bob', 69);
                    ",
                )
                .unwrap();
        }
    }

    let mut statement = connection
        .prepare("SELECT * FROM users WHERE age > ?")
        .unwrap();
    statement.bind(1, 50).unwrap();

    while let sqlite3::State::Row = statement.next().unwrap() {
        println!("name = {}", statement.read::<String>(0).unwrap());
        println!("age = {}", statement.read::<i64>(1).unwrap());
    }
}

