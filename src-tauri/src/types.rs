pub enum ResponseItem<T> {
    Single(T),
    Multiple(Vec<T>),
}

pub struct KittyResponse<T> {
    pub data: Option<ResponseItem<T>>,
    pub code: i8,
    pub msg: Option<String>,
}

impl<T> KittyResponse<T> {
    pub fn new(code: i8, data: Option<ResponseItem<T>>, msg: Option<String>) -> Self {
        Self { code, data, msg }
    }

    pub fn from_msg(code: i8, msg: &str) -> Self {
        Self {
            data: None,
            code,
            msg: Some(msg.to_string()),
        }
    }
}

impl<T> Default for KittyResponse<T> {
    fn default() -> Self {
        Self {
            data: None,
            code: Default::default(),
            msg: None,
        }
    }
}
