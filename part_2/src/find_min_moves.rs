use super::*;

impl Solution {
  pub fn find_min_moves(machines: Vec<i32>) -> i32 {
    let sum = machines.iter().fold(0, |acc, num| acc + num);
    if sum % machines.len() as i32 != 0 {
      return -1;
    }
    let average = sum / machines.len() as i32;
    let mut ret = 0;
    let mut diff = 0;
    for i in 0..machines.len() {
      // 各台机器初始与平均的差值N，如果为正，则表示经过该位置的衣服数最小值，因为它至少需要从这个位置派发N件衣服
      // 但为负不能代表衣服的流过的数量，因为该位置为整条路径的终点，不占操作次数
      let v1 = machines[i] - average; 
      // 累计从左到当前位置的衣服累积量，如果为负则表示流入经此处的数量，如果为正，则表示流出经此处的数量
      // 以[4,4,0,0]为例子，减去平均值变为[2,2,-2,-2]
      // 0: 2件流向右，操作数为2，数量变为0
      // 1: 得到左边2件，变为4件，需要4次操作，将衣服流向右邻
      // 2：得到4件后，变为2件，所以需要2次操作，将多余的衣服流向右邻
      // 3: 数量为0，状态平衡
      // 这个问题，可以动态地认为每个点每时刻都会处理流经它的衣服，如果已到平均数，则转发衣服，如果未到平均数，则保留流经的衣服
      // 例如例子中的第二个点，即便由于从左边有2件衣服流入，但衣服的流入是不影响，它本身衣服并行流出
      // 所以只需要知道每个点转发衣服的操作次数，它们的最大值就是答案
      diff += v1;
      let v = if diff >= 0 { diff } else { -diff };
      if v > ret {
        ret = v;
      }
      if v1 > ret {
        ret = v1;
      }
    }
    ret
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  struct Suite {
    machines: Vec<i32>,
    ret: i32
  }

  #[test]
  fn test_find_min_moves_simple() {
    let suites = vec![
      Suite {
        machines: vec![1,0,5],
        ret: 3
      },
      Suite {
        machines: vec![0,3,0],
        ret: 2
      },
      Suite {
        machines: vec![0,2,0],
        ret: -1
      },
      Suite {
        machines: vec![4,0,0,4],
        ret: 2
      },
      Suite {
        machines: vec![4,4,0,0],
        ret: 4
      },
      Suite {
        machines: vec![100000,0,100000,0,100000,0,100000,0,100000,0,100000,0],
        ret: 50000
      },
      Suite {
        machines: vec![0,0,10,0,0,0,10,0,0,0],
        ret: 8
      },
      Suite {
        machines: vec![0,0,4,0,5,0,5,0,4],
        ret: 4
      },
      Suite {
        machines: vec![56,1,2,3,15,54,18,36,69,56],
        ret: 78
      },
      Suite {
        machines: vec![4,9,8,4,0],
        ret: 6
      }
    ];

    for s in suites {
      assert_eq!(s.ret, Solution::find_min_moves(s.machines));
    }
  }
}