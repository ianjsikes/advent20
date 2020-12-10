#![feature(str_split_once)]

use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    io, mem,
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Part One: {}", part_one(INPUT));
    println!("Part Two: {}", part_two(INPUT));
}

fn part_one(input: &str) -> usize {
    let regex = Regex::new(r#"(\w+\s\w+)\sbags?"#).unwrap();
    let mut graph = HashMap::new();

    for line in input.lines() {
        let mut bags = regex.captures_iter(line);
        let outer_bag = bags.next().unwrap().get(1).unwrap().as_str();
        for inner_bag in bags {
            let inner_bag = inner_bag.get(1).unwrap().as_str();
            if inner_bag != "no other" {
                graph
                    .entry(inner_bag)
                    .or_insert_with(HashSet::new)
                    .insert(outer_bag);
            }
        }
    }

    let mut bags = HashSet::new();
    let mut tmp = Vec::new();
    tmp.push("shiny gold");
    while !tmp.is_empty() {
        for bag in mem::take(&mut tmp)
            .into_iter()
            .flat_map(|b| graph.get(b))
            .flatten()
        {
            bags.insert(bag);
            tmp.push(bag);
        }
    }

    bags.len()
}

fn part_two(input: &str) -> usize {
    let regex = Regex::new(r#"(\d+\s)?(\w+\s\w+)\sbags?"#).unwrap();

    let mut graph = HashMap::new();
    for line in input.lines() {
        let mut bags = regex.captures_iter(line);
        let outer_bag_color = bags.next().unwrap().get(2).unwrap().as_str();
        for inner_bag in bags {
            let bag_color = inner_bag.get(2).unwrap().as_str();
            if bag_color != "no other" {
                let bag_count = inner_bag
                    .get(1)
                    .unwrap()
                    .as_str()
                    .trim()
                    .parse::<usize>()
                    .unwrap();
                graph
                    .entry(outer_bag_color)
                    .or_insert_with(HashMap::new)
                    .insert(bag_color, bag_count);
            }
        }
    }

    fn recurse(bags: &HashMap<&str, HashMap<&str, usize>>, bag: &str) -> usize {
        bags.get(bag).map_or(0, |bs| {
            bs.iter().map(|(b, c)| c + c * recurse(bags, b)).sum()
        })
    }

    recurse(&graph, "shiny gold")
}
