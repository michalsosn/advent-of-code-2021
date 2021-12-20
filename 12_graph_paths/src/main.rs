use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::io;
use std::io::BufRead;


fn main() {
    let Input { graph } = Input::read_stdin();
    println!("{:?}", graph);

    let path_count: u64 = count_paths(&graph, false);
    println!("path_count max 1 {}", path_count);

    let path_count: u64 = count_paths(&graph, true);
    println!("path_count max 2 {}", path_count);
}

#[derive(Debug)]
struct Graph {
    adjacency_lists: HashMap<String, Vec<String>>,
}

#[derive(Debug)]
struct Input {
    graph: Graph,
}

impl Input {
    fn read_stdin() -> Self {
        let mut adjacency_lists: HashMap<String, Vec<String>> = HashMap::new();
        io::stdin().lock().lines()
            .for_each(|line| {
                let pair = line.expect("error: unable to read line").trim()
                    .split('-')
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                match pair.as_slice() {
                    [a, b] => {
                        if b != "start" && a != "end" {
                            let a_list = adjacency_lists.entry(a.to_string()).or_insert(Vec::new());
                            a_list.push(b.to_string());
                        }
                        if a != "start" && b != "end" {
                            let b_list = adjacency_lists.entry(b.to_string()).or_insert(Vec::new());
                            b_list.push(a.to_string());
                        }
                    }
                    other => {
                        panic!("Not an edge {:?}", other)
                    }
                }
            });
        for (_, list) in adjacency_lists.iter_mut() {
            list.sort();
            list.reverse();
        }
        adjacency_lists.shrink_to_fit();
        Input {
            graph: Graph {
                adjacency_lists
            }
        }
    }
}


#[derive(Debug)]
struct Visit<'a> {
    name: &'a String,
    visited_small: BTreeSet<&'a String>,
    multi_visit_left: bool,
}

fn count_paths(graph: &Graph, allow_multi_visit: bool) -> u64 {
    let Graph { adjacency_lists } = graph;
    let start = String::from("start");
    let end = String::from("end");

    let mut path_count: u64 = 0;
    let mut queue: VecDeque<Visit> = VecDeque::new();

    queue.push_back(Visit {
        name: &start,
        visited_small: BTreeSet::new(),
        multi_visit_left: allow_multi_visit
    });

    while !queue.is_empty() {
        let Visit { name, visited_small, multi_visit_left } = queue.pop_back().unwrap();
        for neighbor in &adjacency_lists[name] {
            let mut visited_small = visited_small.clone();
            if *neighbor == end {
                path_count += 1;
            } else if is_small(neighbor) {
                if !visited_small.contains(neighbor) {
                    visited_small.insert(neighbor);
                    queue.push_back(Visit { name: neighbor, visited_small, multi_visit_left });
                } else if multi_visit_left {
                    queue.push_back(Visit { name: neighbor, visited_small: visited_small, multi_visit_left: false });
                }
            } else {
                queue.push_back(Visit { name: neighbor, visited_small: visited_small, multi_visit_left });
            }
        }
    }

    path_count
}

fn is_small(name: &str) -> bool {
    name.chars().next().unwrap().is_lowercase()
}
