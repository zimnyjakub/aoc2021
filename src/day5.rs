use std::fs;

pub fn day5() {
    let string = fs::read_to_string("hydrothermal_vents.txt").unwrap();
    let mut lines = string.lines();

    let vents: Vec<HydrothermalVentCloud> = lines
        .map(|s| s.to_string().into())
        .collect();

    let mut ocean_floor = OceanFloor::new(vents);

    ocean_floor.compute_cloud_map();

    // ocean_floor.print_map();
    println!("aoc answer is: {}", ocean_floor.get_aoc2021_answer());
}

struct OceanFloor {
    clouds: Vec<HydrothermalVentCloud>,

    cloud_map: Vec<Vec<i32>>,
}

impl OceanFloor {
    fn new(clouds: Vec<HydrothermalVentCloud>) -> OceanFloor {
        OceanFloor {
            clouds,
            cloud_map: Vec::new(),
        }
    }

    fn compute_cloud_map(&mut self) {
        let extents = self.get_map_extents();
        println!("map extents: x:{}, y:{}", &extents.0, &extents.1);

        for _ in 0..extents.0 + 10 {
            let mut row = Vec::new();
            for _ in 0..extents.1 + 10 {
                row.push(0)
            }
            self.cloud_map.push(row)
        }

        for line in &self.clouds {
            println!("{:?}", line);
            let bump_points = line.expand();
            println!("{:?}", bump_points);
            for point in bump_points {
                self.cloud_map[point.0 as usize][point.1 as usize] += 1;
            }
        }
    }

    fn get_aoc2021_answer(&self) -> usize {
        self.cloud_map.iter().flat_map(|f| f.iter().filter(|it| it.ge(&&2))).cloned().collect::<Vec<i32>>().len()
    }

    fn get_map_extents(&self) -> Point {
        let mut max_x = std::cmp::max(
            self.clouds.iter().map(|it| it.begin.0).max().unwrap(),
            self.clouds.iter().map(|it| it.end.0).max().unwrap(),
        );
        let mut max_y = std::cmp::max(
            self.clouds.iter().map(|it| it.begin.1).max().unwrap(),
            self.clouds.iter().map(|it| it.end.1).max().unwrap(),
        );
        Point(max_x + 1, max_y + 1)
    }

    fn print_map(&self) {
        for x in &self.cloud_map {
            for v in x {
                print!("{:2} ", v);
            }
            println!();
        }
    }
}


#[derive(Debug, Clone, Copy)]
struct HydrothermalVentCloud {
    begin: Point,
    end: Point,
}

impl HydrothermalVentCloud {
    fn expand(&self) -> Vec<Point> {
        let mut points = Vec::new();

        if self.end.0 == self.begin.0 {
            for i in std::cmp::min(self.end.1, self.begin.1)..std::cmp::max(self.end.1, self.begin.1) + 1 {
                points.push(Point(i, self.end.0));
            }
        }
        if self.end.1 == self.begin.1 {
            for i in std::cmp::min(self.end.0, self.begin.0)..std::cmp::max(self.end.0, self.begin.0) + 1 {
                points.push(Point(self.begin.1, i));
            }
        }

        if std::cmp::max(self.end.1, self.begin.1) - std::cmp::min(self.end.1, self.begin.1) ==
            std::cmp::max(self.end.0, self.begin.0) - std::cmp::min(self.end.0, self.begin.0) {
            for x in std::cmp::min(self.end.0, self.begin.0)..std::cmp::max(self.end.1, self.begin.1) + 1 {
                for y in std::cmp::min(self.end.1, self.begin.1)..std::cmp::max(self.end.0, self.begin.0) + 1 {
                    points.push(Point(x,y));
                }
            }
        }

        points
    }
}

#[derive(Debug, Clone, Copy)]
struct Point(i32, i32);

impl From<String> for HydrothermalVentCloud {
    fn from(s: String) -> Self {
        let parts: Vec<&str> = s
            .split("->")
            .map(|x| x.trim())
            .collect();

        HydrothermalVentCloud {
            begin: parts[0].into(),
            end: parts[1].into(),
        }
    }
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split(',').collect();
        Point(parts[0].parse::<i32>().unwrap(), parts[1].parse::<i32>().unwrap())
    }
}