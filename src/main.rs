use rand::Rng; // 0.8.0

struct Guess {
    num1: i32,
    num2: i32,
}

impl Guess {
    fn get_answer(&self) -> i32 {
        self.num1 * self.num2
    }
}

fn get_input(prompt: &str) -> i32 {
    let mut line = String::new();
    println!("{}", prompt);
    std::io::stdin().read_line(&mut line).unwrap();

    //     let int_value = line.trim_end().parse::<i32>().unwrap(); // Skall göras säkrare
    let mut int_value: i32 = -1;
    let result_value = line.trim_end().parse::<i32>();
    match result_value {
        Err(_) => println!("Du måste ange en siffra!!"),
        Ok(num) => int_value = num,
    }
    int_value
}

fn make_random(start: i32, stop: i32) -> Guess {
	// Vi borde kasta om num1 och num2 ibland för mera random 
    let result = Guess {
        num1: rand::thread_rng().gen_range(start..stop+1),
        num2: rand::thread_rng().gen_range(0..9),
    };
    result
}

fn main() {
    println!("welcome to MathQuest v 1.0");
        let start = get_input("Från tabell");
        let stop = get_input("Till tabell");
        println!("Från tabell {} till tabell {}", start, stop);
    loop {
        let test = make_random(start,stop);
        println!("------------------------------------------------\nFråga:");
        println!("{} x {} = ?", test.num1, test.num2);
        let g = get_input("Ge mig ett svar");
        let correct: i32 = test.get_answer();
        if g == -1 {
            println!("MathQuest v 1.0\nHej då!!");
            break;
        }
        println!("{} x {} = {}", test.num1, test.num2, correct);
        if g == correct {
            println!("RÄTT !!!\n");
        } else {
            println!("FEL !!!\n");
        }
    }
}
