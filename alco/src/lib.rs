use std::{
    collections::HashMap,
    env::args,
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

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

type CachegrindStats = usize;
fn run_bench(
    arch: &str,
    executable: &str,
    i: usize,
    params: &[usize],
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

    for p in params {
        status = status.arg(p.to_string());
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

    /*let new_stats = parse_cachegrind_output(&output_file);
    let old_stats = if old_file.exists() {
        Some(parse_cachegrind_output(&old_file))
    } else {
        None
    };
  
    (new_stats, old_stats)*/
    panic!("");
}

/// Custom-test-framework runner. Should not be called directly.
#[doc(hidden)]
pub fn runner(benches: &[&(&'static str, fn(Vec<usize>, bool))]) {
    let mut args_iter = args();
    let executable = args_iter.next().unwrap();

    if let Some("--alco-run") = args_iter.next().as_deref() {
        // In this branch, we're running under cachegrind, so execute the benchmark as quic  kly as
        // possible and exit
        let index: usize = args_iter.next().unwrap().parse().unwrap();
        let args: Vec<usize> = args_iter.map(|x| x.parse().unwrap()).collect();

        // with no dimension argument, just measure the set-up time
        if args.len() == 0 {
            (benches[index].1)(args, true);

            return;
        }

        (benches[index].1)(args, false);

        return;
    }

    // Otherwise we're running normally, under cargo
    if !check_valgrind() {
        return;
    }

    let arch = get_arch();

    let allow_aslr = std::env::var_os("IAI_ALLOW_ASLR").is_some();

    for (i, (name, _func)) in benches.iter().enumerate() {
        println!("calibrate");
        let (calibration, old_calibration) =
            run_bench(&arch, &executable, i, &[], "alco_calibration", allow_aslr);

        println!("{}", name);
        let (stats, old_stats) = run_bench(&arch, &executable, i, &[0], name, allow_aslr  );
    }
}

