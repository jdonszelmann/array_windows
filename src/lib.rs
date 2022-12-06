pub trait ArrayWindowsExt<I: Iterator, const N: usize> {
    fn array_windows(self) -> ArrayWindows<I, N>;
}

impl<I, const N: usize> ArrayWindowsExt<Self, N> for I
where
    I: Iterator,
    <I as Iterator>::Item: Clone,
    Self: Sized,
{
    fn array_windows(mut self) -> ArrayWindows<Self, N> {
        // TODO: don't use a vec?
        let mut res = Vec::with_capacity(N);
        for _ in 0..N {
            let Some(i) = self.next() else {
                return ArrayWindows(None);
            };
            res.push(i);
        }

        let Ok(arr) = res.try_into() else {
            return ArrayWindows(None);
        };

        ArrayWindows(Some((self, arr, false)))
    }
}

pub struct ArrayWindows<I: Iterator, const N: usize>(Option<(I, [<I as Iterator>::Item; N], bool)>);

impl<I, const N: usize> Iterator for ArrayWindows<I, N>
where
    I: Iterator,
    <I as Iterator>::Item: Clone,
{
    type Item = [<I as Iterator>::Item; N];

    fn next(&mut self) -> Option<Self::Item> {
        let Some((iter, buf, update)) = &mut self.0 else {
            return None;
        };

        if *update {
            let nxt = iter.next()?;
            buf[0] = nxt;
            buf.rotate_left(1);
        } else {
            *update = true;
        }

        Some(buf.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::ArrayWindowsExt;

    #[test]
    fn test_array_windows() {
        let mut iter = [1, 2, 3, 4].into_iter().array_windows();

        assert_eq!(iter.next(), Some([1, 2]));
        assert_eq!(iter.next(), Some([2, 3]));
        assert_eq!(iter.next(), Some([3, 4]));
    }

    #[test]
    fn test_array_windows_equal() {
        let mut iter = [1, 2, 3].into_iter().array_windows();

        assert_eq!(iter.next(), Some([1, 2, 3]));
    }
}
