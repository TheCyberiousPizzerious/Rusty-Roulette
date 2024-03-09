use rand::Rng;
use std::io::{self, Write};

fn main() {
    let mut revolver = Revolver::new();
    revolver.load();
    for x in revolver.cylinder {
        println!("{}", x);
    }

    let mut player = Target::new();
    let mut hat = Target::new_hat();

    let result = revolver.shoot(&mut hat);

    println!("{}", result);
    
    let gameloop = loop {
        print!("
        / Options
        / 1 shoot yourself
        / 2 shoot the hat
        / 3 reload the cylinder
        
        Choose: ");
        io::stdout().flush().unwrap(); // unwrap returns the result of a Option or Result type

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line, L");

        let intput: u8 = input.trim().parse()
            .expect("Please enter a number");

        let x = match intput {
            1 => revolver.shoot(&mut player),
            2 => revolver.shoot(&mut hat),
            3 => String::from("Njææææ This aint here yet (it will never be)"),
            //3 => revolver.load(),
            _ => String::from("Not an option, jackass"),
        };

        println!("{}", x);
        if player.alive_check() == false {
            println!("You are dead");
            break
        } else {
            continue
        }
    };
}

struct Target {
    hat: bool,
    alive: bool,
}

struct Revolver {
    cylinder: [bool; 6],   
    chamber: usize,
}

trait Creation {
    fn new() -> Self;
}

impl Target {
    pub fn hit_person(&mut self) -> bool {
        self.alive = false;
        self.alive
    }
    pub fn hit_hat() -> String {
        String::from("")
    }
    pub fn alive_check(&self) -> bool {
        if self.alive == true {
            true
        } else {
            false
        }
    }

    fn new_hat() -> Target {
        Target {
            hat: true,
            alive: false,
        }
    }
}

impl Revolver {
    pub fn load(&mut self) -> [bool; 6] {
        let mut rng = rand::thread_rng();
        self.cylinder[rng.gen_range(0..self.cylinder.len())] = true; //Maybe rand.gen() or something
        
        self.cylinder
    }
    fn shoot(&mut self, target: &mut Target) -> String {
        if target.hat == false {
            if self.cylinder[self.chamber] == true {
                self.cylinder[self.chamber] = false;
                self.next_chamber();
                target.alive = false;
                String::from("Chamber was loaded fired at player. You just shot yourself in the head...")
            } else {
                self.next_chamber();
                String::from("The chamber is empty fired at player")
            }
        } else {
            if self.cylinder[self.chamber] == true {
                self.cylinder[self.chamber] = false;
                self.next_chamber();

                String::from("The chamber was loaded fired at hat. A shot rings out and the room fills with smoke")
            } else {
                String::from("The chamber is empty fired at hat")
            }
        }
    }
    fn next_chamber(&mut self) {
        if self.chamber == self.cylinder.len()-1 {
            self.chamber = 0;
        } else {
            self.chamber += 1;
        };
    }
}

impl Creation for Revolver {
    fn new() -> Revolver {
        Revolver {
            cylinder: [false; 6],
            chamber: 0,
        }
    }
}

impl Creation for Target {
    fn new() -> Target {
        Target {
            hat: false,
            alive: true,
        }
    }
}