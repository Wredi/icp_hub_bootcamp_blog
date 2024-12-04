use std::cell::RefCell;

use crate::blog::Blog;
use crate::config::Config;

mod blog;
mod config;

thread_local!{
    static CONFIG: RefCell<Config> = RefCell::new(Config::new());
    static BLOGS: RefCell<Vec<Blog>> = RefCell::new(Vec::new());
}

#[ic_cdk::update]
fn add_blog(title: String, content: String, tags: Vec<String>) -> Result<String, String> {
    let config = CONFIG.with(|config| config.borrow().clone());

    if title.len() > config.max_title_len as usize {
        return Err("Title is too long!".to_string());
    }

    if content.len() > config.max_content_len as usize {
        return Err("Content is too long!".to_string());
    }

    if tags.len() > config.max_tags_count as usize {
        return Err("Too much tags!".to_string());
    }

    BLOGS.with(|blogs| blogs.borrow_mut().push(
            Blog::new(title, content, tags))
        );

    Ok("All good".to_string())
}

#[ic_cdk::query]
fn get_blogs() -> Vec<Blog> {
    BLOGS.with(|blogs| blogs.borrow().clone())
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
