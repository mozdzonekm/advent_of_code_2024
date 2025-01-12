use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::newline, character::complete::u32, multi::many0,
    multi::separated_list0, sequence::separated_pair, IResult,
};
use petgraph::{graph::DiGraph, visit::Topo};

advent_of_code::solution!(5);

type Update = Vec<u32>;
type Dependency = (u32, u32);

fn parse_input(input: &str) -> IResult<&str, (Vec<Dependency>, Vec<Update>)> {
    let (input, (dependencies, updates)) = separated_pair(
        separated_list0(newline, separated_pair(u32, tag("|"), u32)),
        many0(newline),
        separated_list0(newline, separated_list0(tag(","), u32)),
    )(input)?;
    Ok((input, (dependencies, updates)))
}

fn update_is_valid(pages: &[u32], dependencies: &[(u32, u32)]) -> bool {
    let violated_dependencies = dependencies
        .iter()
        .filter(|(rule_predecessor, rule_sucessor)| {
            let pages_to_check = pages
                .iter()
                .filter(|page| **page == *rule_predecessor || **page == *rule_sucessor)
                .collect_tuple();
            match pages_to_check {
                Some((first, second)) => first == rule_sucessor && second == rule_predecessor,
                _ => false,
            }
        })
        .collect_vec();
    violated_dependencies.is_empty()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, (dependencies, updates)) = parse_input(input).unwrap();
    let result = updates
        .iter()
        .filter(|pages| !pages.is_empty() && update_is_valid(pages, &dependencies))
        .map(|pages| pages[pages.len() / 2])
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, (dependencies, updates)) = parse_input(input).unwrap();
    let incorrectly_order_updates = updates
        .iter()
        .filter(|pages| !pages.is_empty() && !update_is_valid(pages, &dependencies))
        .collect_vec();
    let result: u32 = incorrectly_order_updates
        .iter()
        .map(|update| {
            let relevant_edges = dependencies
                .iter()
                .filter(|(a, b)| update.contains(a) && update.contains(b))
                .collect_vec();
            let dependencies_graph: DiGraph<u32, (u32, u32)> = DiGraph::from_edges(relevant_edges);
            let mut pages_order = Topo::new(&dependencies_graph);
            let mut new_order = vec![];
            while let Some(node) = pages_order.next(&dependencies_graph) {
                let node_value = node.index() as u32;
                if update.contains(&node_value) {
                    new_order.push(node_value);
                }
            }
            new_order
        })
        .map(|pages| pages[pages.len() / 2])
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
