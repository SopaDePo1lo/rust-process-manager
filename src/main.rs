use rand::Rng;
use std::fmt;


// #[derive(Debug)]
struct Manager {
    processes : Vec<Process>,
    amount : u32,
    algorithm : Algorithm,
    total : u32,
}

enum Algorithm {
    FCFS,
    NONE,
    SJF,
}

impl Manager {
    fn _create(amount : u32) -> Manager {
        let mut processes : Vec<Process> = Vec::new();
        let mut total : u32 = 0;
        for _i in 1..=amount {
            let process = Process::_create();
            processes.push(process);
            total += process.run_time;
        }
        Manager { processes: processes, amount: amount , algorithm: Algorithm::FCFS, total : total}

    }
    fn _display(&mut self, ) {
        self.calculate_wait_time();
        let _ = match self.algorithm {
            Algorithm::FCFS => {
                for process in self.processes.iter() {
                    let mut line : String = String::from("");
                    let before : usize = process.wait_time.try_into().unwrap();
                    let after : usize = (self.total-process.wait_time-process.run_time).try_into().unwrap();
                    line += &String::from("-").repeat(before);
                    line += &String::from("+").repeat(process.run_time.try_into().unwrap());
                    line += &String::from("-").repeat(after);
                    println!("{}", line);
                }
            },

            Algorithm::SJF => {

            },
            Algorithm::NONE => {
                for process in self.processes.iter() {
                    println!("{}", process);
                }
            }
            
        };
    }
    fn calculate_wait_time(&mut self) {
        let _ = match self.algorithm {
            Algorithm::FCFS => {
                let mut total_wait_time : u32 = 0;
                for process in self.processes.iter_mut() {
                    process.wait_time = total_wait_time;
                    total_wait_time += process.run_time;
                }
            },

            Algorithm::SJF => {

            },
            Algorithm::NONE => {

            }
            
        };
    }
}
#[derive(Copy, Clone)]
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
    let mut manager = Manager::_create(5);
    println!("{}", manager);
    manager._display();
    manager.algorithm = Algorithm::NONE;
    manager._display();
    // manager.calculate_wait_time();
    // manager._display()
}
