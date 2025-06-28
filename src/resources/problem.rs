use crate::structure::Job;
pub static PROBLEM1_CAPACITY: f32 = 20.0;

pub fn problem1() -> Vec<Job> {
    vec![
        Job {
            release_date: 1.0,
            processing_time: 9.0,
            due_date: 29.0,
            size: 9.0,
            lateness_penalty: 2.0,
        },
        Job {
            release_date: 16.0,
            processing_time: 4.0,
            due_date: 31.0,
            size: 7.0,
            lateness_penalty: 6.0,
        },
        Job {
            release_date: 20.0,
            processing_time: 5.0,
            due_date: 42.0,
            size: 9.0,
            lateness_penalty: 2.0,
        },
        Job {
            release_date: 14.0,
            processing_time: 3.0,
            due_date: 22.0,
            size: 6.0,
            lateness_penalty: 10.0,
        },
        Job {
            release_date: 5.0,
            processing_time: 6.0,
            due_date: 22.0,
            size: 6.0,
            lateness_penalty: 3.0,
        },
        Job {
            release_date: 4.0,
            processing_time: 9.0,
            due_date: 27.0,
            size: 9.0,
            lateness_penalty: 6.0,
        },
        Job {
            release_date: 13.0,
            processing_time: 1.0,
            due_date: 17.0,
            size: 8.0,
            lateness_penalty: 9.0,
        },
        Job {
            release_date: 13.0,
            processing_time: 4.0,
            due_date: 22.0,
            size: 9.0,
            lateness_penalty: 3.0,
        },
        Job {
            release_date: 18.0,
            processing_time: 8.0,
            due_date: 28.0,
            size: 6.0,
            lateness_penalty: 3.0,
        },
        Job {
            release_date: 6.0,
            processing_time: 4.0,
            due_date: 38.0,
            size: 7.0,
            lateness_penalty: 5.0,
        },
    ]
}
