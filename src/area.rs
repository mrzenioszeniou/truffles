pub enum Area {
    Ammochostos,
    Larnaka,
    Lefkosia,
    Limassol,
    Paphos,
}

impl Area {
    pub fn all() -> Box<[Self]> {
        Box::new([
            Area::Ammochostos,
            Area::Larnaka,
            Area::Lefkosia,
            Area::Limassol,
            Area::Paphos,
        ])
    }
}
