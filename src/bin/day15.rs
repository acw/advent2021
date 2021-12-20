use pathfinding::directed::dijkstra::dijkstra;

use advent2021::map::{Graph, Oopsie};

const TEST_DATA: &str = include_str!("../../data/day15t.txt");
const REAL_DATA: &str = include_str!("../../data/day15a.txt");

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Risk(usize);

impl TryFrom<char> for Risk {
    type Error = Oopsie;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        value
            .to_digit(10)
            .map(|x| Risk(x as usize))
            .ok_or(Oopsie::BadCharacter(value))
    }
}

impl Risk {
    fn inc(&mut self) {
        self.0 = if self.0 == 9 { 1 } else { self.0 + 1 };
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct Path {
    points: Vec<(usize, usize)>,
    cost: usize,
}

fn shortest_path(graph: &Graph<Risk>) -> Path {
    let target_x = graph.width - 1;
    let target_y = graph.height - 1;

    let result = dijkstra(
        &(0, 0),
        |(x, y)| {
            graph
                .neighbors(*x, *y)
                .into_iter()
                .map(|p| ((p.x, p.y), p.value.0))
        },
        |(x, y)| x == &target_x && y == &target_y,
    );

    if let Some((points, cost)) = result {
        Path { points, cost }
    } else {
        panic!("Couldn't find a result.");
    }
}

fn embiggen(graph: &Graph<Risk>) -> Graph<Risk> {
    let region8 = graph.clone();
    let mut region9 = region8.clone();
    region9.points_mut().for_each(|x| x.value.inc());
    let mut region1 = region9.clone();
    region1.points_mut().for_each(|x| x.value.inc());
    let mut region2 = region1.clone();
    region2.points_mut().for_each(|x| x.value.inc());
    let mut region3 = region2.clone();
    region3.points_mut().for_each(|x| x.value.inc());
    let mut region4 = region3.clone();
    region4.points_mut().for_each(|x| x.value.inc());
    let mut region5 = region4.clone();
    region5.points_mut().for_each(|x| x.value.inc());
    let mut region6 = region5.clone();
    region6.points_mut().for_each(|x| x.value.inc());
    let mut region7 = region6.clone();
    region7.points_mut().for_each(|x| x.value.inc());

    Graph::from_subgraphs(
        5,
        5,
        &[
            region8,
            region9.clone(),
            region1.clone(),
            region2.clone(),
            region3.clone(),
            region9.clone(),
            region1.clone(),
            region2.clone(),
            region3.clone(),
            region4.clone(),
            region1.clone(),
            region2.clone(),
            region3.clone(),
            region4.clone(),
            region5.clone(),
            region2.clone(),
            region3.clone(),
            region4.clone(),
            region5.clone(),
            region6.clone(),
            region3.clone(),
            region4.clone(),
            region5.clone(),
            region6.clone(),
            region7.clone(),
        ],
    )
}

fn main() -> Result<(), Oopsie> {
    let test_graph: Graph<Risk> = Graph::from_file_data(TEST_DATA)?;
    let real_graph: Graph<Risk> = Graph::from_file_data(REAL_DATA)?;

    println!("Test shortest path: {:?}", shortest_path(&test_graph));
    println!("Real shortest path: {:?}", shortest_path(&real_graph));

    let embiggened_test = embiggen(&test_graph);
    println!(
        "Larger test shortest path: {:?}",
        shortest_path(&embiggened_test)
    );
    let embiggened_real = embiggen(&real_graph);
    println!(
        "Larger real shortest path: {:?}",
        shortest_path(&embiggened_real)
    );

    Ok(())
}
