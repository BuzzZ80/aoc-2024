struct Vm {
    reg: [i64; 3],
    inst_ptr: usize,
}

pub fn run(input: &String) {
    println!("--- Day 17: Chronospatial Computer ---");

    let mut numbers = input
        .split(|c: char| !c.is_numeric())
        .filter_map(|s| s.parse::<i64>().ok());

    let mut vm = Vm {
        reg: [
            numbers.next().expect("Wrong input?"),
            numbers.next().expect("Wrong input?"),
            numbers.next().expect("Wrong input?"),
        ],
        inst_ptr: 0,
    };

    let program: Vec<i8> = numbers.map(|n| n as i8).collect();

    print!("Star 1 (ignore last comma): ");
    while vm.inst_ptr < program.len() - 1 {
        vm.exec(program[vm.inst_ptr], program[vm.inst_ptr + 1]);
    }
    println!();
}

impl Vm {
    fn exec(&mut self, op: i8, data: i8) {
        let combo = match data {
            n if (0..=3).contains(&n) => n as i64,
            n if (4..=6).contains(&n) => self.reg[n as usize - 4],
            _ => 0,
        };
        match op {
            0 => self.reg[0] = self.reg[0] >> combo,
            1 => self.reg[1] = self.reg[1] ^ data as i64,
            2 => self.reg[1] = combo & 0x7,
            3 => {
                if self.reg[0] != 0 {
                    self.inst_ptr = data as usize;
                } else {
                    self.inst_ptr += 2;
                }
            }
            4 => self.reg[1] = self.reg[1] ^ self.reg[2],
            5 => print!("{},", combo & 0x7),
            6 => self.reg[1] = self.reg[0] >> combo,
            7 => self.reg[2] = self.reg[0] >> combo,
            _ => panic!("Wrong input"),
        }
        if op != 3 {
            self.inst_ptr += 2;
        }
    }
}
