use core::panic;
use std::{collections::HashMap, io::empty, path::Path};

use regex::{Captures, Regex};

pub struct FileDTO {
    name: String,
    size: usize,
}

pub struct Directory {
    sub_dirs: Vec<String>,
    files: Vec<String>,
    size: usize,
}

impl Directory {
    fn new() -> Directory {
        return Directory {
            sub_dirs: vec![],
            files: vec![],
            size: 0,
        };
    }
}

impl Directory {
    fn add_sub_dir(&mut self, sub_dir_path: String) {
        self.sub_dirs.push(sub_dir_path);
    }
}

impl Directory {
    fn add_file(&mut self, file_path: String) {
        self.files.push(file_path);
    }
}

pub struct FileSystem {
    current_dir: String,
    dir_index: HashMap<String, Directory>,
    file_index: HashMap<String, usize>,
}

impl FileSystem {
    pub fn new() -> FileSystem {
        let mut empty_fs = FileSystem {
            current_dir: "".to_string(),
            dir_index: HashMap::<String, Directory>::new(),
            file_index: HashMap::<String, usize>::new(),
        };
        empty_fs.add_sub_dir(&"/".to_string());
        return empty_fs;
    }
}

impl FileSystem {
    pub fn add_sub_dirs(&mut self, subdirs: &mut Vec<String>) {
        match self.dir_index.contains_key(&self.current_dir) {
            true => {
                for subdir in subdirs.into_iter() {
                    self.add_sub_dir(subdir);
                }
            }
            false => panic!(
                "ERROR : directory index does not contain key : {}",
                &self.current_dir
            ),
        }
    }
}

impl FileSystem {
    fn add_sub_dir(&mut self, subdir: &String) {
        let sub_dir_path = self.extend_current_dir(subdir);

        let new_empty_dir = Directory::new();
        self.add_dir_to_index(sub_dir_path.clone(), new_empty_dir);

        self.dir_index
            .get_mut(&self.current_dir)
            .unwrap()
            .add_sub_dir(sub_dir_path);
    }
}

impl FileSystem {
    pub fn add_sub_files(&mut self, files: &Vec<FileDTO>) {
        for fDTO in files.into_iter() {
            self.add_sub_file(fDTO);
        }
        self.update_fs_sizes();
    }
}

impl FileSystem {
    fn add_sub_file(&mut self, file: &FileDTO) {
        let file_path = self.extend_current_dir(&file.name);

        self.add_file_to_index(file_path.clone(), 0);

        match self.dir_index.get_mut(&self.current_dir) {
            Some(dir) => dir.add_file(file_path),
            None => panic!(
                "ERROR : tried to add files to non-existing directory : {}",
                &self.current_dir
            ),
        }
    }
}

impl FileSystem {
    fn add_dir_to_index(&mut self, file_path: String, dir: Directory) {
        self.dir_index.insert(file_path, dir);
    }
}

impl FileSystem {
    fn add_file_to_index(&mut self, file_path: String, file_size: usize) {
        self.file_index.insert(file_path, file_size);
    }
}

impl FileSystem {
    fn update_fs_sizes(&mut self) {
        for dir in Path::new(&self.current_dir).ancestors().into_iter() {
            let path = &dir.to_str().unwrap().to_string();
            let new_dir_size = self.calculate_dir_size(&path);

            match self.dir_index.get_mut(path) {
                Some(dir) => dir.size = new_dir_size,
                None => panic!("ERROR : {path} does not exist in the directory index"),
            }
        }
    }
}

impl FileSystem {
    fn calculate_dir_size(&self, path: &String) -> usize {
        let mut total: usize = 0;
        match self.dir_index.get(path) {
            Some(dir) => {
                for sub_dir in &dir.sub_dirs {
                    total += self.dir_index.get(sub_dir).unwrap().size;
                }
                for file in &dir.files {
                    total += self.file_index.get(file).unwrap();
                }
            }
            None => panic!("ERROR : tried to calculate non-existing directory size : {path}"),
        }
        total
    }
}

impl FileSystem {
    fn extend_current_dir(&self, path: &String) -> String {
        Path::new(self.current_dir.as_str())
            .join(path)
            .to_str()
            .unwrap()
            .to_string()
    }
}

impl FileSystem {
    fn get_parent(&self) -> String {
        Path::new(self.current_dir.as_str())
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }
}

impl FileSystem {
    fn cd(&mut self, d: &String) {
        match d.as_str() {
            current => {}
            back => self.current_dir = self.get_parent(),
            _ => self.current_dir = self.extend_current_dir(d),
        }
    }
}

pub struct Cmd {
    verb: String,
    arg: String,
    result: String,
}

impl Cmd {
    pub fn new(cap: Captures) -> Cmd {
        let mut lines = cap[0].lines();
        let (v, arg) = lines.next().unwrap().split_at(5);
        let verb = v.split_at(2).1;

        return Cmd {
            verb: verb.to_string(),
            arg: arg.to_string(),
            result: lines.collect(),
        };
    }
}

const cd: &str = "cd";
const ls: &str = "ls";
const back: &str = "..";

pub fn solution1(content: &String) -> usize {
    let mut fs = FileSystem::new();

    for command in Regex::new(r"\$(.|\n^[^\$])+")
        .unwrap()
        .captures_iter(content)
    {
        let cmd = Cmd::new(command);
        match cmd.verb.as_str() {
            cd => match cmd.arg.as_str() {
                back => match Path::new(fs.current_dir.as_str()).parent() {
                    Some(_) => fs.cd(&back.to_string()),
                    None => {}
                },
                _ => fs.cd(&cmd.arg),
            },
            ls => {
                let mut subdirs: Vec<String> = cmd
                    .result
                    .lines()
                    .filter(|&line| line.contains("dir"))
                    .map(|d| d.split_at(4).1.to_string())
                    .collect();
                fs.add_sub_dirs(&mut subdirs);

                let files = cmd
                    .result
                    .lines()
                    .filter(|&line| Regex::new(r"\d").unwrap().is_match(line));
            }
            x => panic!("ERROR parsing command, found {x}"),
        }

        // If it's a cd just change the current directory
        // If it's a ls
        // remove the first line
        // filter the lines in two groups
        // lines that start with a number
        // parse them as fileDTO
        // add them as files to the current directory
        // lines that start with 'dir'
        // parse them to remove the 'dir' and keep only the name
        // add them as sub_dirs to the current directory
        // else do nothing or print a little warning
    }

    0 // return the size of fs.dir_index.get("/").size
}

pub fn solution2(content: &String) -> usize {
    0
}
