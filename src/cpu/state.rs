use crate::cpu::instruction::{IndexMode, TargetRegister};

#[derive(Clone, Copy)]
pub enum BreakSignal {
    None,
    NMI,
    IRQ,
}

pub enum CPUState {
    FetchInstruction,
    FetchOperand,
    JumpAbsolute,
    JumpIndirect(i32),
    IndexedRead(IndexMode),
    FetchOperandHigh(Option<IndexMode>),
    Indirect(i32, IndexMode),
    DummyRead,
    Read,
    DummyWrite,
    Write,
    Break(i32, BreakSignal),
    JumpSubroutine(i32),
    ReturnInterrupt(i32),
    ReturnSubroutine(i32),
    PushRegister(TargetRegister),
    PullRegister(i32, TargetRegister),
}

impl CPUState {
    pub fn get_mode(&self) -> CycleMode {
        match self {
            CPUState::FetchInstruction | CPUState::FetchOperand | CPUState::FetchOperandHigh(_) | CPUState::JumpAbsolute
            | CPUState::JumpIndirect(0) | CPUState::ReturnSubroutine(3) | CPUState::JumpSubroutine(3) | CPUState::Break(-1, _) => CycleMode::Fetch,

            CPUState::JumpIndirect(_) | CPUState::IndexedRead(_) | CPUState::Indirect(_, _)
            | CPUState::DummyRead | CPUState::Read | CPUState::Break(3, _) | CPUState::Break(4, _) => CycleMode::Read,

            CPUState::ReturnInterrupt(3) | CPUState::ReturnSubroutine(2) | CPUState::PullRegister(1, _) | CPUState::JumpSubroutine(0) => CycleMode::Peak,
            CPUState::ReturnInterrupt(_) | CPUState::ReturnSubroutine(_) | CPUState::PullRegister(_, _) => CycleMode::Pop,
            CPUState::Write | CPUState::DummyWrite => CycleMode::Write,
            CPUState::Break(_, _) | CPUState::PushRegister(_) | CPUState::JumpSubroutine(_) => CycleMode::Push,
        }
    }
}

pub enum CycleMode {
    Fetch,
    Read,
    Write,
    Push,
    Pop,
    Peak
}
