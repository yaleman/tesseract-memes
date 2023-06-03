//! <https://houqp.github.io/leptess/leptess/index.html>

fn main() {

    // get the filename from the first argv
    let filename = match std::env::args().nth(1){
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

    let mut lt = match leptess::LepTess::new(
        Some("/opt/homebrew/Cellar/tesseract/5.3.1/share/tessdata"),
        "eng",
    ) {
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

    let ok_chars = [
        '@',
        '\'',
        '"',
        '‘',
        '’',
    ];
    // strip all letter characters but leave in @
    let text = text
        .chars()
        .filter(|c| c.is_alphanumeric() || ok_chars.contains(c) || c.is_whitespace() )
        .collect::<String>();

    println!("{}", text.replace('\n', " "));
}
