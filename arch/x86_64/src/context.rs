pub struct Context {
    flags: usize,
    bx: usize,
    r12: usize,
    r13: usize,
    r14: usize,
    r15: usize,
    bp: usize,
    sp: usize,
    cr3: usize
}

impl Context {
    pub fn new() -> Context {
        Context {
            flags: 0,
            bx: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            bp: 0,
            sp: 0,
            cr3: 0
        }
    }

    #[inline(never)]
    #[naked]
    pub unsafe fn switch_to(&mut self, next: &mut Context) {
        asm!("xchg bx, bx" : : : "memory" : "intel", "volatile");

/*
        asm!("fxsave [$0]" : : "r"(self.fx) : "memory" : "intel", "volatile");
        self.loadable = true;
        if next.loadable {
            asm!("fxrstor [$0]" : : "r"(next.fx) : "memory" : "intel", "volatile");
        }else{
            asm!("fninit" : : : "memory" : "intel", "volatile");
        }
*/

        asm!("pushfq ; pop $0" : "=r"(self.flags) : : "memory" : "intel", "volatile");
        asm!("push $0 ; popfq" : : "r"(next.flags) : "memory" : "intel", "volatile");

        asm!("mov $0, rbx" : "=r"(self.bx) : : "memory" : "intel", "volatile");
        asm!("mov rbx, $0" : : "r"(next.bx) : "memory" : "intel", "volatile");

        asm!("mov $0, r12" : "=r"(self.r12) : : "memory" : "intel", "volatile");
        asm!("mov r12, $0" : : "r"(next.r12) : "memory" : "intel", "volatile");

        asm!("mov $0, r13" : "=r"(self.r13) : : "memory" : "intel", "volatile");
        asm!("mov r13, $0" : : "r"(next.r13) : "memory" : "intel", "volatile");

        asm!("mov $0, r14" : "=r"(self.r14) : : "memory" : "intel", "volatile");
        asm!("mov r14, $0" : : "r"(next.r14) : "memory" : "intel", "volatile");

        asm!("mov $0, r15" : "=r"(self.r15) : : "memory" : "intel", "volatile");
        asm!("mov r15, $0" : : "r"(next.r15) : "memory" : "intel", "volatile");

        asm!("mov $0, rbp" : "=r"(self.bp) : : "memory" : "intel", "volatile");
        asm!("mov rbp, $0" : : "r"(next.bp) : "memory" : "intel", "volatile");

        asm!("mov $0, rsp" : "=r"(self.sp) : : "memory" : "intel", "volatile");
        asm!("mov rsp, $0" : : "r"(next.sp) : "memory" : "intel", "volatile");

        /* TODO
        asm!("mov $0, cr3" : "=r"(self.cr3) : : "memory" : "intel", "volatile");
        asm!("mov cr3, $0" : : "r"(self.cr3) : "memory" : "intel", "volatile");
        */
    }
}
