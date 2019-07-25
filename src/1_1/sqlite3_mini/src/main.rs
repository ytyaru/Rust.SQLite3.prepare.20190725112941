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

    let mut cursor = connection
        .prepare("SELECT * FROM users WHERE age > ?")
        .unwrap()
        .cursor();
    cursor.bind(&[sqlite3::Value::Integer(50)]).unwrap();

    while let Some(row) = cursor.next().unwrap() {
        println!("name = {}", row[0].as_string().unwrap());
        println!("age = {}", row[1].as_integer().unwrap());
    }
}

