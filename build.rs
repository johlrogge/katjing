extern crate skeptic;

use skeptic::*;

fn main() {
    // add all markdown files in directory book/
    let mdbook_files = markdown_files_of_directory("book/");
    generate_doc_tests(&mdbook_files);
}
