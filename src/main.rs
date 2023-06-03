fn main() {
    let mut lt = match leptess::LepTess::new(
        Some("/opt/homebrew/Cellar/tesseract/5.3.1/share/tessdata"),
        "eng",
    ) {
        Ok(lt) => lt,
        Err(err) => panic!("failed to load tessdata: {:?}", err),
    };
    lt.set_image("./tests/tweet-home-depot.png").unwrap();
    let text = match lt.get_utf8_text() {
        Ok(text) => text,
        Err(err) => {
            eprintln!("failed to get text: {:?}", err);
            return;
        }
    };
    println!("{}", text.replace('\n', " "));
}
