use std::collections::{HashMap, VecDeque};

fn main() {
    run_part_1(include_str!("input.txt"));
}

fn run_part_1(input: &str) {
    let links = parse(input);
    
    let mut topdown = HashMap::new();
    for (a,b) in links.iter() {
        topdown.entry(a).or_insert_with(|| (Vec::new(), None, None)).0.push(b);
        topdown.entry(b).or_insert_with(|| (Vec::new(), None, None));
    }

    let mut to_examine = VecDeque::new();
    let mut total = 0;
    to_examine.push_back(("COM",0,None));
    while let Some((body,dist,parent)) = to_examine.pop_front() {
        if let Some(orbits) = topdown.get_mut(&body) {
            orbits.1 = Some(dist);
            orbits.2 = parent;
            for inner_body in orbits.0.iter() {
                to_examine.push_back((inner_body, dist+1, Some(body)));
            }
        }
        total += dist;
    }

    println!("total links: {}", total);
    let get_links = |mut body| {
        let mut links = Vec::new();
        while let Some(orbits) = topdown.get(&body) {
            if let Some(parent) = orbits.2 {
                links.push(parent);
                body = parent;
            } else {
                break;
            }
        }
        links
    };
    
    let san_links = get_links("SAN");
    let you_links = get_links("YOU");
    for (i,body) in you_links.iter().enumerate() {
        if let Some(pos) = san_links.iter().position(|b| b == body) {
            println!("num transfers: {}", pos + i);
            break;
        }
    }
}


fn parse(input: &str) -> Vec<(&str, &str)> {
    input
        .trim()
        .lines()
        .map(|s| {
            let mut i = s.split(')');
            (i.next().unwrap(), i.next().unwrap())
        })
        .collect()
}
