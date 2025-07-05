use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs::File;
use std::io::Write;
use std::num::NonZeroU32;

use lexopt::prelude::*;

mod benches;
mod channel_shims;
mod executor_shims;
mod macros;

const HELP_MESSAGE: &str = "\
tachyobench
Benchmark runner for Tachyonix

USAGE:
    tachyobench [OPTIONS] <BENCHNAME>

ARGS:
    <BENCHNAME>    If specified, only run benches containing this string in their names

OPTIONS:
    -h, --help             Print help information
    -l, --list             List available benches
    -s, --samples SAMPLES  Repeat benches SAMPLES times and average the result
    -o, --output FILE      Save the results to FILE
    -e, --exec EXECUTOR    Run the bench with the EXECUTOR runtime;
                           possible values:
                               tokio [default],
                               nexosim,
                               smol [requires feature 'smol'],
                               smolscale [requires feature 'smolscale']";

macro_rules! add_test {
    ($group:ident, $channel:ident) => {
        (
            stringify!($group),
            stringify!($channel),
            &[
                (
                    ExecutorId::Tokio,
                    benches::$group::$channel::bench::<crate::executor_shims::TokioExecutor>,
                ),
                #[cfg(feature = "smol")]
                (
                    ExecutorId::Smol,
                    benches::$group::$channel::bench::<crate::executor_shims::SmolExecutor>,
                ),
                #[cfg(feature = "smolscale")]
                (
                    ExecutorId::SmolScale,
                    benches::$group::$channel::bench::<crate::executor_shims::SmolScaleExecutor>,
                ),
                (
                    ExecutorId::Nexosim,
                    benches::$group::$channel::bench::<crate::executor_shims::NexosimExecutor>,
                ),
            ],
        )
    };
}

#[allow(clippy::type_complexity)]
const BENCHES: &[(&str, &str, &[(ExecutorId, fn(NonZeroU32) -> BenchIterator)])] = &[
    add_test!(funnel, async_channel),
    add_test!(funnel, flume),
    add_test!(funnel, futures_mpsc),
    add_test!(funnel, tachyonix),
    add_test!(funnel, thingbuf),
    add_test!(funnel, postage_mpsc),
    add_test!(funnel, tokio_mpsc),
    add_test!(pinball, async_channel),
    add_test!(pinball, flume),
    add_test!(pinball, futures_mpsc),
    add_test!(pinball, tachyonix),
    add_test!(pinball, thingbuf),
    add_test!(pinball, postage_mpsc),
    add_test!(pinball, tokio_mpsc),
];

pub struct BenchResult {
    label: String,
    parameter: String,
    throughput: Vec<f64>,
}
impl BenchResult {
    pub fn new(label: String, parameter: String, throughput: Vec<f64>) -> Self {
        Self {
            label,
            parameter,
            throughput,
        }
    }
}

type BenchIterator = Box<dyn Iterator<Item = BenchResult>>;

#[derive(PartialEq)]
enum ExecutorId {
    Tokio,
    Nexosim,
    #[cfg(feature = "smol")]
    Smol,
    #[cfg(feature = "smolscale")]
    SmolScale,
}
impl ExecutorId {
    const TOKIO: &'static str = "tokio";
    const NEXOSIM: &'static str = "nexosim";
    #[cfg(feature = "smol")]
    const SMOL: &'static str = "smol";
    #[cfg(feature = "smolscale")]
    const SMOLSCALE: &'static str = "smolscale";

    fn new(name: &str) -> Result<Self, ()> {
        match name {
            Self::TOKIO => Ok(ExecutorId::Tokio),
            Self::NEXOSIM => Ok(ExecutorId::Nexosim),
            #[cfg(feature = "smol")]
            Self::SMOL => Ok(ExecutorId::Smol),
            #[cfg(feature = "smolscale")]
            Self::SMOLSCALE => Ok(ExecutorId::SmolScale),
            _ => Err(()),
        }
    }
    fn name(&self) -> &'static str {
        match self {
            ExecutorId::Tokio => Self::TOKIO,
            ExecutorId::Nexosim => Self::NEXOSIM,
            #[cfg(feature = "smol")]
            ExecutorId::Smol => Self::SMOL,
            #[cfg(feature = "smolscale")]
            ExecutorId::SmolScale => Self::SMOLSCALE,
        }
    }
}

struct BenchArgs {
    bench_substrings: Vec<String>,
    executor: ExecutorId,
    samples: NonZeroU32,
    output: Option<OsString>,
}

fn parse_args() -> Result<Option<BenchArgs>, lexopt::Error> {
    let mut samples = NonZeroU32::new(1).unwrap();
    let mut executor = ExecutorId::Tokio;
    let mut bench_substrings = Vec::new();
    let mut output = None;

    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Short('h') | Long("help") => {
                println!("{HELP_MESSAGE}");

                return Ok(None);
            }
            Short('l') | Long("list") => {
                for (group, item, _) in BENCHES {
                    println!("    {group}-{item}")
                }

                return Ok(None);
            }
            Short('s') | Long("samples") => {
                samples = parser.value()?.parse()?;
            }
            Short('o') | Long("output") => {
                output = Some(parser.value()?);
            }
            Short('e') | Long("exec") => {
                let val = parser.value()?;
                executor = ExecutorId::new(val.clone().into_string()?.as_ref()).map_err(|_| {
                    lexopt::Error::UnexpectedValue {
                        option: "exec".into(),
                        value: val,
                    }
                })?;
            }
            Value(val) => {
                bench_substrings.push(val.into_string()?);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    Ok(Some(BenchArgs {
        bench_substrings,
        executor,
        samples,
        output,
    }))
}

fn main() -> Result<(), lexopt::Error> {
    #[allow(clippy::type_complexity)]
    let mut benches: BTreeMap<
        &'static str,
        BTreeMap<&'static str, fn(NonZeroU32) -> Box<dyn Iterator<Item = BenchResult>>>,
    > = BTreeMap::new();

    let BenchArgs {
        bench_substrings,
        executor,
        samples,
        output,
    } = match parse_args()? {
        None => return Ok(()),
        Some(args) => args,
    };

    // Select all requested benches.
    for (group, item, executor_benches) in BENCHES {
        let bench_name = format!("{group}-{item}");
        if bench_substrings.is_empty()
            || bench_substrings
                .iter()
                .any(|name| bench_name.contains(name))
        {
            let bench = executor_benches
                .iter()
                .find(|(id, _)| executor == *id)
                .unwrap()
                .1;
            benches.entry(*group).or_default().insert(*item, bench);
        }
    }

    if benches.is_empty() {
        println!("No matching benches found");

        return Ok(());
    }

    // Open the result file if requested.
    let mut output = output
        .map(|filename| {
            File::create(filename.clone())
                .map_err(|_| format!("Could not open file <{}>", filename.to_str().unwrap()))
        })
        .transpose()?;

    // Run sequentially all requested benchmarks.
    for (group, benches) in benches {
        println!(
            "Running benchmark '{group}' with the {} runtime.",
            executor.name()
        );
        if samples.get() != 1 {
            println!("All results are averaged over {samples} runs.");
        }

        // Only used when saving to file.
        let mut column_headers = Vec::new();
        let mut parameter_column = Vec::new();
        let mut columns = Vec::new();

        for (bench_id, (name, bench)) in benches.into_iter().enumerate() {
            println!("    {name}:");
            let mut data_column = Vec::new();

            for (
                parameter_id,
                BenchResult {
                    label,
                    parameter,
                    throughput,
                },
            ) in bench(samples).enumerate()
            {
                assert!(!throughput.is_empty());

                let mean = throughput.iter().fold(0f64, |acc, s| acc + s) / throughput.len() as f64;

                if output.is_some() {
                    if bench_id == 0 && parameter_id == 0 {
                        column_headers.push(label.clone());
                    }
                    if bench_id == 0 {
                        parameter_column.push(parameter.clone());
                    }
                    data_column.push(format!("{mean:.0}"));
                }

                if throughput.len() == 1 {
                    println!(
                        "        {:<20} {:>12.3} msg/µs",
                        format!("{label}={parameter}"),
                        mean / 1e6
                    );
                } else {
                    let std_dev = (throughput
                        .iter()
                        .fold(0f64, |acc, s| acc + (s - mean) * (s - mean))
                        / throughput.len() as f64)
                        .sqrt();

                    println!(
                        "        {:<20} {:>12.3} msg/µs [±{:.3}]",
                        format!("{label}: {parameter}"),
                        mean * 1e-6,
                        std_dev * 1e-6
                    );
                }
            }
            if output.is_some() {
                columns.push(data_column);
                column_headers.push(String::from(name));
            }
            println!();
        }

        // Save to file if requested.
        if let Some(file) = &mut output {
            columns.insert(0, parameter_column);
            writeln!(
                file,
                "# '{}' benchmark with {} runtime",
                group,
                executor.name()
            )
            .unwrap();
            write!(file, "#").unwrap();
            for header in column_headers {
                write!(file, "{header:>15} ").unwrap();
            }
            writeln!(file).unwrap();
            for row in 0..columns[0].len() {
                for column in &columns {
                    write!(file, " {:>15}", column[row]).unwrap();
                }
                writeln!(file).unwrap();
            }
            writeln!(file).unwrap();
        }
    }

    Ok(())
}
