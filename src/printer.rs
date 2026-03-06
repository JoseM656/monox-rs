pub enum MonoxEvent {
    Checking(String),
    Launching(String),
    Done,
    Merror(String),
}

// This allow to print with the same sintaxis.
pub fn print_event(event: MonoxEvent) {
    match event {
        MonoxEvent::Checking(msg) => println!("  => {}", msg),
        MonoxEvent::Launching(msg) => println!("  ===> {}", msg),
        MonoxEvent::Done => println!("  ...done"),
        MonoxEvent::Merror(msg) => eprintln!("  ✗ {}", msg),
    }
}

#[macro_export]
macro_rules! checking {
    ($($arg:tt)*) => {
        $crate::printer::print_event($crate::printer::MonoxEvent::Checking(format!($($arg)*)))
    };
}

#[macro_export]
macro_rules! launching {
    ($($arg:tt)*) => {
        $crate::printer::print_event($crate::printer::MonoxEvent::Launching(format!($($arg)*)))
    };
}

#[macro_export]
macro_rules! done {
    () => {
        $crate::printer::print_event($crate::printer::MonoxEvent::Done)
    };
}

#[macro_export]
macro_rules! merror {
    ($($arg:tt)*) => {
        $crate::printer::print_event($crate::printer::MonoxEvent::Merror(format!($($arg)*)))
    };
}
