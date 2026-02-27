pub struct TableView<'a, T> {
    pub item: &'a T,
}

impl<'a, T> TableView<'a, T> {
    pub fn new(item: &'a T) -> Self {
        Self { item }
    }
}
