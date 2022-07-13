// We get "warning: static is never used: `STATIC_STR`"
// STATIC_STR is defined in static_stuff.rs
// Since we don't use anything from static_stuff directly
// but STATIC_STR is used via print_static_str in lib.rs
// we would expect a "unused import" warning instead.
// Removing the mod import here will fix the warning.
mod static_stuff;

fn main() {
    unused_test::print_static_str();
}
