use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

pub fn count(input: impl BufRead) -> HashMap<String, usize> {
  let re = Regex::new(r"\w+").unwrap();
  let mut fregs = HashMap::new();

  for line in input.lines() {
    let line = line.unwrap();
    for m in re.find_iter(&line) {
      let word = m.as_str().to_string();
      *fregs.entry(word).or_insert(0) += 1;
    }
  }
  fregs
}

#[test]
fn read_testfile() {
  use std::fs::File;
  use std::io::BufReader;
  let file = File::open("./src/tests/word.txt").unwrap();
  let reader = BufReader::new(&file);

  let fregs = count(reader);

  assert_eq!(Some(&1), fregs.get("aa"));
  assert_eq!(Some(&2), fregs.get("bb"));
  assert_eq!(Some(&2), fregs.get("cc"));
  assert_eq!(Some(&1), fregs.get("dd"));
}
