use std::future::Future;

pub trait Executor: Default {
    fn spawn<T: Future<Output = ()> + Send + 'static>(&mut self, future: T);
    fn join_all(&mut self);
}

pub struct TokioExecutor {
    join_handles: Vec<::tokio::task::JoinHandle<()>>,
    runtime: ::tokio::runtime::Runtime,
}

impl Executor for TokioExecutor {
    fn spawn<T: Future<Output = ()> + Send + 'static>(&mut self, future: T) {
        self.join_handles.push(self.runtime.spawn(future));
    }
    fn join_all(&mut self) {
        let join_handles = std::mem::take(&mut self.join_handles);
        self.runtime.block_on(async move {
            for fut in join_handles {
                fut.await.unwrap();
            }
        });
    }
}

impl Default for TokioExecutor {
    fn default() -> Self {
        let runtime = ::tokio::runtime::Builder::new_multi_thread()
            .build()
            .unwrap();

        Self {
            join_handles: Vec::new(),
            runtime,
        }
    }
}

#[cfg(feature = "smol")]
static SMOL_EXECUTOR: ::smol::Executor<'static> = ::smol::Executor::new();

#[cfg(feature = "smol")]
#[derive(Default)]
pub struct SmolExecutor {
    join_handles: Vec<::smol::Task<()>>,
}
#[cfg(feature = "smol")]
impl Executor for SmolExecutor {
    fn spawn<T: Future<Output = ()> + Send + 'static>(&mut self, future: T) {
        self.join_handles.push(SMOL_EXECUTOR.spawn(future));
    }
    fn join_all(&mut self) {
        let join_handles = std::mem::take(&mut self.join_handles);
        std::thread::spawn(move || {
            smol::future::block_on(SMOL_EXECUTOR.run(smol::future::pending::<()>()))
        });
        ::smol::future::block_on(async move {
            for fut in join_handles {
                fut.await;
            }
        });
    }
}

#[cfg(feature = "smolscale")]
#[derive(Default)]
pub struct SmolScaleExecutor {
    join_handles: Vec<::async_task::Task<()>>,
}
#[cfg(feature = "smolscale")]
impl Executor for SmolScaleExecutor {
    fn spawn<T: Future<Output = ()> + Send + 'static>(&mut self, future: T) {
        self.join_handles.push(::smolscale::spawn(future));
    }
    fn join_all(&mut self) {
        let join_handles = std::mem::take(&mut self.join_handles);
        ::smolscale::block_on(async move {
            for fut in join_handles {
                fut.await;
            }
        });
    }
}

pub struct NexosimExecutor {
    executor: ::nexosim::dev_hooks::Executor,
}
impl Executor for NexosimExecutor {
    fn spawn<T: Future<Output = ()> + Send + 'static>(&mut self, future: T) {
        self.executor.spawn_and_forget(future);
    }
    fn join_all(&mut self) {
        self.executor.run();
    }
}

impl Default for NexosimExecutor {
    fn default() -> Self {
        Self {
            executor: ::nexosim::dev_hooks::Executor::new(::num_cpus::get()),
        }
    }
}
