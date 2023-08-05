//! <https://houqp.github.io/leptess/leptess/index.html>

use std::path::Path;

fn get_tesseract_search_paths() -> Vec<String> {
    let mut search_places: Vec<String> = vec![("./".to_string())];
    if cfg!(target_os = "macos") {
        search_places.push("/opt/homebrew/Cellar/tesseract/".to_string());
    };
    search_places
}

fn find_tesseract_path() -> Option<String> {
    for place in get_tesseract_search_paths() {
        if !Path::new(&place).exists() {
            continue;
        }
        // search the place for a tesseract folder
        for path in glob::glob(&format!("{}/**", place)).unwrap() {
            if let Ok(path) = path {
                if path.ends_with("share/tessdata") {
                    return Some(path.to_str().unwrap().to_string());
                };
            }
        }
    }

    None
}

fn main() {
    // get the filename from the first argv
    let filename = match std::env::args().nth(1) {
        Some(val) => val,
        None => {
            eprintln!("no filename given");
            return;
        }
    };

    // if the file doesn't exist then exit
    if !std::path::Path::new(&filename).exists() {
        eprintln!("file {} does not exist", filename);
        return;
    }

    let tesspath = find_tesseract_path();
    if tesspath.is_none() {
        eprintln!("tesseract not found, please install it, looked for 'share/tessdata' in the following places: {}", get_tesseract_search_paths().join(", "));
        return;
    }

    let mut lt = match leptess::LepTess::new(Some(&tesspath.unwrap()), "eng") {
        Ok(lt) => lt,
        Err(err) => panic!("failed to load tessdata: {:?}", err),
    };
    lt.set_image(filename).unwrap();
    let text = match lt.get_utf8_text() {
        Ok(text) => text,
        Err(err) => {
            eprintln!("failed to get text: {:?}", err);
            return;
        }
    };

    let ok_chars = ['@', '\'', '"', '‘', '’'];
    // strip all letter characters but leave in @
    let text = text
        .chars()
        .filter(|c| c.is_alphanumeric() || ok_chars.contains(c) || c.is_whitespace())
        .collect::<String>();

    println!("{}", text.replace('\n', " "));
}
