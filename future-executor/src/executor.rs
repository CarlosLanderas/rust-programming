use {
    futures::{
        future::{FutureExt, BoxFuture},
        task::{ArcWake, waker_ref},
    },
    std:: {
        future::Future,
        sync::{Arc,Mutex},
        task::{Context, Poll},
        sync::mpsc::{sync_channel, SyncSender, Receiver}
    }
};

struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("too many tasks queued");
    }
}

pub struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

impl Executor {
    pub fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);

                if let Poll::Pending = future.as_mut().poll(context) {
                    *future_slot = Some(future);
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });

        self.task_sender.send(task).expect("too many tasks queued");
    }
}

pub fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS : usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue}, Spawner { task_sender})
}