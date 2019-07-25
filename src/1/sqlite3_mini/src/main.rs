fn main() {
    let connection = sqlite3::open("/tmp/work/db.sqlite3").unwrap();

    // 以下のようにパニックしたらテーブル作成コードをコメントアウトする。
    // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error { code: Some(1), message: Some("table users already exists") }'
    connection
        .execute(
            "
            CREATE TABLE users (name TEXT, age INTEGER);
            INSERT INTO users (name, age) VALUES ('Alice', 42);
            INSERT INTO users (name, age) VALUES ('Bob', 69);
            ",
        )
        .unwrap();

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

