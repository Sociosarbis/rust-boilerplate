#[allow(dead_code)]
struct ParkingSystem {
  spaces: [i32;3]
}


impl ParkingSystem {
    #[allow(dead_code)]
    fn new(big: i32, medium: i32, small: i32) -> Self {
      ParkingSystem {
        spaces: [big, medium, small]
      }
    }
    #[allow(dead_code)]
    fn add_car(&mut self, car_type: i32) -> bool {
      if self.spaces[(car_type - 1) as usize] > 0 {
        self.spaces[(car_type - 1) as usize] -= 1;
        true
      } else {
        false
      }
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    spaces: [i32;3],
    cars: Vec<i32>,
    ret: Vec<bool>
  }

  #[test]
  fn parking_system_simple() {
    let suites = vec![
      Suite {
        spaces: [1, 1, 0],
        cars: vec![1, 2, 3, 1],
        ret: vec![true, true, false, false]
    }];
    for s in suites {
      let mut p = ParkingSystem::new(s.spaces[0], s.spaces[1], s.spaces[2]);
      for i in 0..s.cars.len() {
        assert_eq!(p.add_car(s.cars[i]), s.ret[i]);
      }
    }
  }
}