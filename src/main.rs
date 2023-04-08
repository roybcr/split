use crate::{group::Group, member::Member};

mod group;
mod member;

fn max_balance_index(net_balances: &[f64]) -> Option<usize> {
    net_balances
        .iter()
        .enumerate()
        .filter(|(_, &balance)| balance > 0.0)
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(idx, _)| idx)
}

fn min_balance_index(net_balances: &[f64]) -> Option<usize> {
    net_balances
        .iter()
        .enumerate()
        .filter(|(_, &balance)| balance < 0.0)
        .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(idx, _)| idx)
}

// Complexity: O(n^2)
fn minimize_transactions(matrix: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let n = matrix.len();
    let mut net_balances = vec![0.0; n];

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            net_balances[i] += matrix[j][i] - matrix[i][j];
        }
    }

    let mut result = vec![vec![0.0; n]; n];

    while let (Some(max_idx), Some(min_idx)) = (
        max_balance_index(&net_balances),
        min_balance_index(&net_balances),
    ) {
        let offset = net_balances[max_idx].min(-net_balances[min_idx]);
        result[min_idx][max_idx] += offset;
        net_balances[max_idx] -= offset;
        net_balances[min_idx] += offset;
    }

    result
}

fn main() {
    const S: usize = 6;
    let mut group = Group::<S>::new();
    let members = vec!["adi", "alin", "lerer", "liran", "roy", "thomas"];

    for idx in 0..S {
        group.add(Member::new(idx, members[idx]));
    }

    group.add_debt("adi", "lerer", 124.0).unwrap();
    group.add_debt("alin", "lerer", 20.0).unwrap();
    group.add_debt("lerer", "liran", 935.0).unwrap();
    group.add_debt("lerer", "thomas", 640.0).unwrap();
    group.add_debt("liran", "adi", 362.0).unwrap();
    group.add_debt("liran", "lerer", 125.0).unwrap();
    group.add_debt("liran", "roy", 1987.0).unwrap();
    group.add_debt("roy", "adi", 1705.0).unwrap();
    group.add_debt("roy", "lerer", 33.0).unwrap();
    group.add_debt("roy", "thomas", 613.0).unwrap();
    group.add_debt("thomas", "adi", 2188.0 - 740.0).unwrap();
    group.add_debt("thomas", "lerer", 81.11).unwrap();
    group.add_debt("thomas", "liran", 658.0).unwrap();

    let debts: Vec<Vec<f64>> = group.into();
    let optimal_cashflow = minimize_transactions(debts);

    println!();
    for (row, debts) in optimal_cashflow.into_iter().enumerate() {
        for (col, debit) in debts.into_iter().enumerate() {
            if row == col || debit == 0.0 {
                continue;
            }

            println!(
                "{debtor:<7} OWES {creditor:<7} ₪ {amount:<6}",
                debtor = members[row],
                creditor = members[col],
                amount = debit.ceil()
            );
        }
    }
}
