use std::fs;

pub fn solve() {
    // let input = fs::read_to_string("test_input.txt").unwrap().trim().to_string();
    let input = fs::read_to_string("input.txt").unwrap().trim().to_string();

    let (order_rules_str, orders_str) = input.split_once("\n\n").unwrap();
    
    let mut order_rules: Vec<Vec<usize>> = (0..100).map(|_| vec![]).collect();

    for l in order_rules_str.trim().split('\n') {
        let (sn1, sn2) = l.split_once('|').unwrap();
        let (n1, n2) = (sn1.parse::<usize>().unwrap(), sn2.parse::<usize>().unwrap());
        order_rules[n1].push(n2);
    }

    let result: usize = orders_str
        .trim()
        .split('\n')
        .map(|s| {
            let orders = s.split(',').map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();

            for i in 1..orders.len() {
                let o = orders[i];
                let afters = &order_rules[o];

                for k in 0..i {
                    if afters.contains(&orders[k]) {
                        return 0;
                    }
                }
            }

            orders[orders.len() / 2]
        })
        .sum();

    println!("{}", result); 
}
