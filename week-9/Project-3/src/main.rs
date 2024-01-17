use std::collections::HashMap;

struct Minister {
    name: String,
    commissioner: String,
    ministry: String,
    geopolitical_zone: String,
}

fn main() {
    // Separate datasets
    let commissioner_data = vec![
        ("Aigbogun Alamba Daudu", "Internal Affairs"),
        ("Murtala Afeez Bendy", "Justice"),
        ("Okorocha Calistus Ogbona", "Defense"),
        ("Adewale Jimeb Akanbi", "Power & Steel"),
        ("Osazuwa Faith Etieve", "Petroleum"),
    ];

    let zone_data = vec![
        ("South West", 1),
        ("North East", 2),
        ("South South", 3),
        ("South West", 4),
        ("South East", 5),
    ];

    // Merge datasets
    let mut merged_data: Vec<Minister> = Vec::new();
    let mut commissioner_map: HashMap<&str, &str> = HashMap::new();

    for (name, commissioner) in commissioner_data {
        commissioner_map.insert(name, commissioner);
    }

    for (zone, s_no) in zone_data {
        if let Some((name, commissioner)) = commissioner_map.remove_entry(zone) {
            let ministry = commissioner; // Assuming ministry is the same as commissioner
            let geopolitical_zone = zone.to_string();
            merged_data.push(Minister {
                name: name.to_string(),
                commissioner: commissioner.to_string(),
                ministry: ministry.to_string(),
                geopolitical_zone,
            });
        } else {
            eprintln!("Data not found for geopolitical zone: {}", zone);
        }
    }

    // Display merged data
    println!("{:<4} {:<20} {:<20} {:<20} {:<10}", "S/N", "NAME", "COMMISSIONER", "MINISTRY", "GEOZONE");
    for (index, minister) in merged_data.iter().enumerate() {
        println!(
            "{:<4} {:<20} {:<20} {:<20} {:<10}",
            index + 1,
            minister.name,
            minister.commissioner,
            minister.ministry,
            minister.geopolitical_zone
        );
    }
}
