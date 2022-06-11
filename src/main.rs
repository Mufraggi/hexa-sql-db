
fn main() {
   let db  = format!("postgres://{}:{}@{}:{}/{}", "postgres", "somePassword", "localhost", "5432", "postgres");
    println!("{}", db);
}
