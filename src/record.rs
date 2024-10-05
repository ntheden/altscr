use regex::Regex;

pub type Records = Vec<Record>;

#[derive(Debug)]
pub struct Record {
    pub data: String,
    pub y_size: usize,
    pub username: String,
    pub date: String, // todo: should be date object
    pub privacy: bool, // nip4 or nip17
}

impl Record {
    fn new() -> Self {
        Self {
            data: "".to_string(),
            y_size: 1,
            username: "user".to_string(),
            date: "".to_string(),
            privacy: true, // not even sure if i will allow nip 4
        }
    }

    pub fn from_str(data: String) -> Self {
        let y_size = Record::count_newlines(&data);
        Self {
            data,
            y_size: y_size.try_into().unwrap(),
            username: "user".to_string(), // todo: later
            date: "".to_string(), // todo: later
            privacy: true, // not even sure if i will allow nip 4
        }
    }

    pub fn count_newlines(text: &str) -> usize {
        let re = Regex::new(r"\n").unwrap();
        re.find_iter(text).count() + 1 // size is 1-based
    }

}
