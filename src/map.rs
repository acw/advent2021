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

impl<T: Clone + TryFrom<char, Error = Oopsie>> Graph<T> {
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

    pub fn points(&self) -> Points<'_, T> {
        Points {
            graph: self,
            curx: 0,
            cury: 0,
        }
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
