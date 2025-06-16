trait Transport {
    fn deliver(&self);
}

struct Truck;
impl Transport for Truck {
    fn deliver(&self) {
        println!("Delivering by land in a box");
    }
}

struct Ship;
impl Transport for Ship {
    fn deliver(&self) {
        println!("Delivering by sea in a container");
    }
}

trait Logistics {
    fn create_transport(&self) -> Box<dyn Transport>;
    
    fn plan_delivery(&self) {
        let transport = self.create_transport();
        println!("Planning delivery...");
        transport.deliver();
    }
}

struct RoadLogistics;
impl Logistics for RoadLogistics {
    fn create_transport(&self) -> Box<dyn Transport> {
        Box::new(Truck)
    }
}

struct SeaLogistics;
impl Logistics for SeaLogistics {
    fn create_transport(&self) -> Box<dyn Transport> {
        Box::new(Ship)
    }
}

fn main() {
    let road_logistics = RoadLogistics;
    road_logistics.plan_delivery();

    let sea_logistics = SeaLogistics;
    sea_logistics.plan_delivery();
}