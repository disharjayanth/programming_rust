#[derive(Debug)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

#[derive(Debug)]
enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

#[derive(Debug)]
enum Point3d {
    ORIGIN,
}

#[derive(Debug)]
enum Shape {
    Sphere { center: Point3d, radius: f32 },
    Cuboid { corner1: Point3d, corner2: Point3d },
}

#[derive(Debug)]
enum DifferentialEquation {}

#[derive(Debug)]
enum EarlyModernistPoem {}

#[derive(Debug)]
enum RelationshipStatus {
    Single,
    InARelationShip,
    ItsComplicated(Option<String>),
    ItsExtremeComplicated {
        car: DifferentialEquation,
        cdr: EarlyModernistPoem,
    },
}

fn main() {
    let four_score_and_seven_years = RoughTime::InThePast(TimeUnit::Years, 4 * 20 + 7);

    let three_hours_from_now = RoughTime::InTheFuture(TimeUnit::Years, 3);

    println!("{:?}", four_score_and_seven_years);
    println!("{:?}", three_hours_from_now);

    let unit_sphere = Shape::Sphere {
        center: Point3d::ORIGIN,
        radius: 1.0,
    };

    println!("{:?}", unit_sphere);

    let some_relation = RelationshipStatus::Single;
    println!("{:?}", some_relation);

    let some_relation1 = RelationshipStatus::ItsComplicated(Some("I dont know".to_string()));
    let some_relation2 = RelationshipStatus::ItsComplicated(None);
    println!("{:?}", some_relation1);
    println!("{:?}", some_relation2);
}
