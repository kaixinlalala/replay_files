extern crate time;
// assertion failed
fn main() {
    let _local0 = time::UtcDateTime::now();
    let _local1 = time::UtcDateTime::weekday(_local0);
    let _ = time::Date::from_iso_week_date(9999 ,52 ,_local1);
}