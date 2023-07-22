use crate::models::{Coordinates, Orientation};
use ndarray::{s, Ix2, SliceInfo, SliceInfoElem};

/// A trait for structs that is linear on the board - has a start point, orientation and length.
pub trait Segment {
    /// Return the x position of the segment.
    fn x(&self) -> usize;

    /// Return the y position of the segment.
    fn y(&self) -> usize;

    /// Return the length of the segment.
    fn length(&self) -> usize;

    /// Return the orientation of the segment.
    fn orientation(&self) -> &Orientation;

    /// Returns the starting coordinates in an array.
    fn coordinates(&self) -> [usize; 2] {
        [self.x(), self.y()]
    }

    /// Returns the [`SliceInfo`] representing this [`Segment`].
    fn slice(&self) -> SliceInfo<[SliceInfoElem; 2], Ix2, Ix2> {
        let x = self.x();
        let y = self.y();
        let length = self.length();

        match self.orientation() {
            Orientation::Down => s![x..x + 1, y..(y + length)],
            Orientation::Right => s![x..(x + length), y..y + 1],
        }
    }

    /// Returns the `n`-th [`Coordinates`] of this [`Segment`].
    fn get(&self, n: usize) -> Option<Coordinates> {
        (n < self.length()).then(|| match self.orientation() {
            Orientation::Down => Coordinates {
                x: self.x(),
                y: self.y() + n,
            },
            Orientation::Right => Coordinates {
                x: self.x() + n,
                y: self.y(),
            },
        })
    }

    /// Iterate through the [`Coordinates`] in this [`Segment`].
    fn iter(&self) -> iterator::IterSegment<'_, Self>
    where
        Self: Sized,
    {
        iterator::IterSegment::new(self)
    }
}

mod iterator {
    use super::*;

    /// A special iterator class for iterating [`Coordinates`] through a [`Segment`].
    pub struct IterSegment<'s, T>
    where
        T: Segment,
    {
        segment: &'s T,
        count: usize,
    }

    impl<'s, T> IterSegment<'s, T>
    where
        T: Segment,
    {
        /// Create a new [`IterSegment`] from something that implements [`Segment`].
        pub fn new(segment: &'s T) -> Self {
            Self { segment, count: 0 }
        }
    }

    impl<'s, T> Iterator for IterSegment<'s, T>
    where
        T: Segment,
    {
        type Item = Coordinates;

        fn next(&mut self) -> Option<Self::Item> {
            let item = self.segment.get(self.count);
            self.count += 1;
            item
        }
    }
}
