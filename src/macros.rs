macro_rules! add_bench {
    () => {
        bench!(async_channel);
        bench!(flume);
        bench!(futures_mpsc);
        bench!(tachyonix);
        bench!(thingbuf);
        bench!(postage_mpsc);
        bench!(tokio_mpsc);
    };
}

pub(crate) use add_bench;
