macro_rules! add_bench {
    () => {
        bench!(async_channel);
        bench!(flume);
        bench!(tachyonix);
        bench!(postage_mpsc);
        bench!(tokio_mpsc);
        bench!(kanal);
    };
}

pub(crate) use add_bench;
