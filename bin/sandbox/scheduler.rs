use std::borrow::BorrowMut;
use std::collections::BTreeMap;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};

pub fn experiment() {
    let mut scheduler = Scheduler::default();
    fn fun1(scheduler: &mut Scheduler) {
        println!("fun1()");
        std::io::stdout().flush().unwrap();
    }
    fn fun2(scheduler: &mut Scheduler) {
        println!("fun2()");
        scheduler.schedule_after(Duration::from_millis(100), fun2);
    }
    scheduler.schedule_after(Duration::from_secs(3), |scheduler | {
        println!("After 3 seconds.");
        scheduler.schedule_after(Duration::from_secs(3), fun2);
        std::io::stdout().flush().unwrap();
    });
    scheduler.schedule_after(Duration::from_secs(10), fun1);
    scheduler.process_blocking();
}

enum SchedulerNode {
    Emtpy,
    Single(Box<dyn Fn(&mut Scheduler)>),
    Multi(Vec<Box<dyn Fn(&mut Scheduler)>>),
}

impl SchedulerNode {
    pub fn push(&mut self, callback: Box<dyn Fn(&mut Scheduler)>) {
        let old = std::mem::replace(self, Self::Emtpy);
        std::mem::replace(self, match old {
            SchedulerNode::Emtpy => SchedulerNode::Single(callback),
            SchedulerNode::Single(single) => SchedulerNode::Multi(vec![single, callback]),
            SchedulerNode::Multi(mut vec) => {
                vec.push(callback);
                SchedulerNode::Multi(vec)
            },
        });
    }

    pub fn invoke(&self, scheduler: &mut Scheduler) {
        match self {
            SchedulerNode::Emtpy => (),
            SchedulerNode::Single(single) => single(scheduler),
            SchedulerNode::Multi(vec) => {
                vec.iter().for_each(|callback| {
                    callback(scheduler);
                });
            },
        }
    }
}



#[derive(Default)]
pub struct Scheduler {
    schedule: BTreeMap<Instant, Option<SchedulerNode>>,
}

impl Scheduler {
    pub fn schedule<F: Fn(&mut Scheduler) + 'static>(&mut self, time: Instant, callback: F) {
        let Some(node) = self.schedule.entry(time).or_insert_with(|| Some(SchedulerNode::Emtpy)) else {
            unreachable!()
        };
        node.push(Box::new(callback));
    }

    pub fn schedule_after<F: Fn(&mut Scheduler) + 'static>(&mut self, duration: Duration, callback: F) {
        let Some(node) = self.schedule.entry(Instant::now() + duration).or_insert_with(|| Some(SchedulerNode::Emtpy)) else {
            unreachable!()
        };
        node.push(Box::new(callback));
    }

    pub fn process_until_now(&mut self) {
        while let Some((time, node)) = self.schedule.pop_first() {
            let now = Instant::now();
            if time > now {
                self.schedule.insert(time, node);
                return;
            }
            let Some(node) = node else {
                continue;
            };
            node.invoke(self);
        }
    }

    pub fn process_blocking(&mut self) {
        while let Some((time, node)) = self.schedule.pop_first() {
            let now = Instant::now();
            if time > now {
                self.schedule.insert(time, node);
                let time_until = time - now;
                const ONE_MS: Duration = Duration::from_millis(1);
                const TWO_MS: Duration = Duration::from_millis(2);
                if time_until >= TWO_MS {
                    let sleep_time = time_until - ONE_MS;
                    std::thread::sleep(sleep_time);
                }
                continue;
            }
            let Some(node) = node else {
                continue;
            };
            node.invoke(self);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.schedule.is_empty()
    }

    pub fn len(&self) -> usize {
        self.schedule.len()
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use crate::scheduler;

    use super::*;

    #[test]
    fn experiment() {
        
    }

    #[test]
    fn size_check() {
        println!("{}", std::mem::size_of::<SchedulerNode>());
        println!("{}", std::mem::size_of::<Option<SchedulerNode>>());
        

    }

    #[test]
    fn extract_arc_test() {
        let arc1 = Arc::new(Mutex::new(String::from("Hello.")));
        let arc2 = Arc::new(Mutex::new(String::from("Hello.")));
        let arc3 = Arc::new(Mutex::new(String::from("Hello.")));
        type ArcStr = Arc<Mutex<String>>;
        fn take_args(arg0: ArcStr, arg1: ArcStr, arg2: ArcStr) {
            #[inline(always)]
            fn first(text1: &mut String, (next, last): (&Arc<Mutex<String>>, &Arc<Mutex<String>>)) {
                #[inline(always)]
                fn second(text1: &mut String, text2: &mut String, last: &Arc<Mutex<String>>) {
                    #[inline(always)]
                    fn third(text1: &mut String, text2: &mut String, text3: &mut String) {
                        *text1 = String::from("text1");
                        *text2 = String::from("text2");
                        *text3 = String::from("text3");
                    };
                    let mut last_lock = last.lock().unwrap();
                    third(text1, text2, last_lock.borrow_mut());
                };
                let mut next_lock = next.lock().unwrap();
                second(text1, next_lock.borrow_mut(), last);
            };
            let mut first_lock = arg0.lock().unwrap();
            first(first_lock.borrow_mut(), (&arg1, &arg2));
        }
        take_args(arc1.clone(), arc2.clone(), arc3.clone());
        println!("{arc1:#?}, {arc2:#?}, {arc3:#?}");
    }
}