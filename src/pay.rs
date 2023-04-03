#![allow(dead_code)]
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub enum Amount {
    N(usize),
    P(usize),
}

#[derive(Debug, Clone)]
pub struct Person {
    pub id:    usize,
    pub debts: Vec<f64>,
}

impl Person {
    pub fn new(id: usize, size: usize) -> Self {
        Person {
            id,
            debts: vec![0.0; size],
        }
    }

    pub fn add_debt(&mut self, to: usize, amount: f64) {
        self.debts[to] = amount;
    }

    // pub fn id(&self) -> String {
    //     self.id.clone()
    // }

    // pub fn get_debts(&self) -> HashMap<String, usize> {
    //     self.debts.clone()
    // }

    // pub fn get_debts_to(&self, t: &str) -> Option<&usize> {
    //     self.debts.get(&String::from(t))
    // }

    // pub fn add_debts(&mut self, debts: HashMap<String, usize>) {
    //     self.debts = debts;
    // }

    // pub fn rem_debt(&mut self, to: &str) {
    //     self.debts.remove(&String::from(to)).unwrap();
    // }

    // pub fn adj_debt(&mut self, to: &str, amount: Amount) {
    //     #[rustfmt::skip]
    //     self.debts
    //         .entry(String::from(to))
    //         .and_modify(|current| {
    //             match amount {
    //                 Amount::N(amount) => { *current -= amount; }
    //                 Amount::P(amount) => { *current += amount; }
    //             };
    //         });
    // }
}

type Mat = Vec<Vec<f64>>;

#[derive(Debug, Clone)]
pub struct Split(Mat);

impl Split {
    pub fn new(group: Vec<Person>) -> Self {
        Split(group.into_iter().map(|p| p.debts).collect())
    }

    pub fn traverse(&mut self) {
        // let mut seen = HashMap::new();

        pub fn dfs(
            node: usize,
            with: usize,
            graph: &mut Mat,
            seen: &mut HashMap<usize, HashSet<usize>>,
        ) -> f64 {
            let graph_read = graph.clone();
            let mut debt_to_with = graph_read[node][with];
            for (i, debt) in graph_read[node].iter().enumerate() {
                #[rustfmt::skip]
                if (*debt).eq(&0.0) { continue; }
                if let Some(debts) = seen.get_mut(&i) {
                    if !debts.contains(&i) {
                        debts.insert(i);

                        if let Some(credit) = graph_read[i].get(node) {
                            if credit.gt(debt) {
                                graph[node][i] = 0.0;
                                graph[i][node] = credit - debt;
                            } else if credit.lt(debt) {
                                graph[i][node] = 0.0;
                                graph[node][i] = debt - credit;
                            }
                        }

                        if with == node {
                            return debt_to_with;
                        }
                    }
                }
            }
        }
    }
}

//     pub fn get_member(&self, id: &str) -> &Person {
//         self.0.get(&String::from(id)).unwrap()
//     }

//     pub fn get_member_mut(&mut self, id: String) -> &mut Person {
//         self.0.get_mut(&id).unwrap()
//     }

//     pub fn resolve_debts(&mut self) {
//         let mat = self
//             .0
//             .values()
//             .map(|p| p.debts.values().collect::<Vec<&usize>>())
//             .collect::<Vec<Vec<&usize>>>();

//         println!("{:#?}", mat);

//         let mut immut_self = self.clone();
//         let mut resolved = HashSet::<&str>::new();
//         let members = immut_self.0.values_mut().collect::<Vec<&mut
// Person>>();

//         for member in members.into_iter() {
//             #[rustfmt::skip]
//             if resolved.contains(&member.id.as_str()) { continue; }
//             for (key, value) in member.get_debts().iter() {
//                 let owe_to = self.get_member_mut(key.to_owned());

//                 if let Some(owe_back) = owe_to.get_debts_to(&member.id()) {
//                     let mid = &member.id();
//                     let oid = &owe_to.id();

//                     match (*owe_back).cmp(&value) {
//                         Ordering::Less => {
//                             let amount = Amount::N(*owe_back);
//                             owe_to.rem_debt(mid);
//                             member.adj_debt(oid, amount);
//                         }

//                         Ordering::Equal => {
//                             owe_to.rem_debt(mid);
//                             member.rem_debt(oid);
//                         }

//                         Ordering::Greater => {
//                             let amount = Amount::N(*value);
//                             member.rem_debt(oid);
//                             owe_to.adj_debt(mid, amount);
//                         }
//                     }
//                 }
//             }

//             resolved.insert(&member.id.as_str());
//         }
//     }

//     // pub fn offset_debts(&mut self) {
//     //     let resolved = HashSet::<&str>::new();
//     //     let mut members = self.0.clone();

//     //     for (id, person) in members.iter_mut() {
//     //         #[rustfmt::skip]
//     //         if resolved.contains(id.as_str()) { continue; }
//     //         let debts = person.get_debts();
//     //         if debts.len() <= 1 {
//     //             continue;
//     //         }

//     //         for (key, value) in debts.iter() {
//     //             let person = self.get_member_mut(key.to_owned());
//     //             if let Some(amount_owed) =
//     // person.get_debts_to(person.id().as) {}         }
//     //     }
//     // }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::collections::HashMap;

//     #[test]
//     fn test_resolve_debts() {
//         let mut roy = Person::new(String::from("roy"));
//         let mut adi = Person::new(String::from("adi"));
//         let mut tom = Person::new(String::from("tom"));

//         roy.add_debts(HashMap::from([
//             (roy.id(), 0),
//             (adi.id(), 1023),
//             (tom.id(), 446),
//         ]));
//         adi.add_debts(HashMap::from([
//             (roy.id(), 23),
//             (adi.id(), 0),
//             (tom.id(), 0),
//         ]));
//         tom.add_debts(HashMap::from([
//             (roy.id(), 0),
//             (adi.id(), 349),
//             (tom.id(), 0),
//         ]));

//         let mut split = super::Split::new(HashMap::from([
//             (roy.id(), roy),
//             (adi.id(), adi),
//             (tom.id(), tom),
//         ]));

//         split.resolve_debts();
//         assert_eq!(split.get_member("roy").get_debts_to("adi"), Some(&1000));
//         assert_eq!(split.get_member("adi").get_debts_to("roy"), None);

//         println!("{:#?}", split.0);
//     }
// }
