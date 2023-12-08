pub enum ResponseItem<T> {
    Single(T),
    Multiple(Vec<T>),
}

pub struct KittyResponse<T> {
    pub data: Option<ResponseItem<T>>,
    pub code: i8,
    pub msg: String,
}
