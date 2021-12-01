use std::fs;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use array2d::Array2D;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: u32,
    position: (usize, usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.0.cmp(&other.position.0))
            .then_with(|| self.position.1.cmp(&other.position.1))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn day15_part1() {
    let file_string = fs::read_to_string("day15/input.txt").expect("File not found");
    let lines = file_string.lines();

    let map_line = |line: &str| -> Vec<u32> {
        line.chars().map(|x| x.to_digit(10).unwrap()).collect()
    };

    let chitons: Array2D<u32> = Array2D::from_rows(&lines.map(map_line).collect::<Vec<Vec<_>>>());

    let get_nbrs = |(i,j)| -> Vec<(usize, usize)> {
        let mut nbrs: Vec<(usize, usize)> = Vec::new();
        if i > 0 {
            nbrs.push((i-1, j));
        }
        if i < chitons.column_len()-1 {
            nbrs.push((i+1, j));
        }
        if j > 0 {
            nbrs.push((i, j-1))
        }
        if j < chitons.row_len()-1 {
            nbrs.push((i, j+1))
        }
        nbrs
    };

    let mut heap = BinaryHeap::new();
    let mut dist: Array2D<u32> = Array2D::filled_with(u32::MAX, chitons.column_len(), chitons.row_len());
    let source = (0,0);
    let dest = (chitons.column_len()-1, chitons.row_len()-1);

    /*for i in 0..chitons.column_len() {
        for j in 0..chitons.row_len() {
            heap.push(State { cost: chitons[(i,j)], position: (i,j) });
        }
    }*/
    heap.push(State { cost: 0, position: (0,0) });
    dist[source] = 0;

    while let Some(State { cost, position }) = heap.pop() {
        //println!("{:?}", heap);
        if position == dest {
            break;
        }

        if cost > dist[position] { continue; }

        for nbr in get_nbrs(position) {
            let alt = dist[position] + chitons[nbr];
            if alt < dist[nbr] {
                heap.push(State { cost: alt, position: nbr });
                dist[nbr] = alt;
            }
        }
    }

        
    let best_risk = dist[dest];
    println!("{:?}", best_risk);
} 

pub fn day15_part1_dp() {
    let file_string = fs::read_to_string("day15/input.txt").expect("File not found");
    let lines = file_string.lines();

    let map_line = |line: &str| -> Vec<u32> {
        line.chars().map(|x| x.to_digit(10).unwrap()).collect()
    };

    let chitons: Array2D<u32> = Array2D::from_rows(&lines.map(map_line).collect::<Vec<Vec<_>>>());
    let WIDTH = chitons.column_len();
    let HEIGHT = chitons.row_len();
    let mut dp: Array2D<u32> = Array2D::filled_with(u32::MAX, WIDTH, HEIGHT);
    dp[(WIDTH-1, HEIGHT-1)] = 0;
    
    let mut calc_dp = |(x,y)| {
        //println!("{}, {}", x, y);
        let down_cost = 
        if y < HEIGHT-1 {
            chitons[(x, y+1)] + dp[(x, y+1)]
        } else {
            u32::MAX
        };
        let right_cost = 
        if x < WIDTH-1 {
            chitons[(x+1, y)] + dp[(x+1, y)]
        } else {
            u32::MAX
        };
        dp[(x,y)] = std::cmp::min(down_cost, right_cost);
    };

    for i in 2..WIDTH {
        for j in 0..i {
            let pos = (WIDTH-i+j, HEIGHT-1-j);
            calc_dp(pos);
        }
    }

    for i in (0..WIDTH-1).rev() {
        for j in 0..=i+1 {
            let pos = (j, i+1-j);
            calc_dp(pos);
        }
    }

    calc_dp((0,0));
        
    let best_risk = dp[(0,0)];
    println!("{}", best_risk);
} 


pub fn day15_part2() {
    let file_string = fs::read_to_string("day15/input.txt").expect("File not found");
    let lines = file_string.lines();

    let map_line = |line: &str| -> Vec<u32> {
        line.chars().map(|x| x.to_digit(10).unwrap()).collect()
    };

    let chitons: Array2D<u32> = Array2D::from_rows(&lines.map(map_line).collect::<Vec<Vec<_>>>());
    let WIDTH = chitons.column_len();
    let HEIGHT = chitons.row_len();
    let mut big_chitons: Array2D<u32> = Array2D::filled_with(0, WIDTH*5, HEIGHT*5);

    for i in 0..5 {
        for j in 0..5 {
            let offset = i+j; 
            // copy in chitons + offset for each block
            for ii in 0..WIDTH {
                for jj in 0..HEIGHT {
                    let x = i*WIDTH + ii;
                    let y = j*HEIGHT + jj;
                    big_chitons[(x,y)] = (chitons[(ii,jj)] + offset as u32 - 1) % 9 + 1;
                }
            }
        }
    }

    let WIDTH = WIDTH*5;
    let HEIGHT = HEIGHT*5;

    let get_nbrs = |(i,j)| -> Vec<(usize, usize)> {
        let mut nbrs: Vec<(usize, usize)> = Vec::new();
        if i > 0 {
            nbrs.push((i-1, j));
        }
        if i < WIDTH-1 {
            nbrs.push((i+1, j));
        }
        if j > 0 {
            nbrs.push((i, j-1))
        }
        if j < HEIGHT-1 {
            nbrs.push((i, j+1))
        }
        nbrs
    };

    let mut heap = BinaryHeap::new();
    let mut dist: Array2D<u32> = Array2D::filled_with(u32::MAX, WIDTH, HEIGHT);
    let source = (0,0);
    let dest = (WIDTH-1, HEIGHT-1);

    heap.push(State { cost: 0, position: (0,0) });
    dist[source] = 0;

    while let Some(State { cost, position }) = heap.pop() {
        //println!("{:?}", heap);
        if position == dest {
            break;
        }

        if cost > dist[position] { continue; }

        for nbr in get_nbrs(position) {
            //println!("{:?}", nbr);
            let alt = dist[position] + big_chitons[nbr];
            if alt < dist[nbr] {
                heap.push(State { cost: alt, position: nbr });
                dist[nbr] = alt;
            }
        }
    }

        
    let best_risk = dist[dest];
    println!("{:?}", best_risk);
    //let best_risk = dp[(0,0)];
    //println!("{}", best_risk);
}