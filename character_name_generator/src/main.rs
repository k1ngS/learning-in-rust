use rand::seq::SliceRandom;

fn main() {
    let first_names = ["Ar", "Bel", "Cal", "Dor", "El", "Fen", "Gal", "Hal", "Ith", "Jar"];
    let last_names = ["dor", "wen", "mir", "thas", "gorn", "lith", "dil", "nor", "ras", "vorn"];

    let titles = ["the Brave", "the Wise", "the Swift", "the Bold", "the Cunning", "the Fierce", "the Just", "the Mighty", "the Silent", "the Valiant"];


    let mut rng = rand::thread_rng();

    let first = first_names.choose(&mut rng).unwrap();
    let last = last_names.choose(&mut rng).unwrap();

    let full_name = format!("{} {}", first, last);
    let title = titles.choose(&mut rng).unwrap();

    println!("Your fantasy name is: {} {}!", full_name, title);
}
