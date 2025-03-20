#[derive(Debug, Default, PartialEq)]
pub struct CPURegisters {
    /// x1 -> Return address
    ra: u64,
    /// x2 -> Stack pointer
    sp: u64,
    /// x3 -> Global pointer
    gp: u64,
    /// x4 -> Thread pointer
    tp: u64,
    /// x5-7 -> Temporaries
    t0: u64,
    t1: u64,
    t2: u64,
    /// x8 -> Saved/frame pointer
    s0: u64,
    /// x9 -> Saved register
    s1: u64,
    /// x10-11 -> Fn args / return values
    a0: u64,
    a1: u64,
    /// x12-x17 -> Fn args
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
    a7: u64,
    /// x18-x27 -> Saved registers
    s2: u64,
    s3: u64,
    s4: u64,
    s5: u64,
    s6: u64,
    s7: u64,
    s8: u64,
    s9: u64,
    s10: u64,
    s11: u64,
    /// x28-x31 -> Temporaries
    t3: u64,
    t4: u64,
    t5: u64,
    t6: u64,
}

impl CPURegistersAccessTrait for CPURegisters {
    fn get_registers(&self) -> &CPURegisters {
        self
    }

    fn get_registers_mut(&mut self) -> &mut CPURegisters {
        self
    }
}

pub trait CPURegistersAccessTrait {
    fn get_registers(&self) -> &CPURegisters;
    fn get_registers_mut(&mut self) -> &mut CPURegisters;

    fn get_register(&self, index: u8) -> u64 {
        match index & 0b0001_1111 {
            0 => 0,
            1 => self.get_ra(),
            2 => self.get_sp(),
            3 => self.get_gp(),
            4 => self.get_tp(),
            5 => self.get_t0(),
            6 => self.get_t1(),
            7 => self.get_t2(),
            8 => self.get_s0(),
            9 => self.get_s1(),
            10 => self.get_a0(),
            11 => self.get_a1(),
            12 => self.get_a2(),
            13 => self.get_a3(),
            14 => self.get_a4(),
            15 => self.get_a5(),
            16 => self.get_a6(),
            17 => self.get_a7(),
            18 => self.get_s2(),
            19 => self.get_s3(),
            20 => self.get_s4(),
            21 => self.get_s5(),
            22 => self.get_s6(),
            23 => self.get_s7(),
            24 => self.get_s8(),
            25 => self.get_s9(),
            26 => self.get_s10(),
            27 => self.get_s11(),
            28 => self.get_t3(),
            29 => self.get_t4(),
            30 => self.get_t5(),
            31 => self.get_t6(),
            _ => unreachable!(),
        }
    }

    fn set_register(&mut self, index: u8, value: u64) {
        match index & 0b0001_1111 {
            0 => {}
            1 => self.set_ra(value),
            2 => self.set_sp(value),
            3 => self.set_gp(value),
            4 => self.set_tp(value),
            5 => self.set_t0(value),
            6 => self.set_t1(value),
            7 => self.set_t2(value),
            8 => self.set_s0(value),
            9 => self.set_s1(value),
            10 => self.set_a0(value),
            11 => self.set_a1(value),
            12 => self.set_a2(value),
            13 => self.set_a3(value),
            14 => self.set_a4(value),
            15 => self.set_a5(value),
            16 => self.set_a6(value),
            17 => self.set_a7(value),
            18 => self.set_s2(value),
            19 => self.set_s3(value),
            20 => self.set_s4(value),
            21 => self.set_s5(value),
            22 => self.set_s6(value),
            23 => self.set_s7(value),
            24 => self.set_s8(value),
            25 => self.set_s9(value),
            26 => self.set_s10(value),
            27 => self.set_s11(value),
            28 => self.set_t3(value),
            29 => self.set_t4(value),
            30 => self.set_t5(value),
            31 => self.set_t6(value),
            _ => unreachable!(),
        }
    }

    fn get_ra(&self) -> u64 {
        self.get_registers().ra
    }

    fn set_ra(&mut self, value: u64) {
        self.get_registers_mut().ra = value;
    }

    fn get_sp(&self) -> u64 {
        self.get_registers().sp
    }

    fn set_sp(&mut self, value: u64) {
        self.get_registers_mut().sp = value;
    }

    fn get_gp(&self) -> u64 {
        self.get_registers().gp
    }

    fn set_gp(&mut self, value: u64) {
        self.get_registers_mut().gp = value;
    }

    fn get_tp(&self) -> u64 {
        self.get_registers().tp
    }

    fn set_tp(&mut self, value: u64) {
        self.get_registers_mut().tp = value;
    }

    fn get_t0(&self) -> u64 {
        self.get_registers().t0
    }

    fn set_t0(&mut self, value: u64) {
        self.get_registers_mut().t0 = value;
    }

    fn get_t1(&self) -> u64 {
        self.get_registers().t1
    }

    fn set_t1(&mut self, value: u64) {
        self.get_registers_mut().t1 = value;
    }

    fn get_t2(&self) -> u64 {
        self.get_registers().t2
    }

    fn set_t2(&mut self, value: u64) {
        self.get_registers_mut().t2 = value;
    }

    fn get_s0(&self) -> u64 {
        self.get_registers().s0
    }

    fn set_s0(&mut self, value: u64) {
        self.get_registers_mut().s0 = value;
    }

    fn get_s1(&self) -> u64 {
        self.get_registers().s1
    }

    fn set_s1(&mut self, value: u64) {
        self.get_registers_mut().s1 = value;
    }

    fn get_a0(&self) -> u64 {
        self.get_registers().a0
    }

    fn set_a0(&mut self, value: u64) {
        self.get_registers_mut().a0 = value;
    }

    fn get_a1(&self) -> u64 {
        self.get_registers().a1
    }

    fn set_a1(&mut self, value: u64) {
        self.get_registers_mut().a1 = value;
    }

    fn get_a2(&self) -> u64 {
        self.get_registers().a2
    }

    fn set_a2(&mut self, value: u64) {
        self.get_registers_mut().a2 = value;
    }

    fn get_a3(&self) -> u64 {
        self.get_registers().a3
    }

    fn set_a3(&mut self, value: u64) {
        self.get_registers_mut().a3 = value;
    }

    fn get_a4(&self) -> u64 {
        self.get_registers().a4
    }

    fn set_a4(&mut self, value: u64) {
        self.get_registers_mut().a4 = value;
    }

    fn get_a5(&self) -> u64 {
        self.get_registers().a5
    }

    fn set_a5(&mut self, value: u64) {
        self.get_registers_mut().a5 = value;
    }

    fn get_a6(&self) -> u64 {
        self.get_registers().a6
    }

    fn set_a6(&mut self, value: u64) {
        self.get_registers_mut().a6 = value;
    }

    fn get_a7(&self) -> u64 {
        self.get_registers().a7
    }

    fn set_a7(&mut self, value: u64) {
        self.get_registers_mut().a7 = value;
    }

    fn get_s2(&self) -> u64 {
        self.get_registers().s2
    }

    fn set_s2(&mut self, value: u64) {
        self.get_registers_mut().s2 = value;
    }

    fn get_s3(&self) -> u64 {
        self.get_registers().s3
    }

    fn set_s3(&mut self, value: u64) {
        self.get_registers_mut().s3 = value;
    }

    fn get_s4(&self) -> u64 {
        self.get_registers().s4
    }

    fn set_s4(&mut self, value: u64) {
        self.get_registers_mut().s4 = value;
    }

    fn get_s5(&self) -> u64 {
        self.get_registers().s5
    }

    fn set_s5(&mut self, value: u64) {
        self.get_registers_mut().s5 = value;
    }

    fn get_s6(&self) -> u64 {
        self.get_registers().s6
    }

    fn set_s6(&mut self, value: u64) {
        self.get_registers_mut().s6 = value;
    }

    fn get_s7(&self) -> u64 {
        self.get_registers().s7
    }

    fn set_s7(&mut self, value: u64) {
        self.get_registers_mut().s7 = value;
    }

    fn get_s8(&self) -> u64 {
        self.get_registers().s8
    }

    fn set_s8(&mut self, value: u64) {
        self.get_registers_mut().s8 = value;
    }

    fn get_s9(&self) -> u64 {
        self.get_registers().s9
    }

    fn set_s9(&mut self, value: u64) {
        self.get_registers_mut().s9 = value;
    }

    fn get_s10(&self) -> u64 {
        self.get_registers().s10
    }

    fn set_s10(&mut self, value: u64) {
        self.get_registers_mut().s10 = value;
    }

    fn get_s11(&self) -> u64 {
        self.get_registers().s11
    }

    fn set_s11(&mut self, value: u64) {
        self.get_registers_mut().s11 = value;
    }

    fn get_t3(&self) -> u64 {
        self.get_registers().t3
    }

    fn set_t3(&mut self, value: u64) {
        self.get_registers_mut().t3 = value;
    }

    fn get_t4(&self) -> u64 {
        self.get_registers().t4
    }

    fn set_t4(&mut self, value: u64) {
        self.get_registers_mut().t4 = value;
    }

    fn get_t5(&self) -> u64 {
        self.get_registers().t5
    }

    fn set_t5(&mut self, value: u64) {
        self.get_registers_mut().t5 = value;
    }

    fn get_t6(&self) -> u64 {
        self.get_registers().t6
    }

    fn set_t6(&mut self, value: u64) {
        self.get_registers_mut().t6 = value;
    }
}
