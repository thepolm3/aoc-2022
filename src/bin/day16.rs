use itertools::Itertools;
use ndarray::Array2;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

fn digit1(input: &str) -> IResult<&str, usize> {
    map_res(nom::character::complete::digit1, str::parse)(input)
}

fn parse_valve(input: &str) -> IResult<&str, (&str, usize, Vec<&str>)> {
    map(
        tuple((
            tag("Valve "),
            alpha1,
            tag(" has flow rate="),
            digit1,
            alt((
                tag("; tunnels lead to valves "),
                tag("; tunnel leads to valve "),
            )),
            separated_list1(tag(", "), alpha1),
        )),
        |(_, name, _, flow_rate, _, neighbors)| (name, flow_rate, neighbors),
    )(input)
}

fn parse(input: &str) -> IResult<&str, (usize, Vec<(usize, usize, Vec<usize>)>)> {
    let (remaining, valves) = separated_list1(line_ending, parse_valve)(input)?;

    let lookup: Vec<&str> = valves.iter().map(|(i, _, _)| *i).collect();
    let start_index = lookup.iter().position(|&x| x == "AA").unwrap();
    let indexed_valves = valves
        .into_iter()
        .map(|(i, flow, nbrs)| {
            (
                lookup.iter().position(|&x| x == i).unwrap(),
                flow,
                nbrs.into_iter()
                    .map(|n| lookup.iter().position(|&x| x == n).unwrap())
                    .collect(),
            )
        })
        .collect();
    Ok((remaining, (start_index, indexed_valves)))
}

fn floyd_warshall(dist: &mut Array2<Option<usize>>) {
    let n = dist.dim().0;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if let (Some(q), Some(r)) = (dist[[j, i]], dist[[i, k]]) {
                    dist[[j, k]] = Some(dist[[j, k]].unwrap_or(q + r).min(q + r));
                }
            }
        }
    }
}

fn get_flows_and_dist(valves: Vec<(usize, usize, Vec<usize>)>) -> (Vec<usize>, Array2<usize>) {
    let n = valves.len();
    let mut adjacancy = Array2::<Option<usize>>::default((n, n));

    for (v, _, nbrs) in &valves {
        adjacancy[[*v, *v]] = Some(0);
        for nbr in nbrs {
            adjacancy[[*v, *nbr]] = Some(1);
        }
    }
    floyd_warshall(&mut adjacancy);

    let dist = adjacancy.map(|x| x.unwrap());

    let flows = valves.into_iter().map(|(_, flow, _)| flow).collect();

    (flows, dist)
}

//pass flows as a reference to prevent having to repeatedly clone it
fn max_pressure<const N: usize>(
    flows: &mut Vec<usize>,
    adjacency: &Array2<usize>,
    positions: [usize; N],
    times: [usize; N],
) -> usize {
    let mut max = 0;
    for flow_index in 0..flows.len() {
        let flow = flows[flow_index];
        if flow == 0 {
            continue;
        }
        let worker_index = times.iter().position_max().unwrap();
        let current = positions[worker_index];
        if let Some(remaining_time) =
            times[worker_index].checked_sub(adjacency[[current, flow_index]] + 1)
        {
            //short circuit if naiive bound fails
            if max > flows.iter().sum::<usize>() * remaining_time {
                continue;
            }
            flows[flow_index] = 0;
            let mut new_times = times;
            new_times[worker_index] = remaining_time;
            let mut new_positions = positions;
            new_positions[worker_index] = flow_index;
            max = max.max(
                flow * remaining_time + max_pressure(flows, adjacency, new_positions, new_times),
            );
            flows[flow_index] = flow;
        }
    }
    max
}

fn main() {
    let timer = std::time::Instant::now();
    let input = std::fs::read_to_string("test_inputs/day16.txt")
        .unwrap()
        .trim()
        .to_owned();

    let (remaining, (start_index, valves)) = parse(&input).unwrap();

    assert_eq!(remaining.trim(), "");

    let (mut flows, dist) = get_flows_and_dist(valves);

    println!("{}", max_pressure(&mut flows, &dist, [start_index], [30]));

    println!(
        "{}",
        max_pressure(&mut flows, &dist, [start_index; 2], [26; 2])
    );
    println!("{:?}", timer.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("test_inputs/day16.txt")
            .unwrap()
            .trim()
            .to_owned();

        let (remaining, (start_index, valves)) = parse(&input).unwrap();

        assert_eq!(remaining.trim(), "");

        let (mut flows, dist) = get_flows_and_dist(valves);

        assert_eq!(
            1651,
            max_pressure::<1>(&mut flows, &dist, [start_index], [30])
        );
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("test_inputs/day16.txt")
            .unwrap()
            .trim()
            .to_owned();

        let (remaining, (start_index, valves)) = parse(&input).unwrap();

        assert_eq!(remaining.trim(), "");

        let (mut flows, dist) = get_flows_and_dist(valves);

        assert_eq!(
            1707,
            max_pressure(&mut flows, &dist, [start_index; 2], [26; 2])
        );
    }
}
