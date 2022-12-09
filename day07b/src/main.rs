use std::collections::HashMap;
use regex::Regex;

fn main() {
  let input = std::fs::read_to_string("input.txt").unwrap();
  let root = Directory::build_root(input);

  root.print();

  // root.for_each_directory(|name, dir| {
  //   println!("{} (size={})", name, dir.size())
  // });

  println!("{}", root.answer());
}

#[derive(Debug)]
struct Directory {
  directories: HashMap<String, Directory>,
  files: HashMap<String, File>
}
impl Directory {
  fn new() -> Directory {
    Directory { directories: HashMap::new(), files: HashMap::new() }
  }

  fn build_root(input: String) -> Directory {
    let mut root = Directory::new();
    let mut pwd: Vec<String> = vec![];

    let instructions = [
                         Regex::new(r"\$ cd /").unwrap(),
                         Regex::new(r"\$ cd \.\.").unwrap(),
                         Regex::new(r"\$ cd (\w+)").unwrap(),
                         Regex::new(r"\$ ls").unwrap(),
                         Regex::new(r"dir (\w+)").unwrap(),
                         Regex::new(r"(\d+) (\w+(?:\.\w+)?)").unwrap()
                       ];

    for line in input.lines() {
      if instructions[0].is_match(line) {
        pwd.clear();
      }

      if instructions[1].is_match(line) {
        pwd.pop();
      }

      if let Some(caps) = instructions[2].captures(line) {
        let name = caps.get(1).unwrap().as_str().to_string();
        // TODO - maybe check the dir exists in current, but skipping for now.

        pwd.push(name);
      }

      if instructions[3].is_match(line) {
        // skip. we can actually just ignore the ls and just read what comes after.
      }

      if let Some(caps) = instructions[4].captures(line) {
        let name = caps.get(1).unwrap().as_str().to_string();
        let current: &mut Directory = pwd.iter().fold(&mut root, |dir, n| dir.directories.get_mut(n).unwrap());

        current.add_directory(name);
      }

      if let Some(caps) = instructions[5].captures(line) {
        let size: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        let name = caps.get(2).unwrap().as_str().to_string();
        let current: &mut Directory = pwd.iter().fold(&mut root, |dir, n| dir.directories.get_mut(n).unwrap());

        current.add_file(name, File::new(size));
      }
    }

    return root;
  }

  fn add_directory(&mut self, name: String) {
    self.directories.insert(name, Directory::new());
  }

  fn add_file(&mut self, name: String, file: File) {
    self.files.insert(name, file);
  }

  fn print(&self) {
    println!("- / (dir)");
    self.__print__(1);
  }
  fn __print__(&self, level: usize) {
    for (name, dir) in self.directories.iter() {
      println!("{}- {} (dir)", " ".repeat(level * 2), name);
      dir.__print__(level + 1);
    }

    for (name, file) in self.files.iter() {
      println!("{}- {} (file, size={})", " ".repeat(level * 2), name, file.size);
    }
  }

  fn size(&self) -> usize {
    self.files.values().map(|f| f.size).sum::<usize>() +
      self.directories.values().map(|d| d.size()).sum::<usize>()
  }

  // fn for_each_directory<F>(&self, f: F) where F: Fn(&str, &Directory) {
  //   self.__for_each_directory__("/", &f);
  // }
  // fn __for_each_directory__<F>(&self, name: &str, f: &F) where F: Fn(&str, &Directory) {
  //   f(name, self);

  //   for (n, d) in self.directories.iter() {
  //     d.__for_each_directory__(n, f);
  //   }
  // }

  fn answer(&self) -> usize {
    if (70_000_000 - self.size()) > 30_000_000 {
      panic!("we don't need to delete anything...");
    }
    let needed: usize = 30_000_000 - (70_000_000 - self.size());

    let mut sizes: Vec<usize> = vec![];
    self.__answer__(needed, &mut sizes);

    *sizes.iter().min().unwrap()
  }
  fn __answer__(&self, needed: usize, sizes: &mut Vec<usize>) {
    let s = self.size();
    if s > needed {
      sizes.push(s);
    }

    for (_, dir) in self.directories.iter() {
      dir.__answer__(needed, sizes);
    }
  }
}

#[derive(Debug)]
struct File {
  size: usize
}
impl File {
  fn new(size: usize) -> File {
    File { size }
  }
}
