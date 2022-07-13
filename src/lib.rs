mod static_stuff;

pub fn print_static_str() {
    println!("{}", static_stuff::STATIC_STR);
}
