#![allow(non_local_definitions)]
use ark_ff::{MontBackend, Fp64, MontConfig};

#[derive(MontConfig)]
#[modulus = "7"]
#[generator = "3"]

pub struct FConfig;

pub type F = Fp64<MontBackend<FConfig,1>>;
