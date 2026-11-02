//! A tiny persistent worker pool: `run(n, f)` calls `f(0..n)` spread over the
//! workers and the calling thread, and returns only when every call finished.
//! Persistent threads avoid spawning (clone/mmap/mprotect/...) per frame.

use std::sync::{Arc, Condvar, Mutex};
use std::thread::JoinHandle;

type Job = *const (dyn Fn(usize) + Sync);

#[derive(Clone, Copy)]
struct JobPtr(Job);
// Safety: the pointee is Sync, and `run` keeps it alive until all calls return.
unsafe impl Send for JobPtr {}

struct State {
    gen: u64,
    job: Option<JobPtr>,
    n: usize,
    next: usize,
    done: usize,
    quit: bool,
}

struct Shared {
    m: Mutex<State>,
    job_cv: Condvar,
    done_cv: Condvar,
}

pub struct Pool {
    shared: Arc<Shared>,
    workers: Vec<JoinHandle<()>>,
}

impl Pool {
    /// `threads` total, including the caller (so `threads - 1` workers).
    pub fn new(threads: usize) -> Pool {
        let shared = Arc::new(Shared {
            m: Mutex::new(State { gen: 0, job: None, n: 0, next: 0, done: 0, quit: false }),
            job_cv: Condvar::new(),
            done_cv: Condvar::new(),
        });
        let workers = (1..threads.max(1)).map(|_| {
            let s = shared.clone();
            std::thread::spawn(move || worker(&s))
        }).collect();
        Pool { shared, workers }
    }

    pub fn threads(&self) -> usize {
        self.workers.len() + 1
    }

    // Exclusive ownership prevents another caller replacing the borrowed job
    // while workers are still using it.
    pub fn run(&mut self, n: usize, f: &(dyn Fn(usize) + Sync)) {
        if self.workers.is_empty() || n <= 1 {
            (0..n).for_each(f);
            return;
        }
        // Erase the borrow's lifetime; we wait below until no worker can touch it.
        let job: Job = unsafe { std::mem::transmute::<&(dyn Fn(usize) + Sync), Job>(f) };
        let s = &*self.shared;
        {
            let mut st = s.m.lock().unwrap();
            st.job = Some(JobPtr(job));
            st.n = n;
            st.next = 0;
            st.done = 0;
            st.gen += 1;
        }
        s.job_cv.notify_all();
        loop {
            let i = {
                let mut st = s.m.lock().unwrap();
                if st.next >= st.n {
                    break;
                }
                st.next += 1;
                st.next - 1
            };
            f(i);
            let mut st = s.m.lock().unwrap();
            st.done += 1;
        }
        let mut st = s.m.lock().unwrap();
        while st.done < st.n {
            st = s.done_cv.wait(st).unwrap();
        }
        st.job = None;
    }
}

fn worker(s: &Shared) {
    let mut seen = 0u64;
    loop {
        let mut st = s.m.lock().unwrap();
        loop {
            if st.quit {
                return;
            }
            if st.gen != seen {
                if st.next < st.n {
                    break;
                }
                seen = st.gen; // this generation's work is all taken
            }
            st = s.job_cv.wait(st).unwrap();
        }
        let i = st.next;
        st.next += 1;
        let job = st.job.expect("job set while work remains");
        drop(st);
        unsafe { (*job.0)(i) };
        let mut st = s.m.lock().unwrap();
        st.done += 1;
        if st.done == st.n {
            s.done_cv.notify_all();
        }
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        self.shared.m.lock().unwrap().quit = true;
        self.shared.job_cv.notify_all();
        for w in self.workers.drain(..) {
            let _ = w.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn runs_every_index_exactly_once_many_times() {
        let mut pool = Pool::new(4);
        for round in 0..2000 {
            let n = round % 9;
            let hits: Vec<AtomicUsize> = (0..n).map(|_| AtomicUsize::new(0)).collect();
            pool.run(n, &|i| { hits[i].fetch_add(1, Ordering::Relaxed); });
            assert!(hits.iter().all(|h| h.load(Ordering::Relaxed) == 1), "round {round}");
        }
    }
}
