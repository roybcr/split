#![allow(dead_code)]
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

#[derive(Debug, Clone)]
pub enum Amount {
    N(usize),
    P(usize),
}

#[derive(Debug, Clone)]
pub struct Person {
    pub id:    String,
    pub debts: HashMap<String, usize>,
}

impl Person {
    pub fn new(id: String) -> Self {
        Person {
            id,
            debts: HashMap::new(),
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn get_debts(&self) -> HashMap<String, usize> {
        self.debts.clone()
    }

    pub fn get_debts_to(&self, t: &str) -> Option<&usize> {
        self.debts.get(&String::from(t))
    }

    pub fn add_debts(&mut self, debts: HashMap<String, usize>) {
        self.debts = debts;
    }

    pub fn rem_debt(&mut self, to: &str) {
        self.debts.remove(&String::from(to)).unwrap();
    }

    pub fn adj_debt(&mut self, to: &str, amount: Amount) {
        #[rustfmt::skip]
        self.debts
            .entry(String::from(to))
            .and_modify(|current| {
                match amount {
                    Amount::N(amount) => { *current -= amount; }
                    Amount::P(amount) => { *current += amount; }
                };
            });
    }
}

#[derive(Debug, Clone)]
pub struct Split(HashMap<String, Person>);

impl Split {
    pub fn new(members: HashMap<String, Person>) -> Self {
        Split(members)
    }

    pub fn get_member(&self, id: &str) -> &Person {
        self.0.get(&String::from(id)).unwrap()
    }

    pub fn get_member_mut(&mut self, id: String) -> &mut Person {
        self.0.get_mut(&id).unwrap()
    }

    pub fn resolve_debts(&mut self) {
        let mut immut_self = self.clone();
        let mut resolved = HashSet::<&str>::new();
        let members = immut_self.0.values_mut().collect::<Vec<&mut Person>>();

        for member in members.into_iter() {
            #[rustfmt::skip]
            if resolved.contains(&member.id.as_str()) { continue; }
            for (key, value) in member.get_debts().iter() {
                let owe_to = self.get_member_mut(key.to_owned());

                if let Some(owe_back) = owe_to.get_debts_to(&member.id()) {
                    let mid = &member.id();
                    let oid = &owe_to.id();

                    match (*owe_back).cmp(&value) {
                        Ordering::Less => {
                            let amount = Amount::N(*owe_back);
                            owe_to.rem_debt(mid);
                            member.adj_debt(oid, amount);
                        }

                        Ordering::Equal => {
                            owe_to.rem_debt(mid);
                            member.rem_debt(oid);
                        }

                        Ordering::Greater => {
                            let amount = Amount::N(*value);
                            member.rem_debt(oid);
                            owe_to.adj_debt(mid, amount);
                        }
                    }
                }
            }

            resolved.insert(&member.id.as_str());
        }
    }

    pub fn offset_debts(&mut self) {
        let resolved = HashSet::<&str>::new();
        let mut members = self.0.clone();

        for (id, person) in members.iter_mut() {
            #[rustfmt::skip]
            if resolved.contains(id.as_str()) { continue; }
            let debts = person.get_debts();
            if debts.len() <= 1 {
                continue;
            }

            // for (key, value) in debts.iter() {
            // let person = self.get_member_mut(key.to_owned());
            // if let Some(amount_owed) = .get_debts_to(&member.id()) {
            // }
            // }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_resolve_debts() {
        let mut roy = Person::new(String::from("roy"));
        let mut adi = Person::new(String::from("adi"));
        let mut tom = Person::new(String::from("tom"));

        roy.add_debts(HashMap::from([(adi.id(), 1023), (tom.id(), 446)]));
        adi.add_debts(HashMap::from([(roy.id(), 23), (tom.id(), 0)]));
        tom.add_debts(HashMap::from([(adi.id(), 349)]));

        let mut split = super::Split::new(HashMap::from([
            (roy.id(), roy),
            (adi.id(), adi),
            (tom.id(), tom),
        ]));

        split.resolve_debts();
        assert_eq!(split.get_member("roy").get_debts_to("adi"), Some(&1000));
        assert_eq!(split.get_member("adi").get_debts_to("roy"), None);

        println!("{:#?}", split.0);
    }
}
