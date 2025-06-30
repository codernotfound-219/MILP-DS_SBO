use crate::structure::Job;
pub static PROBLEM1_CAPACITY: u32 = 20;
pub static PROBLEM2_CAPACITY: u32 = 20; // You can adjust this as needed
pub static PROBLEM3_CAPACITY: u32 = 20;

pub fn problem1() -> Vec<Job> {
    vec![
        Job {
            release_date: 1,
            processing_time: 9,
            due_date: 29,
            size: 9,
        },
        Job {
            release_date: 16,
            processing_time: 4,
            due_date: 31,
            size: 7,
        },
        Job {
            release_date: 20,
            processing_time: 5,
            due_date: 42,
            size: 9,
        },
        Job {
            release_date: 14,
            processing_time: 3,
            due_date: 22,
            size: 6,
        },
        Job {
            release_date: 5,
            processing_time: 6,
            due_date: 22,
            size: 6,
        },
        Job {
            release_date: 4,
            processing_time: 9,
            due_date: 27,
            size: 9,
        },
        Job {
            release_date: 13,
            processing_time: 1,
            due_date: 17,
            size: 8,
        },
        Job {
            release_date: 13,
            processing_time: 4,
            due_date: 22,
            size: 9,
        },
        Job {
            release_date: 18,
            processing_time: 8,
            due_date: 28,
            size: 6,
        },
        Job {
            release_date: 6,
            processing_time: 4,
            due_date: 38,
            size: 7,
        },
    ]
}

pub fn problem2() -> Vec<Job> {
    vec![
        Job {
            release_date: 5,
            processing_time: 8,
            due_date: 24,
            size: 8,
        },
        Job {
            release_date: 4,
            processing_time: 4,
            due_date: 21,
            size: 7,
        },
        Job {
            release_date: 19,
            processing_time: 2,
            due_date: 25,
            size: 8,
        },
        Job {
            release_date: 14,
            processing_time: 5,
            due_date: 28,
            size: 6,
        },
        Job {
            release_date: 4,
            processing_time: 8,
            due_date: 22,
            size: 8,
        },
        Job {
            release_date: 15,
            processing_time: 9,
            due_date: 34,
            size: 9,
        },
        Job {
            release_date: 15,
            processing_time: 5,
            due_date: 28,
            size: 9,
        },
        Job {
            release_date: 9,
            processing_time: 6,
            due_date: 21,
            size: 6,
        },
        Job {
            release_date: 17,
            processing_time: 4,
            due_date: 34,
            size: 9,
        },
        Job {
            release_date: 9,
            processing_time: 1,
            due_date: 19,
            size: 7,
        },
        Job {
            release_date: 11,
            processing_time: 2,
            due_date: 22,
            size: 5,
        },
        Job {
            release_date: 8,
            processing_time: 5,
            due_date: 24,
            size: 8,
        },
        Job {
            release_date: 20,
            processing_time: 6,
            due_date: 39,
            size: 6,
        },
        Job {
            release_date: 18,
            processing_time: 7,
            due_date: 38,
            size: 5,
        },
        Job {
            release_date: 24,
            processing_time: 4,
            due_date: 40,
            size: 6,
        },
    ]
}

pub fn problem3() -> Vec<Job> {
    vec![
        Job {
            release_date: 12,
            processing_time: 1,
            due_date: 42,
            size: 4,
        },
        Job {
            release_date: 16,
            processing_time: 10,
            due_date: 55,
            size: 9,
        },
        Job {
            release_date: 10,
            processing_time: 7,
            due_date: 38,
            size: 4,
        },
        Job {
            release_date: 19,
            processing_time: 6,
            due_date: 39,
            size: 4,
        },
        Job {
            release_date: 6,
            processing_time: 1,
            due_date: 22,
            size: 8,
        },
        Job {
            release_date: 3,
            processing_time: 2,
            due_date: 28,
            size: 9,
        },
        Job {
            release_date: 5,
            processing_time: 10,
            due_date: 34,
            size: 5,
        },
        Job {
            release_date: 12,
            processing_time: 7,
            due_date: 26,
            size: 10,
        },
        Job {
            release_date: 18,
            processing_time: 5,
            due_date: 24,
            size: 4,
        },
        Job {
            release_date: 17,
            processing_time: 2,
            due_date: 44,
            size: 4,
        },
        Job {
            release_date: 2,
            processing_time: 5,
            due_date: 13,
            size: 4,
        },
        Job {
            release_date: 6,
            processing_time: 9,
            due_date: 45,
            size: 7,
        },
        Job {
            release_date: 14,
            processing_time: 8,
            due_date: 29,
            size: 9,
        },
        Job {
            release_date: 12,
            processing_time: 4,
            due_date: 24,
            size: 5,
        },
        Job {
            release_date: 7,
            processing_time: 9,
            due_date: 26,
            size: 5,
        },
    ]
}
