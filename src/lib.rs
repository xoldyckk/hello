use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self},
};

pub struct ThreadPool {
    // sends closure jobs for execution to receivers inside spawned threads,
    // wrapped in Option type to make it easily destroyable by swapping the
    // Some variant with None variant, we want it to be destroyable because
    // sender being destroyed signals the receivers that no more messages
    // are to be received, therefore signalling them to halt their process
    // of listening for messages sent by the sender
    // .recv() method on receivers blocks the thread execution and waits for
    // messages to be sent by the sender, when sender goes out of scope(destroyed)
    // .recv() returns an Err variant, which gives us the programmer a lean way
    // for gracefully shutting down whatever task we were doing with the receiver
    sender: Option<mpsc::Sender<Job>>,
    threads: Vec<Option<(usize, thread::JoinHandle<()>)>>,
}

impl ThreadPool {
    pub fn execute<F>(&self, f: F)
    where
        // any type F which implementation these traits can be passed in as the argument to this method
        F: FnOnce() + Send + 'static,
    {
        // Box is needed to hold the trait object because, it has no
        // definite known size at compile time, therefore rust compiler
        // will fail to compile it unless it is stored on the heap using
        // Box smart pointer
        let job = Box::new(f);
        // as_ref() just gives back an immutable reference to sender here
        self.sender.as_ref().unwrap().send(job).unwrap();
    }

    pub fn new(size: usize) -> ThreadPool {
        // makes sure that there is at least 1 thread in the thread pool,
        // panics if 0 is provided as the value for number of threads
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel::<Job>();
        // since receiver itself cannot
        // be cloned unlike sender.clone(), following the principle
        // multiple producer single consumer(mpsc), we can have multiple
        // instaces of sender by cloning it directly, but only a single
        // instace of receiver, Arc lets us have multiple instances of
        // receiver across threads in a thread-safe way, using Mutex to
        // make sure at any time only a single thread can access the
        // received messages queue stored in receiver
        let receiver = Arc::new(Mutex::new(receiver));
        let mut threads = Vec::with_capacity(size);

        for id in 1..=size {
            let receiver = Arc::clone(&receiver);
            // here loop keyword is used to create a implicit loop closure
            // that runs as long as it is not terminated by calling the
            // break statement inside it, the looping is done basically to
            // keep checking the receiver queue for new messages sent by
            // the thread pool sender, loops internal to the closure scope
            // are not used because they would make this thread own the
            // and not release the receiver lock till the closure is terminated,
            // basically making our multi-threaded implementation single-threaded,
            // this has something to do with `temporary` value in rust which is
            // dropped as soon as it is used, for example using values returned
            // by a function in an expression
            let thread = thread::spawn(move || loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Thread {id} got a job; executing.");
                        job();
                    }
                    Err(_) => {
                        break;
                    }
                }
            });

            threads.push(Some((id, thread)));
        }

        // explicit drop of receiver not required here because it is
        // dropped anyway afer this function's scope ends, but always
        // remember to never have a valid instance of receiver, because
        // as long as it exists in the memory, sender will assume that
        // receiver is still accepting messages which might not be the
        // desired behaviour in many cases

        // drop(receiver);

        ThreadPool {
            sender: Some(sender),
            threads,
        }
    }
}

impl Drop for ThreadPool {
    // called whenever the associated thread pool object goes out of scope,
    // here we need a custom implementation for it because we don't want
    // to end the program abruptly, if there are messages remaining in the
    // queue sent from the thread pool sender or if the threads are currently
    // processing a request, they need to be handled before the server is
    // shut down, so basically we're trying to gracefully shut down the
    // server instead of shutting it down abruptly
    fn drop(&mut self) {
        // signals the receivers passed to threads in thread pool,
        // that it has been dropped and for them to stop listening
        // for new messages, so calling .recv() method on receivers
        // results in an Err variant being returned, Err variant is
        // a programmatic signal to the programmer to halt the execution
        // of the thread closure
        drop(self.sender.take());

        for thread in &mut self.threads {
            // for each Some variant that holds a thread in thread pool
            // we call thread.join().unwrap() for main() thread to wait
            // for the spawned thread to finish it's processing successfully,
            // ignores the None variant
            if let Some((thread_id, thread)) = thread.take() {
                if !thread.is_finished() {
                    // this is synchronous and halts the thread it is
                    // called in(main thread) here, until the thread it references
                    // comes to a halt by completing its closure logic execution
                    thread.join().unwrap();
                }
                println!("Thread {} disconnected; shutting down.", thread_id);
            }
        }
    }
}

// type alias for a Job trait object stored on the heap using Box smart pointer
type Job = Box<dyn FnOnce() + Send + 'static>;
