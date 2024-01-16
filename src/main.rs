use std::thread;
use std::time::Duration;

struct Philosopher {
    name: String,
}
impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }

    fn eat(&self) {
        println!("{} started eating.", self.name);
        thread::sleep(Duration::from_millis(1000));
        println!("{} finished eating.", self.name);
    }
}

fn main() {
    let philosophers = vec![
        Philosopher::new("Aristotle"),
        Philosopher::new("Friedrich Nietzsche"),
        Philosopher::new("Immanuel Kant"),
        Philosopher::new("Socrates"),
        Philosopher::new("Thomas Aquinas"),
        Philosopher::new("David Hume"),
        Philosopher::new("John Locke"),
        Philosopher::new("Plato"),
    ];

    let handles: Vec<_> = philosophers.into_iter().map(|p|{
        thread::spawn(move || {
            p.eat();
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
