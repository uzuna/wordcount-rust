//! wordcountはシンプルな文字、単語、行の出現頻度の計数機能を提供します
//! 詳しくは[`count`](fn.count.html)関数のドキュメントを見てください
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

/// デフォルトは[`Word`](enum.CountOption.html#variant.Word)
impl Default for CountOption {
  fn default() -> Self {
    CountOption::Word
  }
}

/// # Example
/// 入力中の単語の頻度を数える例
///
/// ```
/// use std::io::Cursor;
/// use wordcount::{count, CountOption};
///
/// let mut input = Cursor::new("aa bb cc bb");
/// let freq = count(input, CountOption::Word);
///
/// assert_eq!(freq["aa"], 1);
/// ```
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

#[test]
fn word_count_works() {
  use std::io::Cursor;

  let mut exp = HashMap::new();
  exp.insert("aa".to_string(), 1);
  exp.insert("bb".to_string(), 2);
  exp.insert("cc".to_string(), 1);

  assert_eq!(count(Cursor::new("aa bb cc bb"), CountOption::Word), exp);
}

#[test]
fn return_result() -> std::io::Result<()> {
  Ok(())
}

/// panicするテストは`should_panic`アトリビュートを付ける
#[test]
#[should_panic]
fn is_panic() {
  panic!("panic!");
}

/// Mapの複数のKey-Valueペアに対してassertするマクロ
macro_rules! assert_map {
  ($expr: expr, {$($key: expr => $value:expr),*}) => {
    $(assert_eq!($expr[$key], $value));*
  };
}

#[test]
fn word_count_works3() {
  use std::io::Cursor;

  let freqs = count(Cursor::new("aa bb cc"), CountOption::Word);

  assert_map!(freqs, {"aa"=>1, "cc"=>1, "bb"=>1});
}
