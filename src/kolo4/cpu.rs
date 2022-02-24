#![allow(unused)]

struct Process {
    stopped: bool,
    pc: u8,
    stack: [u32; 16],
    stack_cap: u8,
}

type OpResult<T> = Result<T, ()>;

impl Process {
    fn new(offset: u8) -> Self {
        Self {
            stopped: false,
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
        if addr == 666 {
            return Err(());
        }
        Ok(addr as usize % 256)
    }
    fn step(
        &mut self,
        mem: &mut [u32; 256],
        program_index: u32,
        awaiting_teleport: &mut Vec<u32>,
    ) -> OpResult<()> {
        let instr = mem[self.pc as usize];

        let opcode = (instr & 0xFF) as u8;
        let immediate = (instr >> 8) & 0xFFFF;
        // let zero = (instr >> 24) & 0xFF;

        // if zero != zero {
        //     return Err(());
        // }

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
                mem[addr] = val;
            }
            0x09 /* ADD */  => {
                let a = self.stack_pop()?;
                let b = self.stack_pop()?;
                let c = a.wrapping_add(b);
                self.stack_push(c)?;
            }
            0x0a /* SUB */ => {
                let a = self.stack_pop()?;
                let b = self.stack_pop()?;
                let c = a.wrapping_sub(b);
                self.stack_push(c)?;
            }
            0x0b /* DIV */ => {
                let a = self.stack_pop()?;
                let b = self.stack_pop()?;
                if b == 0 {
                    return Err(());
                }
                let c = a.wrapping_div(b);
                self.stack_push(c)?;
            }
            0x0c /* POW */ => {
                let a = self.stack_pop()?;
                let b = self.stack_pop()?;
                // rust 0.pow(0) is 1
                let c = a.wrapping_pow(b);
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
                self.pc = immediate as u8;
            }
            0x12 /* ARMED_BOMB */ => {
                return Err(());
            }
            0x13 /* BOMB */ => {
                mem[self.pc as usize] = 0x12; /* ARMED_BOMB */
            }
            0x14 /* TLPORT */ => {
                awaiting_teleport.push(program_index);
                // teleport instructions exit early and don't increment pc, then wait for more teleports to be called
                self.stopped = true;
                return Ok(());
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
        self.pc = self.pc.wrapping_add(1);

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

    let disassemble_query = std::env::args().nth(1).map(|s| s.parse::<usize>().unwrap());

    for q_i in 0..Q {
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
            let dst = &mut mem[offset..(offset + n)];
            dst.fill_with(|| split.next().unwrap().parse::<u32>().unwrap());

            // if Some(q_i) == disassemble_query {
            //     eprintln!(" Dissasembly {}:{}", offset, n);
            //     for val in dst {
            //         let string = disasm(*val);
            //         eprintln!("  {}", string);
            //     }
            // }
        }

        let mut dead = 0;
        let mut awaiting_teleport = Vec::new();
        for _ in 0..5000 {
            if dead == processes.len() {
                break;
            }

            for (i, proc) in processes.iter_mut().enumerate() {
                if proc.stopped {
                    continue;
                }

                // mem at 0 is always zero
                mem[0] = 0;

                match proc.step(mem, i as u32, &mut awaiting_teleport) {
                    Ok(_) => (),
                    Err(_) => {
                        proc.stopped = true;
                        let disasm = disasm(mem[proc.pc as usize]);
                        // eprintln!(" !Fault {}; Pc {} '{}'", i, proc.pc, disasm);
                        dead += 1;
                    }
                }
            }

            if awaiting_teleport.len() >= 2 {
                awaiting_teleport.sort_unstable();

                let first_pc = processes[awaiting_teleport[0] as usize].pc;

                for i in 0..(awaiting_teleport.len() - 1) {
                    let dst = awaiting_teleport[i] as usize;
                    let src = awaiting_teleport[i + 1] as usize;

                    processes[dst].pc = processes[src].pc.wrapping_add(1);
                    processes[dst].stopped = false;
                }

                let last = *awaiting_teleport.last().unwrap() as usize;
                processes[last].pc = first_pc.wrapping_add(1);
                processes[last].stopped = false;

                awaiting_teleport.clear();
            }
        }

        let the_answer = mem[42];
        let b = processes.iter().map(|p| p.pc as u32).sum::<u32>();

        println!("{} {}", the_answer, b);
    }
}

fn disasm(instr: u32) -> String {
    let opcode = instr & 0xFF;
    let immediate = (instr >> 8) & 0xFFFF;

    macro_rules! gen_disasm {
        ($($ins:literal $body:literal $($imm:ident)*;)+) => {
            match opcode {
                $(
                    $ins => gen_disasm!(@format $body $($imm)*),
                )+
                _ => "Invalid".to_string()
            }
        };
        (@format $simple:literal) => {
            $simple.to_string()
        };
        (@format $simple:literal imm) => {
            format!("{} {}", $simple, immediate)
        };
    }

    gen_disasm!(
        0x00 "NOP";
        0x01 "PC";
        0x02 "PUSH" imm;
        0x03 "POP";
        0x04 "SWAP";
        0x05 "DUP";
        0x06 "PUSHSSZ";
        0x07 "LOAD";
        0x08 "STORE";
        0x09 "ADD";
        0x0a "SUB";
        0x0b "DIV";
        0x0c "POW";
        0x0d "BRZ" imm;
        0x0e "BR3" imm;
        0x0f "BR7" imm;
        0x10 "BRGE" imm;
        0x11 "JMP" imm;
        0x12 "ARMED_BOMB";
        0x13 "BOMB";
        0x14 "TLPORT";
        0x15 "JNTAR";
    )
}
