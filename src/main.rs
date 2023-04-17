use rand::Rng;
use std::fmt;


// #[derive(Debug)]
struct Manager {
    processes : Vec<Process>,
    amount : u32,
}

impl Manager {
    fn _create(amount : u32) -> Manager {
        let mut processes : Vec<Process> = Vec::new();
        for _i in 1..=amount {
            let process = Process::_create();
            processes.push(process);
        }
        Manager { processes: processes, amount: amount }

    }
}

struct Process {
    run_time : u32,
    wait_time : u32,
}

impl Process {
    fn _create() -> Process {
        let run_time = rand::thread_rng().gen_range(1..=10);

        Process { run_time: run_time, wait_time: 0 }
    }
}

impl fmt::Display for Manager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.amount)
    }
}
impl fmt::Display for Process {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.run_time, self.wait_time)
    }
}

fn main() {
    let manager = Manager::_create(5);
    println!("{}", manager)
}
