# The tachyonix benchmark

This is a small benchmarking suite for `async` MPSC message passing, used to
monitor regressions in [Asynchronix][asynchronix] and [Tachyonix][tachyonix].


## Channels and runtimes

At the moment, the following MPMC/MPSC channels are available:
- [tachyonix]
- [async-channel]
- [flume]
- [futures-channel]
- [postage::mpsc]
- [tokio::mpsc]

It is possible to select one of the following runtimes:
- [asynchronix]
- [tokio]
- [async-std]
- [smolscale]

[tachyonix]: https://github.com/asynchronics/tachyonix
[async-channel]: https://github.com/smol-rs/async-channel
[flume]: https://github.com/zesterer/flume
[futures-channel]: https://github.com/rust-lang/futures-rs
[postage::mpsc]: https://github.com/austinjones/postage-rs
[tokio::mpsc]: https://github.com/tokio-rs/tokio
[asynchronix]: https://github.com/asynchronics/asynchronix
[tokio]: https://github.com/tokio-rs/tokio
[async-std]: https://github.com/async-rs/async-std
[smolscale]: https://github.com/geph-official/smolscale


## Benchmarks

There are currently 2 parametric benchmarks:
- *pinball*: fully connected graph where messages ("balls") perform a random
  walk between nodes ("pins"),
- *funnel*: many-to-one messaging in a tight loop.

Benchmarks always run on all available logical threads.


### Disclaimer

Benchmarking multithreaded `async` code is tricky: a lot depends on the detailed
implementation and scheduling strategy of the executor, and performance
variability is typically much higher than with synchronous code, in particular
when the load is not sufficient to keep all worker threads busy.

*Criterion*, the de-facto standard Rust benchmarking framework, cannot be used
as it lacks support for task spawning and multithreading. This bench uses
straightforward execution time measurement and statistical analysis, running
long-lasting tests to reduce noise and minimize the impact of executor startup.
Async executors are complex enough to prevent obvious benchmark-defeating
compiler optimizations, so no black-boxing is implemented.


### Pinball

This benchmark is an upgraded version of the classical ping-pong benchmark. Its
main goal is to measure performance in situations where receivers are often
starved but senders are never blocked.

Each test rig consists of a complete graph (a.k.a. fully connected graph) which
edges are the channels. Each node forwards any message it receives to another
randomly chosen node. For this benchmark, each graph contains 13 nodes, each
node containing in turn 1 receiver and 12 senders (1 for each other node).
Importantly, each channel has enough capacity to never block on sending. The
benchmark concurrently runs 61 such rigs of 13 nodes.

The test is performed for various numbers of messages ("balls"), which are
initially fairly distributed across the graph. The messages then perform a
random walk between the nodes ("pins") until they have visited a pre-defined
amount of nodes.


### Funnel

This benchmark is ubiquitous and often simply referred to as the "MPSC
benchmark". It consists of a single receiver connected to many senders which
receive and send messages in a tight loop.

What this benchmark measures is unfortunately not only related to the absolute
speed of enqueue, dequeue and notify operations: it also depends on the relative
speed of these operations. Unsurprisingly, the standard deviation on the results
is large compared to the pinball benchmark. Corollary: despite its popularity,
this benchmark is neither very realistic nor very objective.

In this particular implementation, each receiver is connected to 13 senders. The
benchmark runs 61 such rigs of 13 senders and 1 receiver concurrently.

The test is performed for various channel capacities. Note that unlike the other
channels, tokio's MPSC channel reserves 1 additional slot for each of the 13
senders on top of the nominal capacity.


## Example usage

For help, type:

```
$ bench -h
```

To see all benchmarks for all channels, type:

```
$ bench -l
```

To run the *pinball* benchmark for all channels using Tokio, type:

```
$ bench pinball
```

To run all benchmarks for `tachonix` using Tokio, type:

```
$ bench tachyonix
```

To run only the *funnel* benchmark for `flume` using Tokio and average the
results over 5 runs, type:

```
$ bench -s 5 funnel-flume
```

To run all benchmarks for `async-channel` with Asynchronix instead of Tokio, type:

```
$ bench async_channel -e asynchronix
```

## License

The code in this repository is licensed under the [Apache License, Version
2.0](LICENSE-APACHE) or the [MIT license](LICENSE-MIT), at your option. Note,
however, that the Tokio and Smolscale dependencies are respectively available
under the MIT license and the ISC license only. Although the ISC license is
broadly considered compatible with the MIT license, it is your responsibility to
ensure that any distribution by you of this bench in an executable form indeed
conforms to the licenses of these dependencies.
