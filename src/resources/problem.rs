use crate::structure::Job;
pub static PROBLEM1_CAPACITY: u32 = 20;

pub fn problem1() -> Vec<Job> {
    vec![
        Job {
            release_date: 1,
            processing_time: 9,
            due_date: 29,
            size: 9,
            lateness_penalty: 2,
        },
        Job {
            release_date: 16,
            processing_time: 4,
            due_date: 31,
            size: 7,
            lateness_penalty: 6,
        },
        Job {
            release_date: 20,
            processing_time: 5,
            due_date: 42,
            size: 9,
            lateness_penalty: 2,
        },
        Job {
            release_date: 14,
            processing_time: 3,
            due_date: 22,
            size: 6,
            lateness_penalty: 10,
        },
        Job {
            release_date: 5,
            processing_time: 6,
            due_date: 22,
            size: 6,
            lateness_penalty: 3,
        },
        Job {
            release_date: 4,
            processing_time: 9,
            due_date: 27,
            size: 9,
            lateness_penalty: 6,
        },
        Job {
            release_date: 13,
            processing_time: 1,
            due_date: 17,
            size: 8,
            lateness_penalty: 9,
        },
        Job {
            release_date: 13,
            processing_time: 4,
            due_date: 22,
            size: 9,
            lateness_penalty: 3,
        },
        Job {
            release_date: 18,
            processing_time: 8,
            due_date: 28,
            size: 6,
            lateness_penalty: 3,
        },
        Job {
            release_date: 6,
            processing_time: 4,
            due_date: 38,
            size: 7,
            lateness_penalty: 5,
        },
    ]
}
