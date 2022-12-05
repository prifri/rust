mod bankers;

use self::bankers::Resource;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Banker<const NRES: usize, const NTH: usize> {
    resource: Arc<Mutex<Resource<NRES, NTH>>>,
}

/*
 * prifri, 2022.12.05:
 * - resoure 에 lock을 wrap한것.
 * - available
 */
impl<const NRES: usize, const NTH: usize> Banker<NRES, NTH> {
    pub fn new(max: [[usize; NRES]; NTH]) -> Self {
        Banker {
            resource: Arc::new(Mutex::new(Resource::new(0, max))),
        }
    }

    pub fn take(&self, id: usize, resource: usize) -> bool {
        let mut r = self.resource.lock().unwrap();
        let result = r.take(id, resource);
        if !result && resource != 0 {
            println!("{} {} fail", id, resource);
            r.show(id, true, result);
        }
        result
    }

    pub fn release(&self, id: usize, resource: usize) {
        let mut r = self.resource.lock().unwrap();
        r.release(id, resource);
        //r.show(id, false, true);
    }
}
