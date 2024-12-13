#[derive(Debug, Clone, PartialEq)]
pub enum ThreadState {
    Idle,
    Processing,
    Blocked,
    Terminated,
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub enum ThreadPriority {
    Critical = 0,
    High = 1,
    Normal = 2,
    Low = 3,
    Background = 4,
}