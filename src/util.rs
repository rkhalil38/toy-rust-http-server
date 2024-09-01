use std::{fs::File, io::{self, BufRead}, path::Path};

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {

    let file =  match File::open(filename) {
        Ok(file) => file,
        Err(err) => panic!("COULD NOT OPEN FILE: {}", err)
    };
    Ok(io::BufReader::new(file).lines())
}

pub fn stringify_file(path: &str) -> String {
    let mut payload = String::new();

    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            payload.push_str(&line);
        }
    }

    return payload;

}

pub fn extract_path(request: String) -> String {
    let mut capture = false;
    let mut path = String::new();
    for ch in request.chars() {
        if ch == ' ' && capture == true {
            break;
        }

        if ch == '/' {
            capture = true;
        }

        if capture {
            path.push_str(&ch.to_string());
        }
    }

    return path;
}