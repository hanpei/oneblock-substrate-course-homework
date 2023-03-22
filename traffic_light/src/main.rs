enum TrafficLight {
    Red,
    Green,
    Yellow,
}
trait TrafficLightTime {
    fn time(&self) -> u8;
}

impl TrafficLightTime for TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Green => 70,
            TrafficLight::Yellow => 20,
        }
    }
}

fn main() {
    let red_time = TrafficLight::Red.time();
    let green_time = TrafficLight::Green.time();
    let yellow_time = TrafficLight::Yellow.time();
    println!("Red light time: {}", red_time);
    println!("Green light time: {}", green_time);
    println!("Yellow light time: {}", yellow_time);
}
