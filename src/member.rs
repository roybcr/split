use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Member<'a, const SIZE: usize> {
    pub id:    usize,
    pub name:  &'a str,
    pub debts: HashMap<usize, f64>,
}

impl<'a, const S: usize> Member<'a, S> {
    pub fn new(id: usize, name: &'a str) -> Self {
        Member {
            id,
            name,
            debts: (0..S).map(|i| (i, 0.0)).collect::<HashMap<usize, f64>>(),
        }
    }

    pub fn add_debt(&mut self, id: usize, debt: f64) { self.debts.insert(id, debt); }
}
