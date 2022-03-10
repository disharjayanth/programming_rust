// Two possible alternatives for what a 'Brown' could be working on.
#[derive(Copy, Clone, Debug)]
enum BroomIntent {
    FetchWater,
    DumpWater,
}

#[derive(Debug)]
struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent,
}

fn chop(b: Broom) -> (Broom, Broom) {
    // Intialize `broom1` mostly from 'b', changing only 'height'. Since
    // 'String' is not 'Copy', 'broom1' take ownership of 'b''s name.
    let mut broom1 = Broom {
        health: b.health / 2,
        ..b
    };

    // Intialize 'broom2' mostly from 'broom1'. Since String is not
    // 'Copy', we must clone 'name' explicitly.
    let mut broom2 = Broom {
        name: broom1.name.clone(),
        ..broom1
    };

    broom1.name.push_str(" I");
    broom2.name.push_str("  II");

    (broom1, broom2)
}

fn main() {
    let broom = Broom {
        name: "Broom".to_string(),
        height: 12,
        health: 100,
        position: (1.1, 2.2, 3.3),
        intent: BroomIntent::FetchWater,
    };

    let (broom_1, broom_2) = chop(broom);

    println!("{:?}", broom_1);
    println!("{:?}", broom_2);
}
