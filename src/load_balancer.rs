pub trait LoadBalancer<T> {
    fn add(&mut self, val: T);
    fn pick(&mut self) -> Option<T>;
    fn remove(&mut self, val: T) -> bool;
    fn all(&mut self) -> &Vec<T>;
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
        if self.values.len() == 0 {
            return None;
        }

        let val = self.values[self.idx];

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

    fn all(&mut self) -> &Vec<T> {
        &self.values
    }
}

pub struct StickyRoundRobin<T: Copy> {
    values: Vec<T>,
    idx: usize,
    stick: usize,
    curr_stick_value: usize,
}

impl<T: Copy> StickyRoundRobin<T> {
    pub fn new(stick: i32) -> Self {
        StickyRoundRobin {
            values: vec![],
            idx: 0,
            stick: stick as usize,
            curr_stick_value: 0,
        }
    }
}

impl<T: Copy + std::cmp::PartialEq> LoadBalancer<T> for StickyRoundRobin<T> {
    fn add(&mut self, val: T) {
        self.values.push(val);
    }

    fn all(&mut self) -> &Vec<T> {
        &self.values
    }

    fn pick(&mut self) -> Option<T> {
        if self.values.len() == 0 {
            return None;
        }

        if self.curr_stick_value == self.stick {
            self.curr_stick_value = 0;
            if self.idx == self.values.len() - 1 {
                self.idx = 0;
            } else {
                self.idx += 1;
            }
        }

        self.curr_stick_value += 1;
        Some(self.values[self.idx])
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
