use std::collections::HashMap;

use crate::member::Member;

#[derive(Debug)]
pub enum GroupError<'a> {
    MemberNotExists(&'a str),
}

type GroupResult<'a, T> = Result<T, GroupError<'a>>;

#[derive(Debug, Clone)]
pub struct Group<'a, const SIZE: usize> {
    pub members: HashMap<&'a str, Member<'a, SIZE>>,
}

impl<'a, const S: usize> Group<'a, S> {
    pub fn new() -> Self {
        Group {
            members: HashMap::new(),
        }
    }

    pub fn get(&self, name: &'a str) -> Option<&Member<'a, S>> { self.members.get(name) }

    pub fn get_mut(&mut self, name: &'a str) -> Option<&mut Member<'a, S>> {
        self.members.get_mut(name)
    }

    pub fn add(&mut self, member: Member<'a, S>) {
        self.members.entry(member.name).or_insert(member);
    }

    pub fn add_debt(
        &mut self,
        debtor_name: &'a str,
        creditor_name: &'a str,
        amount: f64,
    ) -> GroupResult<()> {
        if let Some(creditor) = self.get(creditor_name) {
            let creditor_id = creditor.id;
            if let Some(debtor) = self.get_mut(debtor_name) {
                debtor.add_debt(creditor_id, amount);
                return Ok(());
            }
        }

        Err(GroupError::MemberNotExists(
            "Couldn't find one or more of the requested members.",
        ))
    }
}

impl<'a, const S: usize> Into<Vec<Vec<f64>>> for Group<'a, S> {
    fn into(self) -> Vec<Vec<f64>> {
        let mut mat = vec![vec![0.0f64; S]; S];

        for member in self.members.values().into_iter() {
            for (col, debt) in member.debts.clone().into_iter() {
                mat[member.id][col] = debt;
            }
        }

        mat
    }
}
