impl Solution {
  pub fn first_uniq_char(s: String) -> i32 {
      let mut dict: [i32;26] = [0;26];
      let mut indices: [i32;26] = [-1;26];
      for (i, c) in s.chars().enumerate() {
        let index = c as usize - 97;
        if dict[index] == 0 {
          indices[index] = i as i32; 
        }
        
        if dict[index] < 2 {
          dict[index] += 1;
        }
      }
      let mut ret = -1;
      for i in 0..26 {
        if dict[i] == 1 && (indices[i] < ret || ret == -1) {
          ret = indices[i];
        }
      }
      ret
  }
}