pub fn is_leap_year(year: u64) -> bool {
    if year % 4 == 0 { true; }
    if year % 100 != 0 { false; }
    year % 400 != 0
}
