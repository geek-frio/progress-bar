pub enum IterType {
    Bounded(usize),
    Unbounded,
}
pub trait ProgressDisplay<T>
where
    T: Iterator,
{
    fn with_progress(self) -> Progress<T>;
}

pub trait Bounded<T>
where
    T: Iterator,
{
    fn bounded(self) -> Self;
}

pub struct Progress<T>
where
    T: Iterator,
{
    iter: T,
    comp_size: usize,
    iter_type: Option<IterType>,
}

impl<T: Iterator> Progress<T> {
    fn compute(iter_num: usize, total: usize) -> (usize, usize) {
        if total > 50 {
            (iter_num / (total / 50), 50)
        } else {
            (iter_num, total)
        }
    }
}

impl<T> Bounded<T> for Progress<T>
where
    T: ExactSizeIterator,
{
    fn bounded(mut self) -> Self {
        self.iter_type = Some(IterType::Bounded(self.iter.len()));
        self
    }
}

impl<T> ProgressDisplay<T> for T
where
    T: Iterator,
{
    fn with_progress(self) -> Progress<T> {
        Progress {
            iter: self,
            comp_size: 0,
            iter_type: Some(IterType::Unbounded),
        }
    }
}

impl<T> Iterator for Progress<T>
where
    T: Iterator,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.comp_size += 1;
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        let item = self.iter.next();
        if item.is_some() {
            if let Some(iter_type) = self.iter_type.as_ref() {
                match iter_type {
                    IterType::Unbounded => {
                        println!("Iter num:{}", self.comp_size);
                    }
                    IterType::Bounded(size) => {
                        let (progress, bar_size) = Self::compute(self.comp_size, *size);
                        println!("progess is:{}, bar_size is:{}", progress, bar_size);
                        println!(
                            "Iter num:{}; total_iter:{};\n progress:[{:bar_size$}]",
                            self.comp_size,
                            *size,
                            "▮".repeat(progress),
                        );
                    }
                }
            }
        }
        item
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn expensive_process<T>(_el: T) {
        std::thread::sleep(Duration::from_millis(200));
    }

    #[test]
    fn test() {
        let mut v = vec![];
        for i in 0..50 {
            v.push(i);
        }
        for i in v.iter().with_progress().bounded() {
            expensive_process(i);
        }
    }
}
