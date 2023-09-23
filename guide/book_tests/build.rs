use skeptic::*;

fn main() {
    let book_dir = std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join("src");
    let book_src_path_str = format!("{}", book_dir.display());
    let mdbook_files = markdown_files_of_directory(&book_src_path_str);
    generate_doc_tests(&mdbook_files);
}
