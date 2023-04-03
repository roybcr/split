fn main() {
    // group debts represents 4 group members debts
    // where group_debts[i][j] is the amount member i owes to member j.
    let group_debts = vec![
        vec![0., 10., 20., 0.],
        vec![5., 0., 10., 17.],
        vec![0., 0., 0.0, 0.0],
        vec![32., 0., 5.0, 0.],
    ];

    // minimize_transactions represents the optimal deficit distribution between
    // group members, that requires the least number of transactions to be
    // resolved.
    println!("{:#?}", minimize_transactions(group_debts));
}

// Complexity: O(n^3)
fn minimize_transactions(matrix: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let n = matrix.len();
    let mut net_balances = vec![0.0; n];

    for i in 0..n {
        for j in 0..n {
            // if A owes B x and B owes A y,
            // B owes A (y - x) and A owes B (x - y)
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
