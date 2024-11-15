enum FavouriteBackgroundVideo {
    SubwaySurfers,
    MinecraftParkour,
    SandCutting,
}

enum BestGameAndWhy {
    Minecraft(String),
    Robocks(String),
    HelloEverybodyMyNameIsMarkiplierAndWelcomeBackToFiveNightsAtFreddys(String),
    GorbinosQuest(bool),
    John,
}

fn enums() {
    let bad_opinion = FavouriteBackgroundVideo::MinecraftParkour;

    let good_opinion = BestGameAndWhy::GorbinosQuest(true);

    // note to self: after this, explain Option<T>, no null value in rust etc

    match good_opinion {
        BestGameAndWhy::John => println!("John"),
        BestGameAndWhy::GorbinosQuest(yes) => println!("{yes}: 500 hours of mind-pumping action!"),
        BestGameAndWhy::Robocks(s) => println!("In life, there is Roblox. {s}"),
        _ => println!("FUCK YOU SPECIFICALLY"),
    }

    let _ = match Some(32) {
        Some(x) => x,
        None => 0,
    };

    if let Some(thing) = Some(32) {
        println!("this is a {thing}");
    }

    // go over vectors/hashmaps really quickly after this
}
