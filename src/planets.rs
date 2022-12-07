use std::collections::HashMap;
use std::io;

struct Planet {
    name: String,
    gravity: f32
}

fn calculate_weight(weight: f32, planet: &Planet) -> f32 {
    let weight:f32 = (weight / 9.81) * planet.gravity;
    return weight;
}

fn print_welcome_text() {
    println!("\n");
    println!("Welcome to planet weight calculator! \nWe will calculate how much your \nweight is in any given planet or moon\non our solar system!");
    println!("Choose a planet or a moon. Available planets or moons are:");
    println!("\n");
    println!("\n");
}

pub(crate) fn planets() {
    let entities:HashMap<String, f32> = HashMap::from([
        ("Mercury 🪐".to_string(), 3.7),
        ("Venus 🪐".to_string(), 8.87),
        ("Earth 🌍".to_string(), 9.81),
        ("Mars 🪐".to_string(), 3.721),
        ("Jupiter 🪐".to_string(), 24.79),
        ("Saturn 🪐".to_string(), 10.44),
        ("Uranus 🪐".to_string(), 8.87),
        ("Neptune 🪐".to_string(), 11.15),
        ("Pluto 🪐".to_string(), 0.62),
        ("Moon 🌔".to_string(), 1.62),
        ("Europa 🌔".to_string(), 1.315),
        ("Ganymede 🌔".to_string(), 1.428),
        ("Io 🌔".to_string(), 1.796),
        ("Callisto 🌔".to_string(), 1.236),
        ("Titan 🌔".to_string(), 1.352),
        ("Triton 🌔".to_string(), 0.779),
        ("Charon 🌔".to_string(), 0.288),
        ("Phobos ☄".to_string(), 0.0057),
        ("Deimos  ☄".to_string(), 0.003),
        ("Phoebe ☄".to_string(), 0.049),
        ("Lapetus 🌔".to_string(), 0.223),
        ("Hyperion ☄".to_string(), 0.02),
        ("Rhea 🌔".to_string(), 0.264),
        ("Dione 🌔".to_string(), 0.232),
        ("Tethys 🌔".to_string(), 0.145),
        ("Enceladus 🌔".to_string(), 0.113),
        ("Mimas ☄".to_string(), 0.064),
        ("Nereid ☄".to_string(), 0.071),
        ("Proteus ☄".to_string(), 0.07),
    ]);

    print_welcome_text();

    for (key, value) in &entities {
        println!("--------------------------");
        println!("Planet or moon name: {}", key);
        println!("gravity {}", value);
    }

    let mut chosen_entity: String = String::new();

    while !entities.contains_key(&chosen_entity.trim().to_string()) {
        println!("Give a Planet or a moon: ");
        io::stdin().read_line(&mut chosen_entity).unwrap();

        if !entities.contains_key(&chosen_entity.trim().to_string()) {
            println!("We've got {} planets, but {} is not one of them.", entities.len(), chosen_entity);
            chosen_entity = "".to_string();
        }
    }

    println!("Give your weight:");
    let mut user_weight = String::new();
    io::stdin().read_line(&mut user_weight).unwrap();
    let planet = Planet { name: chosen_entity.trim().parse().unwrap(), gravity: *entities.get(chosen_entity.trim()).unwrap() };

    let weight_in_float:f32 = user_weight.trim().parse::<f32>().unwrap();
    println!("Your weight on planet {}: {:.2}kg", planet.name, calculate_weight(weight_in_float, &planet));
}