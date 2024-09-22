use axum::{routing::post, Router};

use crate::error::AppError;

async fn integers(input: String) -> Result<String, AppError> {
    let mut nums: Vec<u64> = input.lines().flat_map(|n| n.parse::<u64>()).collect();
    nums.sort();
    let v: i64 = nums
        .iter()
        .enumerate()
        .map(|(i, &n)| {
            let n = n as i64;
            match i % 2 {
                1 => -n,
                _ => n,
            }
        })
        .sum();
    let packages = String::from("🎁");
    Ok(packages.repeat(v as usize))
}

#[derive(Debug)]
struct Star {
    x: i32,
    y: i32,
    z: i32,
}

impl Star {
    fn distance(&self, other: &Star) -> f32 {
        let squared_sum: f32 = (other.x as f32 - self.x as f32).powf(2.0)
            + (other.y as f32 - self.y as f32).powf(2.0)
            + (other.z as f32 - self.z as f32).powf(2.0);
        squared_sum.sqrt()
    }
}

impl From<&str> for Star {
    fn from(s: &str) -> Self {
        let mut coords = s.split(' ');
        let x = coords.next().unwrap().parse::<i32>().unwrap();
        let y = coords.next().unwrap().parse::<i32>().unwrap();
        let z = coords.next().unwrap().parse::<i32>().unwrap();
        Self { x, y, z }
    }
}

#[derive(Debug, Clone)]
struct Portal {
    source: usize,
    destination: usize,
}

impl From<&str> for Portal {
    fn from(s: &str) -> Self {
        let mut path = s.split(' ');
        let source = path.next().unwrap().parse::<usize>().unwrap();
        let destination = path.next().unwrap().parse::<usize>().unwrap();
        Self {
            source,
            destination,
        }
    }
}

#[derive(Debug)]
struct StarMap {
    stars: Vec<Star>,
    portals: Vec<Portal>,
}

impl From<String> for StarMap {
    fn from(s: String) -> Self {
        let mut lines = s.lines();
        let number_of_stars = lines.next().unwrap().parse::<usize>().unwrap();
        let mut stars = Vec::new();
        for _i in 0..number_of_stars {
            stars.push(Star::from(lines.next().unwrap()));
        }
        let number_of_portals = lines.next().unwrap().parse::<usize>().unwrap();
        let mut portals = Vec::new();
        for _i in 0..number_of_portals {
            portals.push(Portal::from(lines.next().unwrap()));
        }
        Self { stars, portals }
    }
}

impl StarMap {
    fn last_star_id(&self) -> usize {
        self.stars.len() - 1
    }

    fn get_star(&self, id: usize) -> Option<&Star> {
        self.stars.iter().nth(id)
    }

    fn portals_from_source(&self, source: usize) -> Vec<Portal> {
        self.portals
            .iter()
            .cloned()
            .filter(|p| p.source == source)
            .collect()
    }

    fn find_all_paths(&self) -> Vec<Vec<usize>> {
        let mut result: Vec<Vec<usize>> = Vec::new();
        for portal in self.portals_from_source(0) {
            let mut path: Vec<usize> = Vec::new();
            path.push(portal.source);
            path.push(portal.destination);
            self.find_path(&mut result, portal.destination, &mut path);
        }
        result
    }

    fn find_path(&self, paths: &mut Vec<Vec<usize>>, start_star_id: usize, path: &mut Vec<usize>) {
        if start_star_id == self.last_star_id() {
            paths.push(path.clone());
        } else {
            for portal in self.portals_from_source(start_star_id) {
                if !path.contains(&portal.destination) {
                    path.push(portal.destination);
                    self.find_path(paths, portal.destination, path);
                }
                path.pop();
            }
        }
    }
}

async fn rocket(input: String) -> String {
    let star_map = StarMap::from(input);
    let result = star_map.find_all_paths();
    let min_path = result
        .iter()
        .min_by(|v1, v2| v1.len().cmp(&v2.len()))
        .unwrap();
    let mut distance: f32 = 0.0;
    let mut path_iter = min_path.iter().peekable();
    while let Some(star) = path_iter.next() {
        if let Some(other) = path_iter.peek() {
            let star = star_map.get_star(*star).unwrap();
            let other = star_map.get_star(**other).unwrap();
            let curr_distance = star.distance(other);
            distance += curr_distance;
        }
    }
    format!("{} {:.3}", min_path.len() - 1, distance)
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/22/integers", post(integers))
        .route("/22/rocket", post(rocket))
}
