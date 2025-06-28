#[derive(Debug)]
pub struct Job {
    pub release_date: u32,
    pub processing_time: u32,
    pub due_date: u32,
    pub size: u32,
    pub lateness_penalty: u32,
}
