
fn main() {
    println!("Hello, world!");
    convert_temperature(23., 'C');
    christmas_carol();
}

fn convert_temperature(temperature : f32, unit : char) -> f32{
    if unit == 'C' {
       return (temperature * 1.8) + 32.;
    } else if unit == 'F' {
        return (temperature - 32.) / 1.8; 
    }
    println!("Please enter a valid unit");
    return 0.
}

fn nth_fibonacci(nth : i32) -> i32 {
    let mut sequence:[i32;2]=[0,1];
    for i in 1..sequence.len() {
        sequence[i+1]=sequence[i]+sequence[i-1];
        if sequence.len() as i32 == nth {
            return sequence[i+1]
        }
    }
    return 0 
}

fn christmas_carol() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"]; 
    let individual_gift = [
        "A partridge in a pear tree",
        "Two turtle doves ", 
        "Three French hens", 
        "four calling birds", 
        "five gold rings",
        "six geese a-laying", 
        "seven swans a-swimming", 
        "eight maids a-milking", 
        "nine ladies dancing", 
        "ten lords a-leaping", 
        "eleven pipers piping", 
        "twelve drummers drumming",
        ];
    
        let mut total_gifts = vec![];
    for index in 1..13 {
        total_gifts.push(individual_gift[index]);
        let current_gifts = &total_gifts[0..total_gifts.len()-1];      
        let mut reverse_current_gifts = vec![];

        for i in 0..current_gifts.len() {
            reverse_current_gifts.push(current_gifts[(current_gifts.len()-1) - i]);
        }

        if index == 1 {
            println!("On the {} day of Christmas, my true love sent to me {}", index, individual_gift[0])
        } else {
            println!("On the {} day of Christmas, my true love sent to me {} and {}", index, reverse_current_gifts.join(","), total_gifts[0])
        }
    }
}