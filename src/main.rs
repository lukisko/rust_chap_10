

fn main() {
    println!("Hello, world!");

    let poin = Point {x:10.0,y:20.0};
    println!("x part of point is {} and distance from origin is {}.",poin.x(),poin.distance_from_origin());
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &number in list {
        if number > largest {
            largest = number;
        }
    }
    return largest;
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

//trait is interface

pub trait Summary{
    fn summarize(&self) -> String{
        String::from("Read more...")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

//using default implementation
impl Summary for NewsArticle {}

//to return trait
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Sum {}

// to play what should types implement just ad plus and they need to implement both
pub trait Test {
    fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Summary + Clone,
          U: Clone + Sum;
}

//lifetime of input and output (shorter one is used)
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

