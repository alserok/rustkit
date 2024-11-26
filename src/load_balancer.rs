pub trait LoadBalancer<T> {
    fn add(&mut self, val: T);
    fn pick(&mut self) -> Option<T>;
    fn remove(&mut self, val: T) -> bool;
}

pub struct RoundRobin<T: Copy> {
    values: Vec<T>,
    idx: usize,
}

impl<T: Copy> RoundRobin<T> {
    pub fn new() -> Self {
        RoundRobin {
            values: vec![],
            idx: 0,
        }
    }
}

impl<T: Copy + std::cmp::PartialEq> LoadBalancer<T> for RoundRobin<T> {
    fn add(&mut self, val: T) {
        self.values.push(val);
    }

    fn pick(&mut self) -> Option<T> {
        let val = self.values[self.idx as usize];

        self.idx += 1;

        if self.idx == self.values.len() {
            self.idx = 0
        }

        Some(val)
    }

    fn remove(&mut self, val: T) -> bool {
        if let Some(pos) = self.values.iter().position(|x| *x == val) {
            self.values.remove(pos);
            true
        } else {
            false
        }
    }
}
