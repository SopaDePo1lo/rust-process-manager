use rand::Rng;
use std::{fmt, fs::File, process};
// use csv::WriterBuilder;

// #[derive(Debug)]
struct Manager {
    processes: Vec<Process>,
    amount: usize,
    algorithm: Algorithm,
    total: u32,
    quant: u32,
}

enum Algorithm {
    FCFS,
    NONE,
    SJF,
    PRIORITY,
}

impl Manager {
    fn _create(amount: usize) -> Manager {
        let mut processes: Vec<Process> = Vec::new();
        let mut total: u32 = 0;
        for _i in 0..amount {
            let process = Process::_create(_i);
            total += process.run_time;
            processes.push(process);
        }
        Manager {
            processes,
            amount,
            algorithm: Algorithm::FCFS,
            total,
            quant: 4,
        }
    }

    fn _load_config(&mut self, file_path: String) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(file_path)?;
        let mut _reader = csv::Reader::from_reader(file);
        self.processes = Vec::new();
        self.amount = 0;
        self.total = 0;
        for result in _reader.records() {
            let record = result?;
            let mut params: Vec<i32> = vec![0, 0, 0];
            for i in 0..=2 {
                params[i] = String::from(record.get(i).unwrap()).parse()?;
            }
            // println!("{:?}", record);
            let process: Process = Process {
                run_time: params[1] as u32,
                wait_time: 0,
                p_id: params[0] as usize,
                priority: params[2],
            };
            self.processes.push(process);
            self.amount += 1;
            self.total += params[1] as u32;
        }
        Ok(())
    }

    fn _fcfs_sjf(&mut self) {
        for process in self.processes.iter() {
            let mut line: String = String::from("");
            let before: usize = process.wait_time.try_into().unwrap();
            let after: usize = (self.total - process.wait_time - process.run_time)
                .try_into()
                .unwrap();
            line += &String::from("-").repeat(before);
            line += &String::from("+").repeat(process.run_time.try_into().unwrap());
            line += &String::from("-").repeat(after);
            println!("{}", line);
        }
        println!();
    }
    fn _display(&mut self, rr: bool) {
        self.calculate_wait_time();
        if rr {
            let _ = match self.algorithm {
                Algorithm::FCFS => {
                    self.processes.sort_by_key(|s| s.p_id);
                    self.rr();
                }
                Algorithm::SJF => {
                    self.processes.sort_by_key(|s| s.run_time);
                    self.rr();
                }
                Algorithm::NONE => {
                    for process in self.processes.iter() {
                        println!("{}", process);
                    }
                }
                Algorithm::PRIORITY => {
                    self.rr();
                }
            };
        } else {
            let _ = match self.algorithm {
                Algorithm::FCFS => {
                    self._fcfs_sjf();
                }
                Algorithm::SJF => {
                    self._fcfs_sjf();
                }
                Algorithm::NONE => {
                    for process in self.processes.iter() {
                        println!("{}", process);
                    }
                }
                Algorithm::PRIORITY => {
                    self._fcfs_sjf();
                }
            };
        }
    }
    fn rr(&mut self) -> () {
        let mut time_passed = vec![0; self.amount];
        let mut lines = vec![String::new(); self.amount];
        let mut completed: u32 = 0;
        let mut passed: u32 = 0;
        while completed < self.amount as u32 + 10{
            for (i, process) in self.processes.iter().enumerate() {
                let length: usize = lines[i].len();
                if (process.run_time - time_passed[i]) >= self.quant {
                    lines[i] += &"-".repeat((passed as usize) - length);
                    lines[i] += &"+".repeat(self.quant as usize);
                    passed += self.quant;
                    time_passed[i] += self.quant;
                } else {
                    lines[i] += &"-".repeat((passed as usize) - length);
                    lines[i] += &"+".repeat((process.run_time - time_passed[i]) as usize);
                    passed += process.run_time - time_passed[i];
                    time_passed[i] += process.run_time - time_passed[i];
                    if time_passed[i] == process.run_time {
                        completed += 1;
                        continue;
                    }
                }
            }
        }
        for line in lines.iter() {
            println!("{}{}", line, "-".repeat(self.total as usize - line.len()));
        }
    }

    fn calculate_wait_time(&mut self) {
        let _ = match self.algorithm {
            Algorithm::FCFS => {
                let mut total_wait_time: u32 = 0;
                for process in self.processes.iter_mut() {
                    process.wait_time = total_wait_time;
                    total_wait_time += process.run_time;
                }
            }

            Algorithm::SJF => {
                self.processes.sort_by_key(|s| s.run_time);
                let mut total_wait_time: u32 = 0;
                for process in self.processes.iter_mut() {
                    process.wait_time = total_wait_time;
                    total_wait_time += process.run_time;
                }
                self.processes.sort_by_key(|s| s.p_id);
            }
            Algorithm::NONE => {}
            Algorithm::PRIORITY => {
                self.processes.sort_by_key(|s| s.priority);
                let mut total_wait_time: u32 = 0;
                for process in self.processes.iter_mut() {
                    process.wait_time = total_wait_time;
                    total_wait_time += process.run_time;
                }
                self.processes.sort_by_key(|s| s.p_id);
            }
        };
    }
}

struct Process {
    run_time: u32,
    wait_time: u32,
    p_id: usize,
    priority: i32,
}

impl Process {
    fn _create(p_id: usize) -> Process {
        let run_time = rand::thread_rng().gen_range(1..=10);
        let priority = rand::thread_rng().gen_range(-20..=20);

        Process {
            run_time,
            wait_time: 0,
            p_id,
            priority,
        }
    }
}

impl fmt::Display for Manager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.amount)
    }
}
impl fmt::Display for Process {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}",
            self.run_time, self.wait_time, self.priority
        )
    }
}

fn main() {
    let mut manager = Manager::_create(5);
    // println!("{}", manager);
    manager.algorithm = Algorithm::FCFS;
    manager._display(false);
    // manager.load_config(String::from("out.csv")).expect("Иди нахуй у тебя error");
    manager._display(true);
}
