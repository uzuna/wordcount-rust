use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

/// 数える対象を指定するオプション
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CountOption {
  /// 文字数
  Char,

  /// ワード数
  Word,

  /// 行数
  Line,
}

impl Default for CountOption {
  fn default() -> Self {
    CountOption::Word
  }
}

pub fn count(input: impl BufRead, option: CountOption) -> HashMap<String, usize> {
  let re = Regex::new(r"\w+").unwrap();
  let mut fregs = HashMap::new();

  for line in input.lines() {
    let line = line.unwrap();
    use crate::CountOption::*;
    match option {
      Char => {
        for c in line.chars() {
          *fregs.entry(c.to_string()).or_insert(0) += 1;
        }
      }
      Word => {
        for m in re.find_iter(&line) {
          let word = m.as_str().to_string();
          *fregs.entry(word).or_insert(0) += 1;
        }
      }
      Line => *fregs.entry(line).or_insert(0) += 1,
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

  let fregs = count(reader, CountOption::default());

  assert_eq!(Some(&1), fregs.get("aa"));
  assert_eq!(Some(&2), fregs.get("bb"));
  assert_eq!(Some(&2), fregs.get("cc"));
  assert_eq!(Some(&1), fregs.get("dd"));
}
