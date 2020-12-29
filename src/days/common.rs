use std::io::{self, BufRead};
use std::{
    env,
    fs::{read_dir, File},
};

pub fn file_to_lines(filename: &str) -> Result<Vec<String>, &str> {
    let path: String;
    if filename.starts_with('/') {
        path = filename.into();
    } else {
        let base_dir = String::from(env::current_exe().unwrap().to_str().unwrap());
        let base_dir: Vec<&str> = base_dir.split('/').collect();
        let base_dir = base_dir[0..base_dir.len() - 3].join("/");
        path = base_dir + "/src/" + filename;
    }

    if let Ok(file) = File::open(&path) {
        let lines = io::BufReader::new(file).lines();
        let lines: Vec<String> = lines.collect::<Result<_, _>>().unwrap();
        return Ok(lines);
    }
    eprintln!("couldn't open file {}", path);
    Err("invalid filename")
}

#[allow(dead_code)]
pub fn test_files(prefix: &str, method: impl Fn(&[String], bool) -> i64, mod_path: &str) {
    let mut current_path = String::from(env::current_dir().unwrap().to_str().unwrap());
    let mod_slices: Vec<&str> = mod_path.split("::").collect();
    if mod_slices.len() >= 3 {
        current_path = format!("{}/src/{}/tests", current_path, &mod_slices[1..3].join("/"));
    }

    //println!("{}", current_path);
    if let Ok(paths) = read_dir(current_path) {
        for path in paths {
            let path = path.unwrap();
            let name = String::from(path.file_name().to_str().unwrap());
            if name.starts_with(prefix) && name.ends_with(".txt") {
                let parts: Vec<&str> = name.split('-').collect();
                let expected: i64 = match parts[2].parse() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("malformed test file name: {}", name);
                        continue;
                    }
                };

                let full_path = String::from(path.path().to_str().unwrap());
                if let Ok(lines) = file_to_lines(&full_path) {
                    let res = method(&lines, false);
                    assert_eq!(expected, res);
                    println!("test: {}", full_path);
                }
            }
        }
    }
}

#[macro_export]
macro_rules! myTest {
    () => {
        #[cfg(test)]
        mod test {
            use crate::days::common::test_files;

            #[test]
            fn test_first() {
                test_files("p1", super::p1, module_path!());
            }

            #[test]
            fn test_second() {
                test_files("p2", super::p2, module_path!());
            }
        }
    };
}
