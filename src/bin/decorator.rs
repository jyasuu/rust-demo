trait Coffee {
    fn cost(&self) -> f64;
    fn description(&self) -> String;
}

struct SimpleCoffee;
impl Coffee for SimpleCoffee {
    fn cost(&self) -> f64 {
        2.0
    }

    fn description(&self) -> String {
        "Simple Coffee".to_string()
    }
}

struct CoffeeDecorator {
    coffee: Box<dyn Coffee>,
}

impl Coffee for CoffeeDecorator {
    fn cost(&self) -> f64 {
        self.coffee.cost()
    }

    fn description(&self) -> String {
        self.coffee.description()
    }
}

struct MilkDecorator {
    decorator: Box<dyn Coffee>,
}

impl Coffee for MilkDecorator {
    fn cost(&self) -> f64 {
        self.decorator.cost() + 0.5
    }

    fn description(&self) -> String {
        format!("{}, Milk", self.decorator.description())
    }
}

struct WhipDecorator {
    decorator: Box<dyn Coffee>,
}

impl Coffee for WhipDecorator {
    fn cost(&self) -> f64 {
        self.decorator.cost() + 0.7
    }

    fn description(&self) -> String {
        format!("{}, Whip", self.decorator.description())
    }
}

fn main() {
    let coffee: Box<dyn Coffee> = Box::new(SimpleCoffee);
    println!("{}: ${:.2}", coffee.description(), coffee.cost());

    let milk_coffee = MilkDecorator { decorator: coffee };
    println!("{}: ${:.2}", milk_coffee.description(), milk_coffee.cost());

    let whip_milk_coffee = WhipDecorator { decorator: Box::new(milk_coffee) };
    println!("{}: ${:.2}", whip_milk_coffee.description(), whip_milk_coffee.cost());
}