#[derive(Debug)]
pub struct Job {
    pub release_date: f64,
    pub processing_time: f64,
    pub due_date: f64,
    pub size: f64,
    pub lateness_penalty: f64,
}
