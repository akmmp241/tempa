pub struct PageRequest<C> {
    pub after: Option<C>,
    pub limit: i16,
    pub q: Option<String>,
    pub status: Option<String>,
}