macro_rules! add_bench {
    () => {
        bench!(async_channel);
        bench!(flume);
        bench!(futures_mpsc);
        bench!(kanal);
        bench!(postage_mpsc);
        bench!(tachyonix);
        bench!(thingbuf);
        bench!(tokio_mpsc);
    };
}

pub(crate) use add_bench;
