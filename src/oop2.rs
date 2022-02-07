//! We can add to what's loaded from gui

//use gui::drawer::Screen; - too long to write, use below
use super::oop3::{Draw, Screen, Button};
use super::oop4::{Post};

struct SelectBox{
    w: u32,
    opt: Vec<String>,
    name: String
}

impl Draw for SelectBox{
    fn draw(&self) {
        // code to draw
        println!("Drawing TextField {}", self.name)
    }
}

///create Screen
pub fn part_one() {
    let s = Screen {
        components: vec![
            Box::new(SelectBox{w: 75, name: String::from("Anatoly's SelectBox"), opt: vec![
                String::from("Yes"), String::from("No")
            ]}),
            Box::new(Button{w: 50, h: 10, label: String::from("Box")})
        ]
    };
    s.run()
}

pub fn part_two() {
    let mut post = Post::new(); //new blog
    post.add_text("I ate pizza for lunch today");
    assert_eq!("", post.content()); //not req review yet
    post.request_review();
    assert_eq!("", post.content()); //not reviewed/approved yet
    //post.approve();
    //assert_eq!("I ate pizza for lunch today", post.content());
    }