


pub struct Batch<T> {

    pub items: Box<[T]>
}



struct BatchBuilder<T> {

    batches: Vec<Batch<T>>,
    incomplete_batch: Vec<T>,
    batch_size: usize
}

impl<T> BatchBuilder<T> {

    pub fn new(batch_size: usize) -> Self {

        assert!(batch_size > 0);

        return Self{batches: Vec::new(), incomplete_batch: Vec::with_capacity(batch_size), batch_size};
    }

    pub fn push(&mut self, new_item: T) {

        self.incomplete_batch.push(new_item);

        if self.incomplete_batch.len() == self.batch_size {

            self.flush_batch();
        }
    }

    pub fn flush_batch(&mut self) {

        let items = std::mem::replace(&mut self.incomplete_batch, Vec::with_capacity(self.batch_size));
        let batch = Batch{items: items.into_boxed_slice()};
        self.batches.push(batch);
    }

    pub fn get(mut self) -> Vec<Batch<T>> {

        if self.incomplete_batch.is_empty() == false {

            self.flush_batch();
        }

        assert!(self.incomplete_batch.is_empty());
        return self.batches;
    }
}

#[cfg(test)]
mod test_batch_builder {

    use super::*;


    const BATCH_SIZE: usize = 10;


    struct Item{}


    fn build_accumulator_with_size(size: usize) -> BatchBuilder<Item> {

        let mut acc = BatchBuilder::new(BATCH_SIZE);

        for _ in 0..size {

            acc.push(Item{});
        }

        return acc;
    }

    fn upper_rounded_division(n1: usize, n2: usize) -> usize {

        let down_rounded_division = n1 / n2;
        let upper_rounded_division = if n1 % n2 > 0 { down_rounded_division + 1 } else { down_rounded_division };

        return upper_rounded_division;
    }

    #[test]
    fn test_batch_builder_output_size() {

        #[track_caller]
        fn test_with_item_count(item_count: usize) {

            assert_eq!(build_accumulator_with_size(item_count).get().len(), upper_rounded_division(item_count, BATCH_SIZE));
        }

        test_with_item_count(0);
        test_with_item_count(1);
        test_with_item_count(BATCH_SIZE * 1);
        test_with_item_count((BATCH_SIZE * 1) + 1);
        test_with_item_count((BATCH_SIZE * 1) - 1);
    }
}
