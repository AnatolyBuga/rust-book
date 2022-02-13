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
        // as_ref() returns a ref to the value inside the option
        // unwrap here will never panic, because state contains Some by definition, None is not possible
        self.state.as_ref().unwrap()
        ._content(self)
        //deref coercion - content method ultimately called on dyn State
    }
    pub fn request_review(&mut self) {
        // CANNOT MOVE OUT OF A MUT REF
        //self.state = Some(self.state.unwrap()._request_review());
        //So we have to do this:
        // Notice &mut self, so we can take/mutate self
        //need to set state to None temporarily to get ownership of s
        if let Some(s) = self.state.take() {
            println!("{:?}", self.state);
            //take takes value out of the option leaving None inside
            //this lets us move s out of Post, rather than borrowing it
            self.state = Some(s._request_review()); 
            //consumes current, returns new state, s can't be used anymore
        }
    }
    /// Same as above, cannot move out of self, because it's a pointer 
    /// but can mutate it, so:
    pub fn approve(&mut self) {
        // take turns self.state into None
        if let Some(s) = self.state.take() {
            self.state = Some(s._approve());
        }
    }
}

/// Defines behaviour shared by all three states: Draft, PendingReview, Published
/// here Debug is supertrait. Whatever impl State must also impl Debug
trait State: std::fmt::Debug {
    //method is only valid when called on a Box holding Type
    //takes ownership of Box<Self>
    fn _request_review(self: Box<Self>) -> Box<dyn State>;
    fn _approve(self: Box<Self>) -> Box<dyn State>;
    fn _content<'a>(&self, post: &'a Post) -> &'a str{""}
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
    fn _approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
impl State for PendingReview {
    fn _request_review(self: Box<Self>) -> Box<dyn State> {
        // Not this, because here self is Box<Self> already
        //let a: Box<PendingReview> = self;
        self
    }
    fn _approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published{})
    }
}
impl State for Published {
    fn _request_review(self: Box<Self>) -> Box<dyn State> {self}
    fn _approve(self: Box<Self>) -> Box<dyn State> {self}
    fn _content<'a>(&self, post: &'a Post) -> &'a str{
        &post.content
    }
}

// Another possible design not to have method behaviour depend on state,
// but to implement different Types

pub struct Post2{
    _content: String,
}

pub struct DraftPost {
    _content: String,
}

pub struct PendingReviewPost{
    _content: String,
}

impl Post2 {
    pub fn new() -> DraftPost{
        DraftPost{_content: String::new()}
    }
    pub fn content(&self) -> &str {
        &self._content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self._content.push_str(text);
    }

    /// consume self to avoid lingering DraftPost
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost{
            _content: self._content,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post2 {
        Post2 {_content: self._content}
    }
}