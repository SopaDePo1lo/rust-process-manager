mod processes;
pub mod plot;

pub use crate::processes::{Manager, Algorithm};
pub use crate::plot::{Plot};
use std::{io::{self, Write}};

use iced::widget::{column, pick_list, scrollable, vertical_space, row, checkbox, Image, image, button};
use iced::{Alignment, Element, Length, Sandbox, Settings};

pub fn main() -> iced::Result {
    Example::run(Settings::default())
}

#[derive(Default)]
struct Example {
    selected_algorithm: Option<Algorithm>,
    rr_default: bool,
    // plot: Image<image::Handle>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    AlgorithmSelected(Algorithm),
    RR(bool),
    ButtonPressed,
}

impl Sandbox for Example {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Rust process manager")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::AlgorithmSelected(algorithm) => {
                self.selected_algorithm = Some(algorithm);
            }
            Message::RR(default) => {
                self.rr_default = default;
            }
            Message::ButtonPressed => {
                let mut manager = Manager::_create(1, 4);
                manager._set_algorithm(Algorithm::FCFS);
                manager
                    ._load_config(String::from("out.csv"))
                    .expect("Error loading file");
                manager._set_algorithm(self.selected_algorithm.unwrap());
                let mut graph : Plot = manager._parse_plot_info();
                graph._export_fig(manager, self.rr_default);
                let plot = Image::<image::Handle>::new("images/plot.png");

            }
        }
    }
    
    fn view(&self) -> Element<Message> {
        
        let default_checkbox = checkbox("", self.rr_default)
            .on_toggle(Message::RR);
        
        let pick_list = pick_list(
            &Algorithm::ALL[..],
            self.selected_algorithm,
            Message::AlgorithmSelected,
        )
        .placeholder("Choose algorithm...");
        
        let select_algorithm = row!(
            "Choose planning algorithm",
            pick_list,
            ).spacing(20);
        
        let checkboxes = row![
            "Enable RoundRobin algorithm",
            default_checkbox,
        ].spacing(10);
        
        let content = column![
            vertical_space().height(10),
            select_algorithm,
            vertical_space().height(10),
            checkboxes,
            vertical_space().height(10),
            button("Plot").padding(8).on_press(Message::ButtonPressed),
        ].padding(20)
        .width(Length::Fill)
        .align_items(Alignment::Start)
        .spacing(10);

        scrollable(content).into()
    }
}


fn _get_input(message: &str) -> u32 {
    print!("{}: ", message);
    io::stdout().flush().unwrap();
    let mut result: String = String::new();
    io::stdin()
        .read_line(&mut result)
        .expect("Failed to read input");

    result.trim().parse().expect("Input not an integer")
}