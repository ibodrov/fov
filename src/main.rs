const MAP_WIDTH: i32 = 8;
const MAP_HEIGHT: i32 = 8;

struct Map([u8; (MAP_WIDTH * MAP_HEIGHT) as usize]);

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x: x, y: y }
    }
}

const MAP: Map = Map([1, 1, 1, 1, 1, 1, 1, 1,
                      1, 0, 0, 0, 0, 0, 0, 1,
                      1, 0, 0, 0, 0, 0, 0, 1,
                      1, 0, 0, 1, 1, 1, 1, 1,
                      1, 0, 0, 0, 0, 0, 0, 1,
                      1, 0, 0, 0, 0, 0, 0, 1,
                      1, 0, 0, 0, 0, 0, 0, 1,
                      1, 1, 1, 1, 1, 1, 1, 1]);

fn idx(x: i32, y: i32) -> usize {
    let v = y * MAP_HEIGHT + x;
    v as usize
}

impl Map {
    fn get_at(&self, x: i32, y: i32) -> u8 {
        self.0[idx(x, y)]
    }

    fn set_at(&mut self, x: i32, y: i32, v: u8) {
        self.0[idx(x, y)] = v;
    }

    fn print(&self) {
        let m = self.0;
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let i = idx(x, y);
                let v = m[i];
                print!("{}, ", v);
            }
            println!("");
        }
    }

    fn line_iter(&self, p0: Point, p1: Point) -> BresenhamIter {
        fn delta(v: i32) -> i32 {
            if v > 0 {
                1
            } else if v < 0 {
                -1
            } else {
                0
            }
        }

        let w = p1.x - p0.x;
        let h = p1.y - p0.y;
        let dx1 = delta(w);
        let dy1 = delta(h);

        let mut dx2 = delta(w);
        let mut dy2 = 0;

        let mut longest = w.abs();
        let mut shortest = h.abs();

        if !(longest > shortest) {
            longest = h.abs();
            shortest = w.abs();
            dy2 = delta(h);
            dx2 = 0;
        }

        let numerator = longest >> 1;

        BresenhamIter {
            i: 0,
            x: p0.x,
            y: p0.y,
            numerator: numerator,
            longest: longest,
            shortest: shortest,
            dx1: dx1,
            dy1: dy1,
            dx2: dx2,
            dy2: dy2,
        }
    }
}

impl Clone for Map {
    fn clone(&self) -> Self {
        Map(self.0)
    }
}

struct BresenhamIter {
    i: i32,
    x: i32,
    y: i32,
    numerator: i32,
    longest: i32,
    shortest: i32,
    dx1: i32,
    dy1: i32,
    dx2: i32,
    dy2: i32,
}

#[derive(Debug)]
struct BresenhamStep {
    x: i32,
    y: i32,
}

impl Iterator for BresenhamIter {
    type Item = BresenhamStep;

    fn next(&mut self) -> Option<BresenhamStep> {
        if self.i > self.longest {
            return None;
        }

        self.i += 1;

        let x = self.x;
        let y = self.y;

        self.numerator += self.shortest;
        if self.numerator >= self.longest {
            self.numerator -= self.longest;
            self.x += self.dx1;
            self.y += self.dy1;
        } else {
            self.x += self.dx2;
            self.y += self.dy2;
        }

        Some(BresenhamStep { x: x, y: y })
    }
}

fn main() {
    let mut m = MAP.clone();
    let p0 = Point::new(2, 2);


    fn trace(m: &mut Map, p0: Point, p1: Point) {
        for BresenhamStep { x: sx, y: sy } in m.line_iter(p0, p1) {
            if m.get_at(sx, sy) == 1 {
                break;
            } else {
                m.set_at(sx, sy, 2);
            }
        }
    }

    let y = 0;
    for x in 0..MAP_WIDTH {
        trace(&mut m, p0, Point::new(x, y));
    }

    let x = MAP_WIDTH - 1;
    for y in 0..MAP_HEIGHT {
        trace(&mut m, p0, Point::new(x, y));
    }

    let x = 0;
    for y in 0..MAP_HEIGHT {
        trace(&mut m, p0, Point::new(x, y));
    }

    let y = MAP_HEIGHT - 1;
    for x in 0..MAP_WIDTH {
        trace(&mut m, p0, Point::new(x, y));
    }

    m.print();
}
