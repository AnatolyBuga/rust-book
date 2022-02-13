#[derive(Debug)]
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, val: i32) {
        self.list.push(val);
        self.update_average(val, -1.0);
    }

    pub fn rmv(&mut self) -> Option<i32> {
        let r = self.list.pop();
        match r {
            Some(val) => {
                self.update_average(val, 1.0);
                Some(val)
            }
            None => None,
        }
    }

    fn update_average(&mut self, v: i32, delta: f64){
        self.average = (self.list.len() as f64 + delta) /self.list.len() as f64 * self.average -
             delta / self.list.len() as f64 * v as f64
    }

    fn new_average(v: &Vec<i32>) -> f64 {
        v.iter().sum::<i32>() as f64 / v.len() as f64
    }

    pub fn fn_average(&self) -> f64 {
        self.average //impl Copy, so not moved out
    }

    pub fn new(list: Vec<i32>) -> Self {
        let avrg = Self::new_average(&list);
        Self {
            list,
            average: avrg
        }
    }
}

pub fn oops() {
    let mut ac = AveragedCollection::new(vec![1, 2]);
    println!("ac: {:?}", ac);
    ac.add(0);
    println!("ac: {:?}", ac);
    ac.rmv();
    println!("ac: {:?}", ac);
    ac.add(3);
    println!("ac: {:?}", ac);
    ac.add(-2);
    println!("ac: {:?}", ac);
}