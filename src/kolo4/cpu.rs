struct Process {
    dead: bool,
    pc: u8,
    stack: [u32; 16],
    stack_cap: u8,
}

type OpResult<T> = Result<T, ()>;

impl Process {
    fn new(offset: u8) -> Self {
        Self {
            dead: false,
            pc: offset,
            stack: [0; 16],
            stack_cap: 0,
        }
    }
    fn ensure_stack_cap(&self, min_cap: u8) -> OpResult<()> {
        if self.stack_cap < min_cap {
            return Err(());
        } else {
            return Ok(());
        }
    }
    fn stack_push(&mut self, val: u32) -> OpResult<()> {
        if self.stack_cap == self.stack.len() as u8 {
            return Err(());
        } else {
            self.stack[self.stack_cap as usize] = val;
            self.stack_cap += 1;
            return Ok(());
        }
    }
    fn stack_pop(&mut self) -> OpResult<u32> {
        if self.stack_cap == 0 {
            return Err(());
        } else {
            self.stack_cap -= 1;
            return Ok(self.stack[self.stack_cap as usize]);
        }
    }
    fn process_adress(addr: u32) -> OpResult<usize> {
        Ok(addr as usize)
    }
    fn step(
        &mut self,
        opcode: u8,
        immediate: u32,
        mem: &mut [u32; 256],
        program_index: u32,
        awaiting_teleport: &mut Vec<u32>,
    ) -> OpResult<()> {
        match opcode {
            0x00 /* NOP */  => (),
            0x01 /* PC */  => {
                self.stack_push(self.pc as u32)?;
            }
            0x02 /* PUSH */  => {
                self.stack_push(immediate)?;
            }
            0x03 /* POP */  => {
                self.stack_pop()?;
            }
            0x04 /* SWAP */  => {
                self.ensure_stack_cap(2)?;
                let cap = self.stack_cap as usize - 1;
                let tmp = self.stack[cap];
                self.stack[cap] = self.stack[cap - 1];
                self.stack[cap - 1] = tmp;
            }
            0x05 /* DUP */  => {
                self.ensure_stack_cap(1)?;
                self.stack_push(self.stack[(self.stack_cap - 1) as usize])?;
            }
            0x06 /* PUSHSSZ */  => {
                self.stack_push(self.stack_cap as u32)?;
            }
            0x07 /* LOAD */  => {
                let addr = self.stack_pop()?;
                let addr = Self::process_adress(addr)?;
                let val = mem[addr];
                self.stack_push(val)?;
            }
            0x08 /* STORE */  => {
                let addr = self.stack_pop()?;
                let addr = Self::process_adress(addr)?;
                let val = self.stack_pop()?;

                // if addr != 0 {
                    mem[addr] = val;
                // }
            }
            0x09 /* ADD */  => {
                let a = self.stack_pop()?;
                let b = self.stack_pop()?;
                let c = a + b;
                self.stack_push(c)?;
            }
            0x0a /* SUB */ => {
                let a = self.stack_pop()?;
                let b = self.stack_pop()?;
                let c = a - b;
                self.stack_push(c)?;
            }
            0x0b /* DIV */ => {
                let a = self.stack_pop()?;
                let b = self.stack_pop()?;
                if b == 0 {
                    return Err(());
                }
                let c = a / b;
                self.stack_push(c)?;
            }
            0x0c /* POW */ => {
                let a = self.stack_pop()?;
                let b = self.stack_pop()?;
                // rust 0.pow(0) is 1
                let c = a.pow(b);
                self.stack_push(c)?;
            }
            0x0d /* BRZ */ => {
                let val = self.stack_pop()?;
                if val == 0 {
                    self.pc = self.pc.wrapping_add(immediate as u8);
                }
            }
            0x0e /* BR3 */ => {
                let val = self.stack_pop()?;
                if val == 3 {
                    self.pc = self.pc.wrapping_add(immediate as u8);
                }
            }
            0x0f /* BR7 */ => {
                let val = self.stack_pop()?;
                if val == 7 {
                    self.pc = self.pc.wrapping_add(immediate as u8);
                }
            }
            0x10 /* BRGE */ => {
                let val_a = self.stack_pop()?;
                let val_b = self.stack_pop()?;
                if val_a >= val_b {
                    self.pc = self.pc.wrapping_add(immediate as u8);
                }
            }
            0x11 /* JMP */ => {
                self.pc = self.pc.wrapping_add(immediate as u8);
            }
            0x12 /* ARMED_BOMB */ => {
                return Err(());
            }
            0x13 /* BOMB */ => {
                mem[self.pc as usize] = 0x12; /* ARMED_BOMB */
            }
            0x14 /* TLPORT */ => {
                awaiting_teleport.push(program_index);
                // decrement pc because it will be incremented again at the start of the program and it is explicitly stated that pc doesn't increment after this instruction
                // it is done this way instead of dying and staying in the awaiting_teleport until more teleports are pushed to preserve order within a cycle
                self.pc = self.pc.wrapping_sub(1);
            }
            0x15 /* JNTAR */ => {
                for offset in [2, 4, 8] {
                    // should memory load quirks apply here as well?
                    let a = self.pc.wrapping_add(offset) as usize;
                    let b = self.pc.wrapping_sub(offset) as usize;
                    mem[a] = 0x13; /* BOMB */
                    mem[b] = 0x13;
                }
            }
            _ => return Err(())
        }
        return Ok(());
    }
}

fn stdin_line() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string
}

#[allow(non_snake_case)]
fn main() {
    let line = stdin_line();
    let mut split = line.split_whitespace();

    let Q = split.next().unwrap().parse::<usize>().unwrap();

    let mut processes = Vec::new();
    let mem = {
        static mut MEM_BUF: [u32; 256] = [0; 256];
        unsafe { &mut MEM_BUF }
    };

    for _ in 0..Q {
        let P = stdin_line().trim_end().parse::<usize>().unwrap();

        processes.clear();
        mem.fill(0);

        for _ in 0..P {
            let line = stdin_line();
            let mut split = line.split_whitespace();

            let offset = split.next().unwrap().parse::<u8>().unwrap();
            let n = split.next().unwrap().parse::<u8>().unwrap();

            processes.push(Process::new(offset));

            let offset = offset as usize;
            let n = n as usize;
            mem[offset..(offset + n)].fill_with(|| split.next().unwrap().parse::<u32>().unwrap());
        }

        // 0 is always 0
        mem[0] = 0;

        let mut awaiting_teleport = Vec::new();
        for _ in 0..5000 {
            for (i, proc) in processes.iter_mut().enumerate() {
                if proc.dead {
                    continue;
                }

                mem[0] = 0;

                let instr = mem[proc.pc as usize];

                let opcode = instr & 0xFF;
                let immediate = (instr >> 8) & 0xFFFF;

                match proc.step(
                    opcode as u8,
                    immediate,
                    mem,
                    i as u32,
                    &mut awaiting_teleport,
                ) {
                    Ok(_) => (),
                    Err(_) => {
                        proc.dead = true;
                        continue;
                    }
                }

                proc.pc = proc.pc.wrapping_add(1);
            }

            if awaiting_teleport.len() >= 2 {
                let first_pc = processes[0].pc;
                let mut dst = 0;
                let mut src = 1;
                while src < awaiting_teleport.len() {
                    processes[dst as usize].pc = processes[src as usize].pc.wrapping_add(1);

                    src = dst;
                    dst += 1;
                }
                processes.last_mut().unwrap().pc = first_pc.wrapping_add(1);
            }

            awaiting_teleport.clear();
        }

        let the_answer = mem[42];
        let b = processes.iter().map(|p| p.pc as u32).sum::<u32>();

        println!("{} {}", the_answer, b);
    }
}
