use ark_ff::FftField;
use ark_poly::{domain::EvaluationDomain, Radix2EvaluationDomain};

pub mod ext_wit;
pub mod pre_processing;
pub mod prove;
pub mod proving_key;
pub mod qap;

#[derive(Debug, Clone, PartialEq)]
pub struct ConstraintDomain<F>
where
    F: FftField,
{
    pub m: usize,                               // Constraint size
    pub constraint: Radix2EvaluationDomain<F>,  // Constraint domain
    pub constraint2: Radix2EvaluationDomain<F>, // Constraint2 domain
}

impl<F: FftField> ConstraintDomain<F> {
    #[allow(unused)]
    pub fn new(m: usize) -> Self {
        let constraint = Radix2EvaluationDomain::<F>::new(m).unwrap();
        let constraint2 = Radix2EvaluationDomain::<F>::new(2 * m).unwrap();

        debug_assert_eq!(constraint.size(), m);
        debug_assert_eq!(constraint2.size(), 2 * m);

        ConstraintDomain {
            m,
            constraint,
            constraint2,
        }
    }
}

#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
core::arch::global_asm!(
    ".intel_syntax noprefix",
    ".global __rust_probestack",
    "__rust_probestack:",
    "    push rcx",
    "    mov rcx, rax",
    "2:",
    "    cmp rcx, 4096",
    "    jl 3f",
    "    sub rsp, 4096",
    "    test [rsp], rsp",
    "    sub rcx, 4096",
    "    jmp 2b",
    "3:",
    "    sub rsp, rcx",
    "    test [rsp], rsp",
    "    add rsp, rax",
    "    pop rcx",
    "    ret",
    ".att_syntax prefix"
);
