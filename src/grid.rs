use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
    simd::{Mask, MaskElement, Simd, SimdElement, cmp::SimdPartialEq},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pos {
    pub r: isize,
    pub c: isize,
}

impl Pos {
    pub fn new(r: isize, c: isize) -> Self {
        Pos { r, c }
    }

    pub fn left(&self) -> Self {
        Pos {
            r: self.r,
            c: self.c - 1,
        }
    }

    pub fn right(&self) -> Self {
        Pos {
            r: self.r,
            c: self.c + 1,
        }
    }

    pub fn down(&self) -> Self {
        Pos {
            r: self.r + 1,
            c: self.c,
        }
    }

    pub fn up(&self) -> Self {
        Pos {
            r: self.r - 1,
            c: self.c,
        }
    }

    pub fn neighbors8(self) -> impl Iterator<Item = Pos> {
        const DELTAS: [(isize, isize); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        DELTAS
            .into_iter()
            .map(move |(dr, dc)| Pos::new(self.r + dr, self.c + dc))
    }
}

pub struct Grid<T, const PAD: usize = 0> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T, 0> {
    pub fn from_string(delimiter: char, input: &str) -> Self
    where
        T: From<u8> + Clone,
    {
        let lines: Vec<&str> = input.split(delimiter).collect();
        let height = lines.len();
        let width = lines[0].len();

        let mut data = vec![T::from(b'0'); width * height];
        for (r, line) in lines.iter().enumerate() {
            let row_offset = r * width;
            for (c, b) in line.bytes().enumerate() {
                data[row_offset + c] = T::from(b);
            }
        }

        Grid {
            data,
            width,
            height,
        }
    }
}

impl<const PAD: usize> Grid<u8, PAD> {
    pub fn from_string_with_padding(delimiter: char, fill: char, input: &str) -> Self {
        let unpadded_width = input.find(delimiter).expect("Input did not have a line");
        let unpadded_height = input.len() / unpadded_width;
        let width = unpadded_width + PAD * 2;
        let height = unpadded_height + PAD * 2;

        let mut grid_data = vec![fill as u8; width * height];

        for (r, line) in input.split(delimiter).enumerate() {
            let bytes = line.as_bytes();

            let len = bytes.len();

            let start = (r + PAD) * width + PAD;
            let end = start + len;

            if start < grid_data.len() && end <= grid_data.len() {
                grid_data[start..end].copy_from_slice(bytes);
            }
        }

        Grid {
            data: grid_data,
            width,
            height,
        }
    }
}

impl<T, const PAD: usize> Grid<T, PAD> {
    pub fn new(width: usize, height: usize, default: T) -> Self
    where
        T: Clone,
    {
        Grid {
            data: vec![default; width * height],
            width: width + (PAD * 2),
            height: height + (PAD * 2),
        }
    }

    pub fn transposed(self) -> Grid<T, PAD>
    where
        T: Clone,
    {
        let mut transposed = Grid::<T, PAD>::new(self.height(), self.width(), self.data[0].clone());

        for r in 0..self.width {
            for c in 0..self.height {
                transposed[r][c] = self[c][r].clone();
            }
        }
        transposed
    }

    pub fn width(&self) -> usize {
        self.width - (PAD * 2)
    }

    pub fn height(&self) -> usize {
        self.height - (PAD * 2)
    }

    pub fn row_scan_positions(&self) -> impl Iterator<Item = Pos> + use<T, PAD> {
        let (w, h) = (self.width, self.height);
        (PAD..h - PAD).flat_map(move |y| {
            (PAD..w - PAD).map(move |x| Pos::new((x - PAD) as isize, (y - PAD) as isize))
        })
    }

    pub fn iter_rows(&self) -> impl Iterator<Item = &[T]> {
        self.data.chunks(self.width)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn position<F>(&self, f: F) -> Option<Pos>
    where
        F: Fn(&T) -> bool,
    {
        self.row_scan_positions().find(|pos| f(&self[*pos]))
    }

    pub fn count_eq(&self, target: T) -> usize
    where
        T: SimdElement,
        T::Mask: MaskElement,
        Simd<T, 64>: SimdPartialEq<Mask = Mask<T::Mask, 64>>,
    {
        let simd_target = Simd::<T, 64>::splat(target);
        let mut count: usize = 0;

        for chunk in self.data.chunks(64) {
            let v = Simd::<T, 64>::load_or(chunk, simd_target);
            let mask = v.simd_eq(simd_target);
            count += mask.to_bitmask().count_ones() as usize;
        }
        count
    }
}

impl<T, const PAD: usize> Index<usize> for Grid<T, PAD> {
    type Output = [T];

    fn index(&self, row: usize) -> &Self::Output {
        let start = row * self.width;
        let end = start + self.width;
        &self.data[start..end]
    }
}

impl<T, const PAD: usize> Index<Pos> for Grid<T, PAD> {
    type Output = T;

    fn index(&self, pos: Pos) -> &Self::Output {
        let start = (pos.r + PAD as isize) as usize * self.width;
        let c = (pos.c + PAD as isize) as usize;
        &self.data[start + c]
    }
}

impl<T, const PAD: usize> IndexMut<usize> for Grid<T, PAD> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let start = row * self.width;
        let end = start + self.width;
        &mut self.data[start..end]
    }
}

impl<T, const PAD: usize> IndexMut<Pos> for Grid<T, PAD> {
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        let start = (pos.r + PAD as isize) as usize * self.width;
        let c = (pos.c + PAD as isize) as usize;
        &mut self.data[start + c]
    }
}

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.height {
            for c in 0..self.width {
                write!(f, "{:?} ", self[r][c])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
