extern crate ndarray;

use ndarray::prelude::*;
use std::time::Instant;

#[derive(Debug)]
struct WGraph {
    adj_matrix: ndarray::prelude::Array2<f64>,
    visited: ndarray::prelude::Array1<f64>,
    size: i32,
}

impl WGraph {
    fn add_biedge(&mut self, source: usize, destination: usize, weight: f64) {
        self.adj_matrix[[source - 1, destination - 1]] = weight;
        self.adj_matrix[[destination - 1, source - 1]] = weight;
    }

    fn find_nearest(&mut self, current: usize) -> usize {
        let mut min: f64 = 0.0;
        let mut index: usize = 0;
        let _size = self.size;
        for i in 0usize..(_size as usize) {
            if self.adj_matrix[[current, i]] != 0.0 {
                if min == 0.0 && self.visited[Ix1(i)] == 0.0 {
                    min = self.adj_matrix[[current, i]];
                    index = i;
                }
                if self.adj_matrix[[current, i]] < min && self.visited[Ix1(i)] == 0.0 {
                    min = self.adj_matrix[[current, i]];
                    index = i;
                }
            }
        }
        index
    }

    fn shortest_path(&mut self, source: usize, destination: usize) -> f64 {
        let mut current = source - 1;

        self.visited[current] = 1.0;

        let _size = self.size;

        let mut distances = ndarray::Array1::<f64>::zeros(Ix1(_size as usize));

        for i in 0usize..(_size as usize) {
            if self.adj_matrix[[current, i]] != 0.0 {
                distances[Ix1(i)] = self.adj_matrix[[current, i]];
            }
        }

        for _i in 0usize..(_size as usize) {
            for j in 0usize..(_size as usize) {
                if self.adj_matrix[[current, j]] != 0.0 {
                    if distances[Ix1(j)] == 0.0 && self.visited[Ix1(j)] == 0.0 {
                        distances[Ix1(j)] = self.adj_matrix[[current, j]] + distances[current];
                        self.visited[current] = 1.0;
                    }

                    if distances[Ix1(j)] != 0.0
                        && ((self.adj_matrix[[current, j]] + distances[Ix1(current)]
                            < distances[Ix1(j)])
                            && self.visited[Ix1(j)] == 0.0)
                    {
                        distances[Ix1(j)] = self.adj_matrix[[current, j]] + distances[Ix1(current)];
                        self.visited[ndarray::Ix1(current)] = 1.0;
                    }
                }
            }
            current = WGraph::find_nearest(self, current);
        }
        distances[ndarray::Ix1(destination - 1)]
    }
}

fn main() {
    let before = Instant::now();
    let mut graph = WGraph {
        adj_matrix: ndarray::Array2::<f64>::zeros((5, 5)),
        visited: ndarray::Array1::<f64>::zeros(5),
        size: 5,
    };

    graph.add_biedge(1, 2, 6.0);
    graph.add_biedge(1, 4, 1.0);
    graph.add_biedge(4, 2, 2.0);
    graph.add_biedge(4, 5, 1.0);
    graph.add_biedge(5, 2, 2.0);
    graph.add_biedge(3, 2, 5.0);
    graph.add_biedge(3, 5, 5.0);
    println!("{}", graph.shortest_path(1, 3));
    println!("Elapsed time: {:.2?}", before.elapsed());
}
