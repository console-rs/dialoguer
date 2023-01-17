fn main() {
    #[cfg(not(feature = "fuzzy-select"))]
    println!("feature \"fuzzy-select\" not enabled");
    #[cfg(feature = "fuzzy-select")]
    {
        use dialoguer::theme::ColorfulTheme;
        use dialoguer::MultiFuzzySelect;

        let multiselected = &[
            "Ice Cream",
            "Vanilla Cupcake",
            "Chocolate Muffin",
            "A Pile of sweet, sweet mustard",
            "Carrots",
            "Peas",
            "Pistacio",
            "Mustard",
            "Cream",
            "Banana",
            "Chocolate",
            "Flakes",
            "Corn",
            "Cake",
            "Tarte",
            "Cheddar",
            "Vanilla",
            "Hazelnut",
            "Flour",
            "Sugar",
            "Salt",
            "Potato",
            "French Fries",
            "Pizza",
            "Mousse au chocolat",
            "Brown sugar",
            "Blueberry",
            "Burger",
        ];

        let defaults = &[false, false, true, false];
        let selections = MultiFuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick your food")
            .items(&multiselected[..])
            .defaults(&defaults[..])
            .max_length(10)
            .interact()
            .unwrap();
        if selections.is_empty() {
            println!("You did not select anything :(");
        } else {
            println!("You selected these things:");
            for selection in selections {
                println!("\t{}", multiselected[selection]);
            }
        }
    }
}
