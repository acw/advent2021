use core::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Oopsie {
    #[error("Tried to parse an empty graph?")]
    EmptyGraph,
    #[error("Got a weird, inconsistent line width around {0}")]
    InconsistentGraphWidth(usize),
    #[error("Got weird character parsing graph: {0}")]
    BadCharacter(char),
}

pub struct Graph<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: TryFrom<char, Error = Oopsie>> Graph<T> {
    pub fn from_file_data(file_data: &str) -> Result<Graph<T>, Oopsie> {
        let mut data = Vec::with_capacity(file_data.len());
        let mut width = None;
        let mut height = 0;
        let mut temp_width = 0;

        for c in file_data.chars() {
            if c == '\n' {
                height += 1;

                if let Some(x) = width {
                    if x != temp_width {
                        return Err(Oopsie::InconsistentGraphWidth(height));
                    }
                } else {
                    width = Some(temp_width);
                }

                temp_width = 0;
            } else {
                data.push(T::try_from(c)?);
                temp_width += 1;
            }
        }

        if temp_width != 0 {
            height += 1;
        }

        if height == 0 {
            return Err(Oopsie::EmptyGraph);
        }

        Ok(Graph {
            data,
            width: width.unwrap(),
            height,
        })
    }
}

impl<T> Graph<T> {
    pub fn size(&self) -> usize {
        self.width * self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Point<T>> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(Point {
            x,
            y,
            value: &self.data[(y * self.width) + x],
        })
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<PointMut<T>> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(PointMut {
            x,
            y,
            value: &mut self.data[(y * self.width) + x],
        })
    }

    pub fn coordinates(&self) -> Coord<T> {
        Coord {
            graph: self,
            curx: 0,
            cury: 0,
        }
    }

    pub fn points(&self) -> Points<'_, T> {
        Points {
            graph: self,
            curx: 0,
            cury: 0,
        }
    }

    pub fn points_mut(&mut self) -> impl Iterator<Item = PointMut<T>> {
        let coords: Vec<(usize, usize)> = self.coordinates().collect();
        self.data
            .iter_mut()
            .zip(coords)
            .map(|(value, (x, y))| PointMut { x, y, value })
    }

    pub fn neighbors(&self, x: usize, y: usize) -> Vec<Point<T>> {
        let mut retval = Vec::new();

        if x > 0 {
            retval.push(self.get(x - 1, y).unwrap());
        }
        if y > 0 {
            retval.push(self.get(x, y - 1).unwrap());
        }
        if let Some(v) = self.get(x + 1, y) {
            retval.push(v);
        }
        if let Some(v) = self.get(x, y + 1) {
            retval.push(v);
        }

        retval
    }

    pub fn neighbor_points(&mut self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut retval = Vec::new();

        if y > 0 {
            if x > 0 {
                retval.push((x - 1, y - 1));
            }
            retval.push((x, y - 1));
            if x + 1 < self.width {
                retval.push((x + 1, y - 1))
            };
        }

        if x > 0 {
            retval.push((x - 1, y))
        };
        if x + 1 < self.width {
            retval.push((x + 1, y))
        };

        if y + 1 < self.width {
            if x > 0 {
                retval.push((x - 1, y + 1))
            };
            retval.push((x, y + 1));
            if x + 1 < self.width {
                retval.push((x + 1, y + 1))
            };
        }

        retval
    }
}

impl<T: fmt::Display> Graph<T> {
    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.get(x, y).unwrap().value);
            }
            println!();
        }
    }
}

#[derive(Clone)]
pub struct Point<'a, T> {
    pub x: usize,
    pub y: usize,
    pub value: &'a T,
}

impl<'a, T> fmt::Debug for Point<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl<'a, T> PartialEq for Point<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub struct PointMut<'a, T> {
    pub x: usize,
    pub y: usize,
    pub value: &'a mut T,
}

impl<'a, T> fmt::Debug for PointMut<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl<'a, T> PartialEq for PointMut<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub struct Coord<'a, T> {
    graph: &'a Graph<T>,
    curx: usize,
    cury: usize,
}

impl<'a, T> Iterator for Coord<'a, T> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.cury == self.graph.height {
            return None;
        }

        let next_value = (self.curx, self.cury);

        self.curx += 1;
        if self.curx == self.graph.width {
            self.curx = 0;
            self.cury += 1;
        }

        Some(next_value)
    }
}

pub struct Points<'a, T> {
    graph: &'a Graph<T>,
    curx: usize,
    cury: usize,
}

impl<'a, T> Iterator for Points<'a, T>
where
    T: Clone + TryFrom<char, Error = Oopsie>,
{
    type Item = Point<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cury == self.graph.height {
            return None;
        }

        let next_value = self.graph.get(self.curx, self.cury)?;

        self.curx += 1;
        if self.curx == self.graph.width {
            self.curx = 0;
            self.cury += 1;
        }

        Some(next_value)
    }
}
