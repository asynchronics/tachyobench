pub mod async_channel {
    use ::async_channel as channel;

    #[derive(Clone)]
    pub struct Sender<T> {
        inner: channel::Sender<T>,
    }
    impl<T> Sender<T> {
        pub async fn send(&mut self, message: T) {
            self.inner.send(message).await.unwrap();
        }
    }

    pub struct Receiver<T> {
        inner: channel::Receiver<T>,
    }
    impl<T> Receiver<T> {
        pub async fn recv(&mut self) -> Option<T> {
            self.inner.recv().await.ok()
        }
    }

    pub fn channel<T>(capacity: usize) -> (Sender<T>, Receiver<T>) {
        let (s, r) = channel::bounded(capacity);
        (Sender { inner: s }, Receiver { inner: r })
    }
}

pub mod flume {
    use ::flume as channel;

    #[derive(Clone)]
    pub struct Sender<T> {
        inner: channel::Sender<T>,
    }
    impl<T> Sender<T> {
        pub async fn send(&mut self, message: T) {
            self.inner.send_async(message).await.unwrap();
        }
    }

    pub struct Receiver<T> {
        inner: channel::Receiver<T>,
    }
    impl<T> Receiver<T> {
        pub async fn recv(&mut self) -> Option<T> {
            self.inner.recv_async().await.ok()
        }
    }

    pub fn channel<T>(capacity: usize) -> (Sender<T>, Receiver<T>) {
        let (s, r) = channel::bounded(capacity);
        (Sender { inner: s }, Receiver { inner: r })
    }
}

pub mod futures_mpsc {
    use ::futures_channel::mpsc as channel;
    use ::futures_util::sink::SinkExt;
    use ::futures_util::stream::StreamExt;

    use std::fmt::Debug;

    #[derive(Clone)]
    pub struct Sender<T> {
        inner: channel::Sender<T>,
    }
    impl<T: Debug> Sender<T> {
        pub async fn send(&mut self, message: T) {
            self.inner.send(message).await.unwrap();
        }
    }

    pub struct Receiver<T> {
        inner: channel::Receiver<T>,
    }
    impl<T> Receiver<T> {
        pub async fn recv(&mut self) -> Option<T> {
            self.inner.next().await
        }
    }

    pub fn channel<T>(capacity: usize) -> (Sender<T>, Receiver<T>) {
        let (s, r) = channel::channel(capacity);
        (Sender { inner: s }, Receiver { inner: r })
    }
}

pub mod postage_mpsc {
    use ::postage::mpsc as channel;
    use ::postage::sink::Sink;
    use ::postage::stream::Stream;

    use std::fmt::Debug;

    #[derive(Clone)]
    pub struct Sender<T> {
        inner: channel::Sender<T>,
    }
    impl<T: Debug> Sender<T> {
        pub async fn send(&mut self, message: T) {
            self.inner.send(message).await.unwrap();
        }
    }

    pub struct Receiver<T> {
        inner: channel::Receiver<T>,
    }
    impl<T> Receiver<T> {
        pub async fn recv(&mut self) -> Option<T> {
            self.inner.recv().await
        }
    }

    pub fn channel<T>(capacity: usize) -> (Sender<T>, Receiver<T>) {
        let (s, r) = channel::channel(capacity);
        (Sender { inner: s }, Receiver { inner: r })
    }
}

pub mod tachyonix {
    use ::tachyonix as channel;

    #[derive(Clone)]
    pub struct Sender<T> {
        inner: channel::Sender<T>,
    }
    impl<T: std::fmt::Debug> Sender<T> {
        pub async fn send(&mut self, message: T) {
            self.inner.send(message).await.unwrap();
        }
    }

    pub struct Receiver<T> {
        inner: channel::Receiver<T>,
    }
    impl<T> Receiver<T> {
        pub async fn recv(&mut self) -> Option<T> {
            self.inner.recv().await.ok()
        }
    }

    pub fn channel<T>(capacity: usize) -> (Sender<T>, Receiver<T>) {
        let (s, r) = channel::channel(capacity);
        (Sender { inner: s }, Receiver { inner: r })
    }
}

pub mod tokio_mpsc {
    use ::tokio::sync::mpsc as channel;

    use std::fmt::Debug;

    #[derive(Clone)]
    pub struct Sender<T> {
        inner: channel::Sender<T>,
    }
    impl<T: Debug> Sender<T> {
        pub async fn send(&mut self, message: T) {
            self.inner.send(message).await.unwrap();
        }
    }

    pub struct Receiver<T> {
        inner: channel::Receiver<T>,
    }
    impl<T> Receiver<T> {
        pub async fn recv(&mut self) -> Option<T> {
            self.inner.recv().await
        }
    }

    pub fn channel<T>(capacity: usize) -> (Sender<T>, Receiver<T>) {
        let (s, r) = channel::channel(capacity);
        (Sender { inner: s }, Receiver { inner: r })
    }
}
