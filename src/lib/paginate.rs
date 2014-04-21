pub trait PaginatableIterator<T> {
    fn paginate(self, page: uint) -> Paginate<Self>;
}

impl<T, U : Iterator<T>> PaginatableIterator<T> for U {
    fn paginate(self, page: uint) -> Paginate<U> {
        Paginate {
            iter: self,
            page: page,
        }
    }
}

struct Paginate<I> {
    iter: I,
    page: uint,
}

impl<E, I : Iterator<E>> Iterator<Vec<E>> for Paginate<I> {
    fn next(&mut self) -> Option<Vec<E>> {
        let mut r = Vec::new();
        for _ in range(0, self.page) {
            match self.iter.next() {
                Some(next) => r.push(next),
                None => return Some(r).filtered(|v| !v.is_empty()),
            }
        }
        Some(r)
    }
}

