use super::*;

use std::collections::HashMap;

impl Solution {
  pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
    let mut letter_counter = [0;26];

    for &letter in &letters {
      letter_counter[letter as usize - 97] += 1;
    }
    let mut dp: HashMap<i32, [i32;26]> = HashMap::new();
    dp.insert(0, letter_counter.clone());
    for (i, word) in words.iter().enumerate() {
      let mut word_letter_count = [0;26];
      for c in word.chars() {
        word_letter_count[c as usize - 97] += 1;
      }
      let mut temp_dp: HashMap<i32, [i32;26]> = HashMap::new();
      'a: for (&k, v) in &dp {
        for i in 0..26 {
          if v[i] < word_letter_count[i] {
            continue 'a;
          }
        }
        let mut next_counter = [0;26];
        for i in 0..26 {
          next_counter[i] = v[i] - word_letter_count[i];
        }
        temp_dp.insert(k | (1 << i), next_counter);
      }
      dp.extend(temp_dp);
    }
    let mut ret = 0;
    for (_, counter) in dp {
      let mut temp = 0;
      for i in 0..26 {
        if score[i] > 0 && counter[i] < letter_counter[i] {
          temp += (letter_counter[i] - counter[i]) * score[i];
        }
      }
      if temp > ret {
        ret = temp;
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    words: Vec<String>,
    letters: Vec<char>,
    score: Vec<i32>,
    ret: i32
  }

  #[test]
  fn test_max_score_words_simple() {
    let suites = vec![
      Suite {
        words: t1!["dog","cat","dad","good"],
        letters: vec!['a','a','c','d','d','d','g','o','o'],
        score: vec![1,0,9,5,0,0,3,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0],
        ret: 23
      },
      Suite {
        words: t1!["xxxz","ax","bx","cx"],
        letters: vec!['z','a','b','c','x','x','x'],
        score: vec![4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5,0,10],
        ret: 27
      },
      Suite {
        words: t1!["leetcode"],
        letters: vec!['l','e','t','c','o','d'],
        score: vec![0,0,1,1,1,0,0,0,0,0,0,1,0,0,1,0,0,0,0,1,0,0,0,0,0,0],
        ret: 0
      },
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::max_score_words(s.words, s.letters, s.score));
    }
  }
}