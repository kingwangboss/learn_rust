use std::{thread, time::Duration};

fn main() {
    

}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}

fn test1() {
    fn add_one_v1(x: u32) -> u32 {
        x + 1        
    }

    let add_one_v2 = |x: u32| -> u32 {
        x + 1
    };

    let add_one_v3 = |x| { x + 1};
    add_one_v3(1);

    let add_one_v4 = |x| x + 1;
    add_one_v4(1);
}

// 使用带有泛型和Fn trait的闭包
fn test2() {
    struct Cacher<T> 
        where T: Fn(u32) -> u32
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T> 
        where T: Fn(u32) -> u32    
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }
        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }
}

fn test3() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    // println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}