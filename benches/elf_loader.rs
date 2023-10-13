// Copyright 2020 PUT Maintainers <maintainers@put.com>
//
// Licensed under the Apache License, Version 2.0 <http://www.apache.org/licenses/LICENSE-2.0> or
// the MIT license <http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![feature(test)]

extern crate put_rbpf;
extern crate test;
extern crate put_test_utils;

use put_rbpf::{
    elf::Executable,
    syscalls::bpf_syscall_u64,
    verifier::TautologyVerifier,
    vm::{BuiltinProgram, Config, TestContextObject},
};
use std::{fs::File, io::Read, sync::Arc};
use test::Bencher;

fn loader() -> Arc<BuiltinProgram<TestContextObject>> {
    let mut loader = BuiltinProgram::new_loader(Config::default());
    loader
        .register_function(b"log_64", bpf_syscall_u64)
        .unwrap();
    Arc::new(loader)
}

#[bench]
fn bench_load_elf(bencher: &mut Bencher) {
    let mut file = File::open("tests/elfs/noro.so").unwrap();
    let mut elf = Vec::new();
    file.read_to_end(&mut elf).unwrap();
    let loader = loader();
    bencher.iter(|| {
        Executable::<TautologyVerifier, TestContextObject>::from_elf(&elf, loader.clone()).unwrap()
    });
}

#[bench]
fn bench_load_elf_without_syscall(bencher: &mut Bencher) {
    let mut file = File::open("tests/elfs/noro.so").unwrap();
    let mut elf = Vec::new();
    file.read_to_end(&mut elf).unwrap();
    let loader = loader();
    bencher.iter(|| {
        Executable::<TautologyVerifier, TestContextObject>::from_elf(&elf, loader.clone()).unwrap()
    });
}

#[bench]
fn bench_load_elf_with_syscall(bencher: &mut Bencher) {
    let mut file = File::open("tests/elfs/noro.so").unwrap();
    let mut elf = Vec::new();
    file.read_to_end(&mut elf).unwrap();
    let loader = loader();
    bencher.iter(|| {
        Executable::<TautologyVerifier, TestContextObject>::from_elf(&elf, loader.clone()).unwrap()
    });
}
