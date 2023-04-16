use super::method;

pub struct Response {
    path: String, 
    query_string: Option<String>, 
    method: method::Method
}
impl Response {
    pub fn new(path: String, query_string: Option<String>, method: method::Method) -> Self {
        Response {
            path,
            query_string,
            method
        }
    }
}
