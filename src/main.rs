use rust_toturial::{NewArticle, SocialPost, Summary};



fn main() {
   let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false
   };

   println!("1 new social post: {}", post.summarize());

   let article = returns_summarizable();

   println!("new article is {}", article.summarize());
}

fn returns_summarizable() -> impl Summary {
   NewArticle {
      headline: String::from("hello"),
      location: String::from("hello"),
      author: String::from("hello"),
      content: String::from("hello")
   }
}



