
// std
use std::sync::Arc;

// local
use crate::ignore_node::IgnoreNode;



pub struct Job {

    path: std::path::PathBuf,
    parent_ignore_node: Option<Arc<IgnoreNode>>
}

pub struct JobBatch {

    jobs: Box<[Job]>
}


pub struct JobAccumulator {

    job_batches: Vec<JobBatch>,
    incomplete_batch: Vec<Job>,
    batch_size: usize
}

impl JobAccumulator {

    pub fn new(batch_size: usize) -> Self {

        assert!(batch_size > 0);

        return Self{job_batches: Vec::new(), incomplete_batch: Vec::with_capacity(batch_size), batch_size};
    }

    pub fn push(&mut self, new_job: Job) {

        self.incomplete_batch.push(new_job);

        if self.incomplete_batch.len() == self.batch_size {

            self.flush_job_batch();
        }
    }

    pub fn flush_job_batch(&mut self) {

        let jobs = std::mem::replace(&mut self.incomplete_batch, Vec::with_capacity(self.batch_size));
        let batch = JobBatch{jobs: jobs.into_boxed_slice()};
        self.job_batches.push(batch);
    }

    pub fn get(mut self) -> Vec<JobBatch> {

        if self.incomplete_batch.is_empty() == false {

            self.flush_job_batch();
        }

        debug_assert!(self.incomplete_batch.is_empty());
        return self.job_batches;
    }
}


#[cfg(test)]
mod test_job_accumulator {

    use super::*;


    const BATCH_SIZE: usize = 10;


    fn build_empty_job() -> Job {

        return Job{path: std::path::PathBuf::from(""), parent_ignore_node: None};
    }

    fn build_accumulator_with_size(size: usize) -> JobAccumulator {

        let mut acc = JobAccumulator::new(BATCH_SIZE);

        for _ in 0..size {

            acc.push(build_empty_job());
        }

        return acc;
    }

    fn upper_rounded_division(n1: usize, n2: usize) -> usize {

        let down_rounded_division = n1 / n2;
        let upper_rounded_division = if n1 % n2 > 0 { down_rounded_division + 1 } else { down_rounded_division };

        return upper_rounded_division;
    }

    #[test]
    fn test_job_accumulator_output_size() {

        fn test_with_job_count(job_count: usize) {

            assert_eq!(build_accumulator_with_size(job_count).get().len(), upper_rounded_division(job_count, BATCH_SIZE));
        }

        test_with_job_count(0);
        test_with_job_count(1);
        test_with_job_count(BATCH_SIZE * 1);
        test_with_job_count((BATCH_SIZE * 1) + 1);
        test_with_job_count((BATCH_SIZE * 1) - 1);
    }
}
