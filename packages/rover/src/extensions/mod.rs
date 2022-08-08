mod reply;

pub use self::reply::*;

pub trait Stringify {
    fn to_string(&self) -> String;
}
