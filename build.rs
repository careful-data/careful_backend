fn main() {
    println!("cargo:rustc-env=DATABASE_URL=postgres://user:password@localhost/database_name");
}
