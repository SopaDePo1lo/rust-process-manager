use plotters::prelude::*;
pub use crate::processes::{Manager, Algorithm};

pub struct Plot {
    x_length : u32,
    y_length : u32,
}

impl Plot {
    pub fn _create(x: u32, y: u32) -> Plot {
        Plot {
            x_length : x,
            y_length : y,
        }
    }
    
    
    pub fn _export_fig(&mut self, mut p_manager : Manager, rr: bool)
    {
        let _root_drawing_area = BitMapBackend::new("images/plot.png", (1024, self.y_length*51))
        .into_drawing_area();
        
        _root_drawing_area.fill(&WHITE).unwrap();
        
        let mut chart = ChartBuilder::on(&_root_drawing_area)
            .caption("Procces manager algorithm display", ("TimesNewRoman", 30))
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .build_cartesian_2d(0..self.x_length, 0..self.y_length) //creates a plot with a x and y range
            .unwrap();
        
        chart.configure_mesh().draw().unwrap();
        if !rr {
            for process in p_manager.processes.iter() {
                let start: u32 = process.wait_time.try_into().unwrap();
                let end: u32 = process.run_time as u32 + start + 1;
                let _err = chart.draw_series(
                    LineSeries::new((start..end).map(|x| (x, process.p_id as u32)), BLACK.stroke_width(20))
            ).unwrap();
            }   
        } else {
            let _ = match p_manager.algorithm {
                Algorithm::FCFS => {
                    p_manager._set_sort_id();
                }
                Algorithm::SJF => {
                    p_manager._set_sort_run_time();
                }
                _ => {}
            };
            let size : usize = p_manager._get_amount();
            let quant : u32 = p_manager._get_quant(); 
            let mut time_passed = vec![0; size];
            let mut lines = vec![String::new(); size];
            let mut completed: u32 = 0;
            let mut passed: u32 = 0;
            while completed < size as u32 + 10 {
                for (_, process) in p_manager.processes.iter().enumerate() {
                    if (process.run_time - time_passed[process.p_id]) >= quant {
                        let start: u32 = passed;
                        let end : u32 = start + quant + 1;
                        lines[process.p_id] += &"-".repeat(start as usize);
                        lines[process.p_id] += &"+".repeat(end as usize - 1);
                        let _err = chart.draw_series(
                            LineSeries::new((start..end).map(|x| (x, process.p_id as u32)), BLACK.stroke_width(20))
                        ).unwrap();
                        passed += quant;
                        time_passed[process.p_id] += quant;
                    } else {
                        let start: u32 = passed;
                        let end : u32 = process.run_time - time_passed[process.p_id] + passed + 1;
                        lines[process.p_id] += &"-".repeat(start as usize);
                        lines[process.p_id] += &"+".repeat(end as usize - 1);
                        passed += process.run_time - time_passed[process.p_id];
                        let _err = chart.draw_series(
                            LineSeries::new((start..end).map(|x| (x, process.p_id as u32)), BLACK.stroke_width(20))
                        ).unwrap();
                        time_passed[process.p_id] += process.run_time - time_passed[process.p_id];
                        if time_passed[process.p_id] == process.run_time {
                            completed += 1;
                            continue;
                        }
                    }
                }
            }
        }
        
    }
    
}

