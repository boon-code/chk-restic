use std::thread::{self, JoinHandle};
use crossbeam_channel::{bounded, Receiver, Sender};
use super::action::Action;
use std::path::{Path, PathBuf};

pub struct ParallelWalker {
    walker: Vec<Walker>,
    tx: Sender<PathBuf>,
    rx: Receiver<Action>,
}
impl ParallelWalker {
    pub fn new(num_of_threads: usize, qlen: usize) -> Self {
        let (in_tx, in_rx) = bounded(qlen);    // Path
        let (out_tx, out_rx) = bounded(qlen);  // Action
        let walker: Vec<Walker> = (0..num_of_threads)
            .into_iter()
            .map(|i| Walker::start(out_tx.clone(), in_rx.clone(), i))
            .collect();
        
        Self {
            walker,
            tx: in_tx,
            rx: out_rx,
        }
    }

    pub fn add(&self, path: &Path) {
        self.tx.send(path.to_path_buf());
    }

    pub fn recv(&self) -> Option<Action> {
        let m = self.rx.recv();
        if let a = Some(m.ok()) {
            Some(Action::new_checked())
        } else {
            None
        }
    }

    pub fn stop(self) {
        for mut w in self.walker {
            w.stop();
        }
        println!("Stopped");
    }
}

struct Walker {
    thread: Option<JoinHandle<()>>,
}
impl Walker {
    pub fn start(tx: Sender<Action>, rx: Receiver<PathBuf>, i: usize) -> Self {
        let t = thread::spawn(move || {
            while let Ok(p) = rx.recv() {
                println!("{}", p.display());
                tx.send(Action::new_checked());
                println!("recv");
            }
            println!("Walker-{} stopped", i);
        });
        println!("Walker-{} started", i);
        Walker {
            thread: Some(t)
        }
    }

    pub fn stop(&mut self) {
        let opt: Option<JoinHandle<()>> = self.thread.take();
        if let Some(t) = self.thread.take() {
            t.join();
        }
    }
}

#[cfg(test)]
mod tests;