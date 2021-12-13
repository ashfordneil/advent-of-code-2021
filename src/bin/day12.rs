use advent_of_code_2021::{
    parsing,
    tools::{MoreItertools, StringTools},
    util,
};
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    str::FromStr,
};

use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(default_value = "./data/day-twelve.txt")]
    /// The path to the input file we want to run with.
    file: PathBuf,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Vertex {
    Start,
    End,
    Big(String),
    Small(String),
}

impl FromStr for Vertex {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let output = match s {
            "start" => Vertex::Start,
            "end" => Vertex::End,
            "" => eyre::bail!("No empty strings allowed"),
            _ if s.chars().all(|c| c.is_ascii_uppercase()) => Vertex::Big(s.into()),
            _ if s.chars().all(|c| c.is_ascii_lowercase()) => Vertex::Small(s.into()),
            _ => eyre::bail!("Invalid node string: {:?}", s),
        };

        Ok(output)
    }
}

#[derive(Debug)]
struct Edge {
    from: Vertex,
    to: Vertex,
}

impl FromStr for Edge {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split_parse("-")?;
        Ok(Edge { from, to })
    }
}

#[derive(Debug)]
struct Graph {
    edges: HashMap<Vertex, Vec<Vertex>>,
}

impl Graph {
    fn new(input: impl Iterator<Item = Edge>) -> eyre::Result<Self> {
        let mut edges = HashMap::new();

        let mut seen_start = false;
        let mut seen_end = false;

        for Edge { from, to } in input {
            match &from {
                Vertex::Start => seen_start = true,
                Vertex::End => seen_end = true,
                _ => (),
            };
            match &to {
                Vertex::Start => seen_start = true,
                Vertex::End => seen_end = true,
                _ => (),
            };
            edges
                .entry(from.clone())
                .or_insert_with(Vec::new)
                .push(to.clone());
            edges.entry(to).or_insert_with(Vec::new).push(from);
        }

        eyre::ensure!(seen_start, "Did not see the start node");
        eyre::ensure!(seen_end, "Did not see the end node");

        Ok(Graph { edges })
    }
}

impl FromStr for Graph {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let edges = s.lines_good().parsed().collect::<Result<Vec<_>, _>>()?;

        Graph::new(edges.into_iter())
    }
}

fn main() -> eyre::Result<()> {
    let args = util::setup::<Args>()?;
    let edges = parsing::line_separated::<Edge, _>(args.file)?;
    let graph = Graph::new(edges.into_iter())?;

    println!("Part one: {}", part_one(&graph));
    println!("Part two: {}", part_two(&graph));

    Ok(())
}

fn paths_to_end(
    graph: &Graph,
    vertex: &Vertex,
    seen: &HashSet<&str>,
    has_doubled_small_yet: bool,
) -> usize {
    graph
        .edges
        .get(vertex)
        .iter()
        .flat_map(|x| x.iter())
        .map(|next| match next {
            Vertex::End => 1,
            Vertex::Start => 0,
            Vertex::Big(_) => {
                let output = paths_to_end(graph, next, seen, has_doubled_small_yet);
                output
            }
            Vertex::Small(id) => {
                if !seen.contains(id.as_str()) {
                    let mut new_db = seen.clone();
                    new_db.insert(id);
                    paths_to_end(graph, next, &new_db, has_doubled_small_yet)
                } else if !has_doubled_small_yet {
                    paths_to_end(graph, next, seen, true)
                } else {
                    0
                }
            }
        })
        .sum()
}

fn part_one(input: &Graph) -> usize {
    paths_to_end(input, &Vertex::Start, &HashSet::new(), true)
}

fn part_two(input: &Graph) -> usize {
    paths_to_end(input, &Vertex::Start, &HashSet::new(), false)
}

#[cfg(test)]
mod test {
    const GRAPH_ONE: &'static str = r"
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
    ";

    const GRAPH_TWO: &'static str = r"
        dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc
    ";

    const GRAPH_THREE: &'static str = r"
        fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW
    ";

    #[test]
    fn example_part_one() {
        assert_eq!(10, super::part_one(&GRAPH_ONE.parse().unwrap()));
        assert_eq!(19, super::part_one(&GRAPH_TWO.parse().unwrap()));
        assert_eq!(226, super::part_one(&GRAPH_THREE.parse().unwrap()));
    }

    #[test]
    fn example_part_two() {
        assert_eq!(36, super::part_two(&GRAPH_ONE.parse().unwrap()));
        assert_eq!(103, super::part_two(&GRAPH_TWO.parse().unwrap()));
        assert_eq!(3509, super::part_two(&GRAPH_THREE.parse().unwrap()));
    }
}
