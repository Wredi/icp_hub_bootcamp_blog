use candid::CandidType;

#[derive(Clone, CandidType)]
pub struct Blog {
    title: String,
    date: u32,
    content: String,
    tags: Vec<String>
}

impl Blog {
    pub fn default() -> Self {
        Self {
            title: String::new(),
            date: 0,
            content: String::new(),
            tags: Vec::new()
        }
    }

    pub fn new(title: String, date: u32, content: String, tags: Vec<String>) -> Self {
        Self {
            title,
            date,
            content,
            tags
        }
    }
}
