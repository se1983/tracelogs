#[derive(Debug, Clone)]
pub struct  LogLineToken {
    pub raw_data: String
}

impl LogLineToken{
    pub fn new(data: &str)-> Self{
        LogLineToken{
            raw_data: String::from(data)
        }
    }
}