pub struct Post{
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post{
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        ""
    }
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            println!("{:?}", self.state);
            //take takes value out of the option leaving None inside
            //then we call request_review on Box holding dyn State (because fn def in State)
            self.state = Some(s._request_review()); //consumes current, returns new state, s can't be used anymore
        }
    }
}

/// Defines behaviour shared by all three states: Draft, PendingReview, Published
/// here Debug is supertrait
trait State: std::fmt::Debug {
    //method is only valid when called on a Box holding type
    //takes ownership of Box<Self>
    fn _request_review(self: Box<Self>) -> Box<dyn State>;
}
#[derive(Debug)]
struct Draft {}
#[derive(Debug)]
struct PendingReview {}
#[derive(Debug)]
struct Published {}

impl State for Draft {
    fn _request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview{})
    }
}
impl State for PendingReview {
    fn _request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
//impl State for Published {}