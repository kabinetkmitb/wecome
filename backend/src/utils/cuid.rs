use cuid;

pub fn get_cuid() -> String {
    cuid::cuid().unwrap()
}
