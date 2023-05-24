use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    time::Instant,
};

#[derive(Debug)]
struct SpecialFile {
    path: String,
    word_count: u32,
}

fn main() {
    let start = Instant::now();
    let files = Path::new("book/").read_dir().unwrap();
    let mut total_word_count: u32 = 0;
    let mut total_file_count: u32 = 0;
    let mut all_word_counts: Vec<u32> = Vec::new();
    let mut largest_file = SpecialFile {
        path: String::new(),
        word_count: 0,
    };
    let mut smallest_file = SpecialFile {
        path: String::new(),
        word_count: 0,
    };
    for file in files {
        let filepath = file.as_ref().unwrap().path();
        if filepath.extension() != Some(std::ffi::OsStr::new("html")) {
            continue;
        }
        let file_path = file.unwrap().path();
        let contents = read_file(&file_path);
        let word_count = count_words_in_file(contents);
        if word_count > largest_file.word_count {
            largest_file.path = file_path.display().to_string();
            largest_file.word_count = word_count;
        } else if word_count < smallest_file.word_count || smallest_file.word_count == 0 {
            smallest_file.path = file_path.display().to_string();
            smallest_file.word_count = word_count;
        }
        total_word_count += word_count;
        total_file_count += 1;
        all_word_counts.push(word_count);
        //println!("{}: {} words", file_path.display(), word_count);
    }

    let duration = start.elapsed();
    all_word_counts.sort();
    println!("Total: {} words", total_word_count);
    println!(
        "Largest file: {} with {} words",
        largest_file.path, largest_file.word_count
    );
    println!(
        "Smallest file: {} with {} words",
        smallest_file.path, smallest_file.word_count
    );
    println!(
        "Mean file size: {} words",
        total_word_count / total_file_count
    );
    println!(
        "Median file size: {} words",
        all_word_counts[all_word_counts.len() / 2]
    );
    println!("Time elapsed: {:?}", duration);
}

fn read_file(file_path: &Path) -> String {
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    contents
}

fn count_words_in_file(contents: String) -> u32 {
    let mut word_count = 0;
    for _ in contents.split_whitespace() {
        word_count += 1;
    }
    word_count
}
