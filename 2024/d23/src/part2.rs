use std::collections::{HashMap, HashSet};

use aoc_helper::read_file;

pub fn solve() {
    // let input = read_file("test_input.txt");
    let input = read_file("input.txt");

    let connections_raw: Vec<(&str, &str)> = input
        .lines()
        .map(|l| l.split_once('-').unwrap())
        .collect();

    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut vert_list: HashSet<&str> = HashSet::new();
    for (c1, c2) in connections_raw {
        connections.entry(c1).and_modify(|v| v.push(c2)).or_insert(vec![c2]);
        connections.entry(c2).and_modify(|v| v.push(c1)).or_insert(vec![c1]);
        vert_list.insert(c1);
        vert_list.insert(c2);
    }
    
    let mut output = vec![];
    let vert_list = vert_list.into_iter().collect();
    born_kerbosch_2(vec![], vert_list, vec![], &connections, &mut output);

    let mut max_vec = output[0].clone();
    for vec in output {
        if vec.len() > max_vec.len() {
            max_vec = vec;
        }
    }

    max_vec.sort();
    for i in 0..max_vec.len() - 1 {
        print!("{},", max_vec[i]);
    }
    println!("{}", max_vec[max_vec.len() - 1]);
}

/*
* finds the maximal cliques that include 
*   all of the vertices in R,
*   some of the vertices in P,
*   and none of the vertices in X
*
* algorithm BronKerbosch2(R, P, X) is
*   if P and X are both empty then
*       report R as a maximal clique
*   choose a pivot vertex u in P ⋃ X
*   for each vertex v in P \ N(u) do
*       BronKerbosch2(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
*       P := P \ {v}
*       X := X ⋃ {v}
*/

fn born_kerbosch_2<'a>(
    r: Vec<&'a str>,
    mut p: Vec<&'a str>,
    mut x: Vec<&'a str>,
    connections: &HashMap<&'a str, Vec<&'a str>>,
    output: &mut Vec<Vec<&'a str>>
) {
    if p.is_empty() && x.is_empty() {
        output.push(r);
        return;
    }

    let u = if !p.is_empty() {
        p[0]
    } else {
        x[0]
    };
    let ucon = connections.get(u).unwrap();

    for vertex in p.clone() {
        if ucon.contains(&vertex) {
            continue;
        }
        let vcon = connections.get(vertex).unwrap();

        let mut nr = r.clone();
        nr.push(vertex);

        let np = p
            .clone()
            .iter()
            .filter(|v| vcon.contains(&v))
            .cloned()
            .collect();

        let nx = x
            .clone()
            .iter()
            .filter(|v| vcon.contains(&v))
            .cloned()
            .collect();

        born_kerbosch_2(nr, np, nx, connections, output);

        let ind = p.iter().position(|e| *e == vertex).unwrap();
        p.remove(ind);
        x.push(vertex);
    }
}
