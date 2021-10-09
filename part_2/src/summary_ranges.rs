struct SummaryRanges {
 intervals: Vec<Vec<i32>>
}


impl SummaryRanges {

    fn new() -> Self {
      return SummaryRanges {
        intervals: vec![]
      };
    }
    
    fn add_num(&mut self, val: i32) {
      let mut l = 0;
      if !self.intervals.is_empty() {
        let mut r = self.intervals.len() - 1;
        while l <= r {
          let mid = (l + r) / 2;
          if val + 1 < self.intervals[mid][0] {
            if mid < 1 {
              break;
            } else {
              r = mid - 1;
            }
          } else if val - 1 > self.intervals[mid][1] {
            l = mid + 1;
          } else {
            if val < self.intervals[mid][0] {
              self.intervals[mid][0] = val;
              if mid > 0 {
                if self.intervals[mid - 1][1] + 1 == val {
                  self.intervals[mid - 1][1] = self.intervals[mid][1];
                  self.intervals.remove(mid);
                }
              }
            } else if val > self.intervals[mid][1] {
              self.intervals[mid][1] = val;
              if mid + 1 < self.intervals.len() {
                if self.intervals[mid + 1][0] - 1 == val {
                  self.intervals[mid][1] = self.intervals[mid + 1][1];
                  self.intervals.remove(mid + 1);
                }
              }
            }
            return;
          }
        }
      }
      if l < self.intervals.len() {
        self.intervals.insert(l, vec![val, val]);
      } else {
        self.intervals.push(vec![val, val]);
      }
    }
    
    fn get_intervals(&self) -> Vec<Vec<i32>> {
      return self.intervals.to_owned();
    }
}