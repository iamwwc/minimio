// 已经放到schedule 队列，正在等待schduler调度
pub (crate) const SCHEDULED: usize = 1 << 0;

// Task 正在执行， Task内部的futureu正在被polling
pub (crate) const RUNNING: usize = 1 << 1;

// Task已经完成，在 Poll::Ready时设置，结果Output存放在Task中，待将Output取出后Task被标记为结束
pub (crate) const COMPLETED: usize = 1 << 3;

//
pub (crate) const TASK: usize = 1 << 4;

//有其他Task注册了awaiter，正在等待当前Task完成
pub (crate) const AWAITER: usize = 1 << 5;

// 
pub (crate) const REGISTERING: usize = 1 << 6;

// awaiter已经被通知
pub (crate) const NOTIFYING: usize = 1 << 7;

//单引用
pub (crate) const REFERENCE: usize = 1 << 8;