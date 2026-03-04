pub enum MonoxEvent {
    Checking(String),
    Launching(String),
    Done,
    Error(String),
}

pub fn print_event(event: MonoxEvent) { 
    match event {
        MonoxEvent::Checking(msg) => println!("  :: {}", msg),
        MonoxEvent::Launching(msg) => println!("  => {}", msg),
        MonoxEvent::Done => println!("  ...done!!!"),
        MonoxEvent::Error(msg) => eprintln!("  ✗ {}", msg),
    }
}