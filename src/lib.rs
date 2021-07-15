mod params;
mod error;
mod model;

pub use params::{ParamBuilder, ParamSamples};

use std::{
    collections::HashMap,
    env::args,
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use itertools::Itertools;
use rand::prelude::IteratorRandom;

pub fn black_box<T>(dummy: T) -> T { 
    unsafe {
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret 
    }   
}

fn check_valgrind() -> bool {
    let result = Command::new("valgrind")
        .arg("--tool=cachegrind")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match result {
        Err(e) => {
            println!("Unexpected error while launching valgrind. Error: {}", e);
            false
        }
        Ok(status) => {
            if status.success() {
                true
            } else {
                println!("Failed to launch valgrind. Error: {}. Please ensure that valgrind   is installed and on the $PATH.", status);
                false
            }
        }
    }
}

fn get_arch() -> String {
    let output = Command::new("uname")
        .arg("-m")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to run `uname` to determine CPU architecture.");

    String::from_utf8(output.stdout)
        .expect("`-uname -m` returned invalid unicode.")
        .trim()
        .to_owned()
}

fn basic_valgrind() -> Command {
    Command::new("valgrind")
}

fn run_bench<'a>(
    arch: &str,
    executable: &str,
    i: usize,
    params: &ParamSamples,
    name: &str,
    allow_aslr: bool,
) -> (CachegrindStats, Option<CachegrindStats>) {
    let output_file = PathBuf::from(format!("target/alco/cachegrind.out.{}", name));
    let old_file = output_file.with_file_name(format!("cachegrind.out.{}.old", name));
    std::fs::create_dir_all(output_file.parent().unwrap()).expect("Failed to create directo  ry");

    if output_file.exists() {
        // Already run this benchmark once; move last results to .old
        std::fs::copy(&output_file, &old_file).unwrap();
    }
                                                                                           
    let mut cmd = if allow_aslr {
        basic_valgrind()
    } else {
        //valgrind_without_aslr(arch)
        panic!("");
    };
    let mut status = cmd
        .arg("--tool=cachegrind")
        // Set some reasonable cache sizes. The exact sizes matter less than having fixed s  izes,
        // since otherwise cachegrind would take them from the CPU and make benchmark runs
        // even more incomparable between machines.
        .arg("--I1=32768,8,64")
        .arg("--D1=32768,8,64")
        .arg("--LL=8388608,16,64")
        .arg(format!("--cachegrind-out-file={}", output_file.display()))
        .arg(executable)
        .arg("--alco-run")
        .arg(i.to_string());

    for arg in params.to_string() {
        status = status.arg(arg);
    }

    let status = status
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("Failed to run benchmark in cachegrind");

    if !status.success() {
        panic!(
            "Failed to run benchmark in cachegrind. Exit code: {}",
            status
        );
    }

    let new_stats = parse_cachegrind_output(&output_file);
    let old_stats = if old_file.exists() {
        Some(parse_cachegrind_output(&old_file))
    } else {
        None
    };
  
    (new_stats, old_stats)
}

fn parse_cachegrind_output(file: &Path) -> CachegrindStats {
    let mut events_line = None;
    let mut summary_line = None;

    let file_in = File::open(file).expect("Unable to open cachegrind output file");

    for line in BufReader::new(file_in).lines() {
        let line = line.unwrap();
        if let Some(line) = line.strip_prefix("events: ") {
            events_line = Some(line.trim().to_owned());
        }
        if let Some(line) = line.strip_prefix("summary: ") {
            summary_line = Some(line.trim().to_owned());
        }
    }

    match (events_line, summary_line) {
        (Some(events), Some(summary)) => {
            let events: HashMap<_, _> = events
                .split_whitespace()
                .zip(summary.split_whitespace().map(|s| {
                    s.parse::<u64>()
                        .expect("Unable to parse summary line from cachegrind output file")                  }))
                .collect();

            CachegrindStats {
                instruction_reads: events["Ir"],
                instruction_l1_misses: events["I1mr"],
                instruction_cache_misses: events["ILmr"],
                data_reads: events["Dr"],
                data_l1_read_misses: events["D1mr"],
                data_cache_read_misses: events["DLmr"],
                data_writes: events["Dw"],
                data_l1_write_misses: events["D1mw"],
                data_cache_write_misses: events["DLmw"],
            }
        }
        _ => panic!("Unable to parse cachegrind output file"),
    }
}

impl CachegrindStats {
    pub fn ram_accesses(&self) -> u64 {
        self.instruction_cache_misses + self.data_cache_read_misses + self.data_cache_write_misses
    }

    pub fn summarize(&self) -> CachegrindSummary {
        let ram_hits = self.ram_accesses();
        let l3_accesses =
            self.instruction_l1_misses + self.data_l1_read_misses + self.data_l1_write_misses;
        let l3_hits = l3_accesses - ram_hits;

        let total_memory_rw = self.instruction_reads + self.data_reads + self.data_writes;
        let l1_hits = total_memory_rw - (ram_hits + l3_hits);

        CachegrindSummary {
            l1_hits,
            l3_hits,
            ram_hits,
        }
    }
}

#[derive(Clone, Debug)]
struct CachegrindStats {
    instruction_reads: u64,
    instruction_l1_misses: u64,
    instruction_cache_misses: u64,
    data_reads: u64,
    data_l1_read_misses: u64,
    data_cache_read_misses: u64,
    data_writes: u64,
    data_l1_write_misses: u64,
    data_cache_write_misses: u64,
}

#[derive(Clone, Debug)]
struct CachegrindSummary {
    l1_hits: u64,
    l3_hits: u64,
    ram_hits: u64,
}

/// Custom-test-framework runner. Should not be called directly.
#[doc(hidden)]
pub fn runner<'a>(benches: &'a [&(&'static str, fn(ParamSamples), ParamBuilder<'a>)]) {
    let mut args_iter = args();
    let executable = args_iter.next().unwrap();

    if let Some("--alco-run") = args_iter.next().as_deref() {
        // In this branch, we're running under cachegrind, so execute the benchmark as quic  kly as
        // possible and exit
        let index: usize = args_iter.next().unwrap().parse().unwrap();
        let args: String = args_iter.collect::<Vec<_>>().join(" ");

        let params = ParamSamples::from_string(args).unwrap();
        (benches[index].1)(params);

        return;
    }

    // Otherwise we're running normally, under cargo
    if !check_valgrind() {
        return;
    }

    let arch = get_arch();

    let allow_aslr = true; //std::env::var_os("IAI_ALLOW_ASLR").is_some();

    // sampling parameters
    let (num_seeding_steps, num_steps, min_change) = (6, 30, 50);

    // model estimation parameters
    let (beam_size, max_interactions) = (4, 3);

    for (i, (name, _func, param_builder)) in benches.iter().enumerate() {
        println!("{}", name);

        let (calibration, old_calibration) =
            run_bench(&arch, &executable, i, &param_builder.lower_bound(), "alco_calibration", allow_aslr);

        dbg!(&calibration.instruction_reads);
        dbg!(&calibration.summarize());

        // fit an unimodal polynomial to each term and increase such that the change in
        // instructions is significant
        //
        let mut samples: Vec<(&str, Vec<usize>)> = Vec::new();
        let mut dataset = Vec::new();
        for (param_name, param) in param_builder.params() {
            // if the parameter is a item set, then just collect all indices as samples.
            // we have to try out every item anyways.
            if let Some(length) = param.num_items() {
                samples.push((param_name, (0..length).collect()));
                continue;
            }

            let mut params = param_builder.lower_bound();
            let mut results = vec![];
            let mut current_step = 0;

            for _ in 0..num_seeding_steps {
                params = param_builder.update_step(params, param_name, current_step);
        
                // pass params and calculate stats
                let (stats, old_stats) = run_bench(&arch, &executable, i, &params, name, allow_aslr  );

                let instruction_delta = stats.instruction_reads - calibration.instruction_reads;

                results.push((current_step, instruction_delta));
                dataset.push((params.clone(), instruction_delta));

                current_step += match model::estimate_stepsize(&results, min_change) {
                    0 => break,
                    x => x,
                };
            }

            let results = results.into_iter().map(|x| x.0).collect();
            samples.push((param_name, results));
        }

        // sample with combinations of sample points estimated in previous step. The sample points
        // are randomly permuted and the estimated instruction counter saved into a dataset.
        let mut rng = &mut rand::thread_rng();

        let combs = samples.iter().map(|x| 0..x.1.len())
            .multi_cartesian_product()
            .choose_multiple(&mut rng, num_steps);

        for comb in combs {
            let indices: Vec<(&str, usize)> = samples.iter().zip(comb).map(|(a, b)| (a.0, a.1[b])).collect();
            let params = param_builder.from_indices(indices).unwrap();

            // pass params and calculate stats
            let (stats, old_stats) = run_bench(&arch, &executable, i, &params, name, allow_aslr  );

            let instruction_delta = stats.instruction_reads - calibration.instruction_reads;

            // push current instruction to dataset
            dataset.push((params, instruction_delta));
        }

        // estimate an additive model with beam-search and limited interactions between terms
        let estimation = model::fit_greedy_additive(dataset, beam_size, max_interactions);

        // print complexity estimation (may write to file in future)
        dbg!(&estimation);
    }
}

