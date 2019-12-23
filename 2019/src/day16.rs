pub struct Repeat<I: Iterator> {
    iter: I,
    item: Option<I::Item>,
    n: usize,
    counter: usize,
}

impl<I> Iterator for Repeat<I>
where
    I: Iterator,
    I::Item: Copy,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter == 0 {
            self.counter = self.n;
            self.item = self.iter.next();
        } else {
            self.counter -= 1;
        }

        match self.item {
            Some(item) => Some(item),
            None => None,
        }
    }
}

pub trait RepeatIteratorAdapter: Sized
where
    Self: Iterator,
{
    fn repeat(self, n: usize) -> Repeat<Self>;
}

impl<I> RepeatIteratorAdapter for I
where
    I: Iterator,
{
    fn repeat(self, n: usize) -> Repeat<Self> {
        Repeat {
            iter: self,
            item: None,
            n: n - 1,
            counter: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day16() {
        let input = vec![-1, 0, 1];
        let expected = vec![-1, -1, -1, 0, 0, 0, 1, 1, 1];
        assert_eq!(input.into_iter().repeat(3).collect::<Vec<i32>>(), expected);
    }
}
