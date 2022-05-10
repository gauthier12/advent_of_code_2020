#[macro_use]
extern crate scan_fmt;
extern crate regex;
use rayon::prelude::*;
use regex::Regex;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::HashMap;

#[derive(Hash, Debug)]
struct BagNode
{
    color:String,
    color_id:usize,
    next:Vec<(usize,u8)>
}
impl BagNode {
    fn new(color:String,color_id:usize) -> BagNode {
        BagNode { color,color_id, next:Vec::new() }
    }
}
#[derive( Debug)]
struct BagTree
{
    color_num:HashMap<String,usize>,
    num_color:Vec<String>,
    db:Vec<BagNode>
    
}
impl BagTree
{
    fn new() -> BagTree {
        BagTree{ color_num:HashMap::new(),num_color:Vec::new(),db:Vec::new()}
    }
    fn find_color_num(&self,color_name:&String) -> usize{
        *self.color_num.get(color_name).unwrap()
    }
    fn find_color_name(&self,color_num:&usize) -> String{
        self.num_color[*color_num].to_string()
    }
    fn find_color(&self,search_id:usize, cur_id:usize )->bool
    {
        for (id,num) in self.db[cur_id].next.iter()
        {
            if *id==search_id
            {
                return true
            }
        }
        for (id,num) in self.db[cur_id].next.iter()
        {
            if self.find_color(search_id,*id)
            {
                return true
            }
        }
        return false
    }

    fn count_bag(&self,cur_id:usize )->u32
    {
        let mut tot_bag = 0;
        for (id,num) in self.db[cur_id].next.iter()
        {
            println!("count_bag {} calling {} * {}", cur_id, *num,*id);
            tot_bag += (*num as u32)*self.count_bag(*id);
        }
        tot_bag+1
    }
}

fn main() {
    let mut bag_db = BagTree::new();
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let filename = &args[1];
    let re_node_bag = Regex::new(r"^(.+) bags contain.*$").unwrap();
    let re_next_node = Regex::new(r"(\d)+ ([a-z]+ [a-z]+) bag").unwrap();
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    for line in contents.lines() {
        let color_name = &(re_node_bag.captures_iter(line).next().unwrap()[1]).to_string();
        bag_db.num_color.push(color_name.to_string());
    }

    for (ic,color) in bag_db.num_color.iter().enumerate()
    {
        bag_db.color_num.entry(color.to_string()).or_insert(ic);
    }

    for line in contents.lines() {
        let node_name = &(re_node_bag.captures_iter(line).next().unwrap()[1]).to_string();
        let color_id = bag_db.color_num.get(node_name).unwrap();
        let mut cur_node:BagNode = BagNode::new(node_name.to_string(),*color_id);
        if re_next_node.is_match(line) {
            for part in re_next_node.captures_iter(line) {
               let next_color_name = &(part[2]).to_string();
               let next_color_id = bag_db.find_color_num(next_color_name);
               let next_num = (&part[1]).parse::<u8>().unwrap();
               cur_node.next.push((next_color_id,next_num));
            }
        }
        bag_db.db.push(cur_node);
    }
    let color_searched = "shiny gold".to_string();
    let id_searched = bag_db.find_color_num(&color_searched);
    println!("searching color {} num:{}",color_searched,id_searched);

    println!("Methode A1");
    let mut num_found_a:u32 = 0;
    let start_a1 = Instant::now();
    for cur_bag_id in 0..bag_db.db.len()
    {
        if bag_db.find_color(id_searched, cur_bag_id)
        {
            num_found_a += 1;
        }
    
    }
    let duration_a1 = start_a1.elapsed();
    println!("Methode B1");
    let start_b1 = Instant::now();

    let num_list:Vec<usize> = (0..bag_db.db.len()).collect();
    let  num_found_b:u32 = num_list.par_iter().map(|cid| bag_db.find_color(id_searched, *cid) as u32 ).sum();
    let duration_b1 = start_b1.elapsed();


    println!("Part 2");
    let start_a2 = Instant::now();
    let bag_found_a:u32 = bag_db.count_bag(id_searched) - 1;
    let duration_a2 = start_a2.elapsed();

    let duration = start.elapsed();

    println!("Number of colors containing {} : {:}", color_searched, num_found_a);
    println!("Number of colors containing {} : {:}", color_searched, num_found_b);
    println!("Number of bags in {} : {:}", color_searched, bag_found_a);
    println!("Time elapsed in A1() is: {:?}", duration_a1);
    println!("Time elapsed in B1() is: {:?}", duration_b1);
    println!("Time elapsed in A2() is: {:?}", duration_a2);

    println!("Time elapsed in total is: {:?}", duration);
}
