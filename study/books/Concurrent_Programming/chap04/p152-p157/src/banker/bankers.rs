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
    pub fn new(available: [usize; NRES], 
               max: [[usize; NRES]; NTH]) -> Self {
        Resource {
            available,
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
        let mut work = self.available.clone();
/*
 * prifri, 2022.12.06:
 * - 원본과 다르게 loop를 삭제한다. 직전에 found를 한번이라도 실패했으면
 *   종료하면된다.
 *   ex) thread가 3개있다고 한다.
 *   첫 loop때 0, 1번이 finish true가 되고 2번이 finish false가 되며
 *   found는 true이므로 2번째 loop를 돈다
 *   하지만 2번째 loop에서는 무조건 found가 false가 될것이므로 무의미한
 *   탐색을 한 loop돌며 실패로 끝날것이다.
 *   즉 found가 실패 한번 종료한시점에서 끝내면된다.
 *
 *   이렇게하면 lock을 잡는 시간이 늦고, finish배열을 스택에 할당 및 초기화
 *   할 필요도없으며 num_true도 필요없다.
 */

        for (alc, m) in self.allocation.iter().zip(self.max) {

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
            let need = m.iter().zip(alc).map(|(m, a)| m - a);
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
            if !is_avail {
                return false;
            }

            for (w, a) in work.iter_mut().zip(alc) {
                *w += *a
            }
        }

        return true;
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
            return true;
        } 

        self.allocation[id][resource] -= 1;
        self.available[resource] += 1;
        false
    }

    pub fn release(&mut self, id: usize, resource: usize) {
        if id >= NTH || resource >= NRES || self.allocation[id][resource] == 0 {
            return;
        }

        self.allocation[id][resource] -= 1;
        self.available[resource] += 1;
    }

    #[allow(dead_code)]
    pub fn show(&mut self, id: usize, is_take: bool, success: bool,
                resource: usize) {
        print!("{}{}{}{} | ",
               id,
               if is_take {
                   "take"
               } else {
                   "rele"
               },
               if success {
                   "O"
               } else {
                   "X"
               },
               resource);
        for (_, a) in self.available.iter().enumerate() {
            print!("{} ", a);
        }

        print!("|| ");
        for tidx in 0..NTH {
            for ridx in 0..NRES {
                print!("{} ", self.allocation[tidx][ridx]);
            }

            /*
            print!("- ");

            for ridx in 0..NRES {
                print!("{} ", self.max[tidx][ridx]);
            }
            */

            print!("| ");
        }
        println!("");
    }
}
