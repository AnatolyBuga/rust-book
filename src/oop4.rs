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
}

/// Defines behaviour shared by all three states: Draft, PendingReview, Published
trait State {}

struct Draft {}
struct PendingReview {}
struct Published {}

impl State for Draft {}
impl State for PendingReview {}
impl State for Published {}