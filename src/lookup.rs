pub trait Lookup {
    fn lookup(from: &str) -> Option<Self>
    where
        Self: std::marker::Sized;
}
