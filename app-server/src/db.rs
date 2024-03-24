use rusqlite::Connection;

//create database
pub fn get_db_conn() -> Connection{
    let conn = Connection::open("restaurant.db").expect("failed to open SQlite connection");
    conn
}

pub fn initialize_db(){
    println!("Initialising the database..");
    let conn = Connection::open("restaurant.db").expect("failed to open SQlite connection");
    conn.execute("PRAGMA foreign_keys= ON;", []).expect("Failed to enable foreign key support");
    
    println!("Create Table Table");
    create_table_table_if_not_exists(&conn).expect("Failed to create Table orders");

    println!("Create Menu Table");
    create_menu_table_if_not_exists(&conn).expect("Failed to create Table orders")

    println!("Create Order Table");
    create_order_table_if_not_exists(&conn).expect("Failed to create Table orders")
    
    println!("Create OrderItem Table");
    create_order_item_table_if_not_exists(&conn).expect("Failed to create Table orders")

}

fn_create_table_table_if_not_exists(conn: &Connetion) -> rusqlite::Result<()>{
    conn.execute("CREATE TABLE IF NOT EXISTS tables(id INTEGER PRIMARY KEY, code TEXT NOT NULL UNIQUE)", [])?;
  OK(())  

}

fn_create_menu_table_if_not_exists(conn: &Connetion) -> rusqlite::Result<()>{
    conn.execute("CREATE TABLE IF NOT EXISTS menus(id INTEGER PRIMARY KEY, name TEXT NOT NULL)", [])?;
    OK(())  

}

fn_create_order_table_if_not_exists(conn: &Connetion) -> rusqlite::Result<()>{
    conn.execute("CREATE TABLE IF NOT EXISTS orders(id INTEGER PRIMARY KEY, table_id INTEGER TEXT NOT NULL, FOREIGN_KEY(table_id) REFERENCE tables(id), UNIQUE(table_id))", [])?;
    OK(())  

}


fn_create_order_item_table_if_not_exists(conn: &Connetion) -> rusqlite::Result<()>{
    conn.execute("CREATE TABLE IF NOT EXISTS order_items(id INTEGER PRIMARY KEY, order_id INTEGER TEXT NOT NULL, menu_id INTEGER NOT NULL, cooking_time NOT NULL, quantity INTEGER NOT NULL default1 ,FOREIGN_KEY(order_id) REFERENCE tables(id), FOREIGN_KEY(menus_id) REFERENCE menus(id))", [])?;
    OK(())  

}

 