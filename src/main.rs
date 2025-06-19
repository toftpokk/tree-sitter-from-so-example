use libloading::{Library, Symbol};
use tree_sitter::{Language, Parser};

fn main() {
    let library_path = "./rust.so";
    let language_name = "rust";
    let library = unsafe { Library::new(&library_path).expect("Unable to load dynamic library") };
    let language: Language = unsafe {
        let language_name = format!("tree_sitter_{}", language_name.replace('-', "_"));
        let language_fn: Symbol<unsafe extern "C" fn() -> Language> = library
            .get(language_name.as_bytes())
            .expect("Failed to load symbol");
        language_fn()
    };

    let mut parser = Parser::new();
    parser
        .set_language(&language)
        .expect("Unable to create parser");
    let tree = parser.parse("fn main() {}", None).unwrap();
    let stdout = std::io::stdout();
    tree.print_dot_graph(&stdout);
}
