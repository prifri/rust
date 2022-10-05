
//fn largest<T>(list: &[T]) -> T {
//    let mut largest = list[0];
//
//    for &i in list.iter() {
//        if largest < i {
//            largest = i;
//        }
//    }
//    largest
//}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    fn abc<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

pub trait Summarizable {
    fn summary(&self) -> String {
        String::from("not implement")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
//    fn summary(&self) -> String {
//        format!("{}, by {} ({})", self.headline, self.author, self.content)
//    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{} :{}", self.username, self.content)
    }
}

fn main() {
    //let v = vec![1, 2, 3, 4, 5];
    //println!("{}", largest(&v));

    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 10.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    println!("{} {}", both_integer.x, both_integer.y);
    println!("{}", both_integer.x());

    let p1 = Point { x: 5, y: 10};
    let p2 = Point { x: "abc", y: 'c'};
    let p3 = p1.abc(p2);

    println!("{} {}", p3.x, p3.y);

    let tweet = Tweet {
        username: String::from("ab"),
        content: String::from("12"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summary());

    let news_article = NewsArticle {
        headline: String::from("ab"),
        location: String::from("12"),
        author: String::from("aba"),
        content: String::from("kkkk"),
    };

    println!("1 new article: {}", news_article.summary());
}
