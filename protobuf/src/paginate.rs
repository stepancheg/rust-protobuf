pub trait PaginatableIterator: Sized {
    fn paginate(self, page: usize) -> Paginate<Self>;
}

impl<T, U : Iterator<Item = T>> PaginatableIterator for U {
    fn paginate(self, page: usize) -> Paginate<U> {
        Paginate {
            iter: self,
            page: page,
        }
    }
}

pub struct Paginate<I> {
    iter: I,
    page: usize,
}

impl<E, I : Iterator<Item = E>> Iterator for Paginate<I> {
    type Item = Vec<E>;

    fn next(&mut self) -> Option<Vec<E>> {
        let mut r = Vec::new();
        for _ in 0..self.page {
            match self.iter.next() {
                Some(next) => r.push(next),
                None if r.is_empty() => return None,
                None => return Some(r),
            }
        }
        Some(r)
    }
}
