use super::method::Method;

pub struct Response {
    path: String, 
    query_string: Option<String>, 
    method: Method
}
impl Response {
    pub fn new(path: String, query_string: Option<String>, method: Method) -> Self {
        Response {
            path,
            query_string,
            method
        }
    }
}
