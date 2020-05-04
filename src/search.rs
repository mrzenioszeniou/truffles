pub trait Search {
    fn search(from: &str) -> Option<Self>
        where Self: std::marker::Sized;
}