struct Process {
    dead: bool,
    pc: u8,
    lifo: [u32; 16],
    lifo_len: u8,
}

impl Process {
    fn new(offset: u8) -> Self {
        Self {
            dead: false,
            pc: offset,
            lifo: [0; 16],
            lifo_len: 0,
        }
    }
    fn die(&mut self, err: &'static str) {
        self.dead = true;
        eprintln!(err)
    }
}

fn stdin_line() -> String {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    string
}

fn main() {
    let line = stdin_line();
    let mut split = line.split_whitespace();
    
    let Q = split.next().unwrap().parse::<usize>().unwrap();

    let mut processes = Vec::new();
    let mut mem = {
        static mut MEM_BUF: [u32; 256] = [0; 256];
        unsafe {
            &mut MEM_BUF
        }
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
            mem[offset..(offset+n)].fill_with(|| split.next().unwrap().parse::<u32>().unwrap());
        }

        for _ in 0..5000 {
            for proc in &mut processes {
                if proc.dead {
                    continue;
                }

                let instr = mem[proc.pc as usize];

                let opcode = (instr >> 24) & 0xFF;
                let oparg = (instr >> 8) & 0xFFFF;

                macro_rules! die {
                    ($err:literal) => {
                        eprintln!($err);
                        proc.dead = true;
                        continue;
                    }
                }      
                
                match opcode {
                    0x00 /* NOP */  => (),
                    0x0b /* DIV */ => {
                        
                    }
                    0x01 /* PC */  => {
    
                    }
                    0x0c /* POW */ => {
    
                    }
                    0x02 /* PUSH */  => {
    
                    }
                    0x0d /* BRZ */ => {
    
                    }
                    0x03 /* POP */  => {
    
                    }
                    0x0e /* BR3 */ => {
    
                    }
                    0x04 /* SWAP */  => {
    
                    }
                    0x0f /* BR7 */ => {
    
                    }
                    0x05 /* DUP */  => {
    
                    }
                    0x10 /* BRGE */ => {
    
                    }
                    0x06 /* PUSHSSZ */  => {
    
                    }
                    0x11 /* JMP */ => {
    
                    }
                    0x07 /* LOAD */  => {
    
                    }
                    0x12 /* ARMED_BOMB */ => {
    
                    }
                    0x08 /* STORE */  => {
    
                    }
                    0x13 /* BOMB */ => {
    
                    }
                    0x09 /* ADD */  => {
    
                    }
                    0x14 /* TLPORT */ => {
    
                    }
                    0x0a /* SUB */  => {
    
                    }
                    0x15 /* JNTAR */ => {
    
                    }
                }

                proc.pc.wrapping_add(1);
            }
        }
    }
}