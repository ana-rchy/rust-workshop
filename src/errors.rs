use std::fs::File;

fn errors() {
    let file_result = File::open("CIA List of Domestic Terrorists.txt");

    let file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("{error:?}"),
    };

    let file2 = File::open("game_dev_soc Plans for the Assasination of Donald Trump.txt").unwrap();
    let file3 = File::open("game_dev_soc South American Escape Plans.txt").expect("oops");

    panic!("fuck you. crashes");
}
