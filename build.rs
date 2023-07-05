/* Metaprogramming: Building the markdown posts by writing Rust code to a file*/
extern crate chrono;
use chrono::prelude::DateTime;
use chrono::Utc;
use std::time::{ UNIX_EPOCH, Duration };
use std::fs::{ metadata, File, read_dir };
use std::io::prelude::*;
use markdown::{to_html_with_options, CompileOptions, Options};

fn convert_to_html(path_to_markdown: &str) -> String {
    let mut file = File::open(path_to_markdown).unwrap();
    let mut file_string = String::new();
    file.read_to_string(&mut file_string).unwrap();
    to_html_with_options(&file_string, &Options {
        compile: CompileOptions {
            allow_dangerous_html: true,
            ..CompileOptions::default()
        },
        ..Options::default()
    }).unwrap()
}

fn main() {
    let mut output = File::create("./src/mdblogs.rs").unwrap();
    let output_str =
    "use phf::phf_map;

pub struct MarkdownBlog<'b> {
pub title: &'b str,
pub date_created: &'b str,
pub post: &'b str,
}

pub static BLOGS: phf::Map<&'static str, MarkdownBlog<'_>> = phf_map! {
    ".to_string();
// pub const BLOGS: &[MarkdownBlog] = &[
    output.write_all(output_str.as_bytes()).unwrap();
    println!("{}", output_str);
    let dir_entry = read_dir("./markdown_blogs").expect("Could not read directory...");
    for path in dir_entry {
        let file_path = path.unwrap().path().display().to_string();
        let md = metadata(&file_path).unwrap();
        let file_path_cloned = file_path.clone();
        let file_split = file_path_cloned.split('/').collect::<Vec<&str>>();
        let file_name = file_split.last().unwrap();
        let blog_name = file_name.split('.').collect::<Vec<&str>>();
        let blog_title = blog_name.first().unwrap();
        let time = md.created().unwrap().duration_since(UNIX_EPOCH).unwrap(); 
        let datetime = DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(time.as_secs()));
        let timestamp_str = datetime.format("%Y-%m-%d").to_string();
        let html_string = convert_to_html(&file_path);
        let output_str = format!(
        "
    \"{title}\" => MarkdownBlog {{
        title: \"{title}\",
        date_created: \"{date_created}\",
        post: r#####\"{post}\"#####,
    }},
        "
        , title=blog_title.trim(), date_created=timestamp_str.trim(), post=html_string.trim());
        println!("{}",output_str);
        output.write_all(output_str.as_bytes()).unwrap();
    }
    let output_str = "};".to_string();
    output.write_all(output_str.as_bytes()).unwrap();
}
