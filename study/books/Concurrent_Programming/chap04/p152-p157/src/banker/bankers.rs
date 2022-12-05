/*
 * prifri, 2022.12.05:
 * - NRES
 *   리소스양
 * - NTH
 *   thread 개수
 * - available[i]
 *   i 리소스에서 사용가능한 개수
 * - allocation
 *   thread가 확보중인 리소스
 * - max
 *   thread가 필요로 하는 리소스의 최대값.
 */

pub struct Resource<const NRES: usize, const NTH: usize> {
    available: [usize; NRES],
    allocation: [[usize; NRES]; NTH],
    max: [[usize; NRES]; NTH],
}

impl<const NRES: usize, const NTH: usize> Resource<NRES, NTH> {
    pub fn new(idx: usize, max: [[usize; NRES]; NTH]) -> Self {
        Resource {
            available: max[idx],
            allocation: [[0; NRES]; NTH],
            max,
        }
    }

/*
 * prifri, 2022.12.05:
 * - @return 데드락이나 굶주림에 빠지지 않은경우 return true.
 *
 */
    fn is_safe(&self) -> bool {
        let mut finish = [false; NTH];
        let mut work = self.available.clone();

        loop {
            let mut found = false;
            let mut num_true = 0;
            let mut false_is_exist = false;

            for (i, alc) in self.allocation.iter().enumerate() {
                if finish[i] {
                    num_true += 1;
                    continue;
                }

/*
 * prifri, 2022.12.05:
 * - need = self.max[i].iter().zip(alc).map(|(m, a)| m - a)
 *   m = &max[i];
 *   a = &alc;
 *   for(j = 0; j < NTH; j++)
 *   {
 *      need[j] = m[j] - a[j];
 *   }
 *
 * - 해당 thread가 가용할수있는 각 resource의 양을 가져온다.
 */
                let need = self.max[i].iter().zip(alc).map(|(m, a)| m - a);
/*
 * prifri, 2022.12.05:
 * - is_avail = work.iter().zip(need).all(|(w, n)| *w >= n);
 * for (j = 0; j < NTH; j++)
 * {
 *      if (work[j] < need[j])
 *      {
 *          break;
 *      }
 * }
 *
 * is_avail = j == NTH;
 *
 * - 해당 thread의 resource들이 전부 확보할수있는지 검사한다.
 * - 가능하다면 found가 된거고, allocation만큼 resource들으 가중하고 종료한다.
 */
                let is_avail = work.iter().zip(need).all(|(w, n)| *w >= n);
                if is_avail {
                    found = true;
                    finish[i] = true;
                    num_true += 1;
                    for (w, a) in work.iter_mut().zip(alc) {
                        *w += *a
                    }
/*
 * prifri, 2022.12.05:
 * - 모든 resource에 대해서 못찾으면 어짜피 그전에 한번이라도 found를
 * 못햇으면 다음것들을 검사할 필요없으니 break.
 * 가 맞을수도 있지만 sleep을 안쓸거면 어짜피 spin이라 필요없긴하다.
 */
                    if false_is_exist {
                        break;
                    }
                    continue;
                }

                false_is_exist = true;
            }

            if num_true == NTH {
                return true;
            }

            if !found {
                return false;
            }

/*
 * IAMROOT, 2022.12.05:
 * - sleep을 넣으면 cpu가 spin을 심하게 안돌긴하지만 실행시간이 수백배
 * 느려지는거같다.
 */
            //std::thread::sleep(std::time::Duration::from_nanos(100));
        }
    }

/*
 * prifri, 2022.12.05:
 * - [id][resource]에 resource 한개를 확보한다.
 */
    pub fn take(&mut self, id: usize, resource: usize) -> bool {
        if id >= NTH || resource >= NRES || self.available[resource] == 0 {
            return false;
        }

        self.allocation[id][resource] += 1;
        self.available[resource] -= 1;

        if self.is_safe() {
            true
        } else {
            self.allocation[id][resource] -= 1;
            self.available[resource] += 1;
            false
        }
    }

    pub fn release(&mut self, id: usize, resource: usize) {
        if id >= NTH || resource >= NRES || self.allocation[id][resource] == 0 {
            return;
        }

        self.allocation[id][resource] -= 1;
        self.available[resource] += 1;
    }

    pub fn show(&mut self, id: usize, is_take: bool, success: bool) {
        print!("{}{}{} | ", id, if is_take { "take" } else { "rele"},
               if success { "O" } else { "X" });
        for (_, a) in self.available.iter().enumerate() {
            print!("{} ", a);
        }

        print!("|| ");
        for tidx in 0..NTH {
            for ridx in 0..NRES {
                print!("{} ", self.allocation[tidx][ridx]);
            }

            print!("- ");

            for ridx in 0..NRES {
                print!("{} ", self.max[tidx][ridx]);
            }

            print!("| ");
        }
        println!("");
    }
}
