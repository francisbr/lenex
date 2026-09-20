//! LENEX primitive value formats.

#[allow(unused_imports)]
use time::Date;

time::serde::format_description!(pub(crate) date, Date, "[year]-[month]-[day]");
