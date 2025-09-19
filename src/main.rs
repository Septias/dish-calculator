use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use regex::Regex;

const BASE: &str = "/home/septias/life/Areas/Kochen";
const PLAN: &str = r#"
# GG Essensplan
## Freitag
- [[Linsendahl]]
	- [[Naan-Brot]](18)
	- [[Naan-Brot2]](2)

## Samstag
- [[Standartfrühstück]](18)
	- [[Rustikales]]
	- Baked beans
	- [[Tofu Rührei]](8)
	- [[Guacamole]](8)
- [[Reis Bowl]]
	- [[Sticky Tofu]](4) SOJA!
	- [[Gebratene Auberginen]](4) SOJA!
	- [[Brokkoli im Teigmantel]](4)
- [[Erdnusssauce]](10) SOJA!
	- [[Avocado Cashew Dressing]](10)
- [[Kürbis Gnocci]]
	- [[Gurkensalat]](10)
	- [[Gemischter Salat]](10)

## Sonntag
- [[Kaiserschmarn]](18)
- [[Kaiserschmarn(Vegan)]](2)
- [[Pizzaschnecken]]"#;
const PARTICIPANTS: f32 = 20.0;

const MEASURES: [&str; 16] = [
    "Dosen", "g", "mg", "kg", "el", "tl", "l", "ml", "Liter", "stk", "Scheiben", "scheiben",
    "scheibe", "Pr.", "EL", "TL",
];

#[derive(Debug, Clone)]
struct Amount {
    amount: f32,
    measure: String,
    name: String,
    dish: String,
}

/// Calculate the ingredients needed for the number of participants and dish.
fn calculate(path: &Path, dish: &str, participants: f32) -> (Vec<Amount>, Option<String>) {
    let file = fs::read_to_string(path).unwrap();
    let start = file.find("Zutaten").expect("Didn't find header »Zutaten«");
    let end = start + file[start..].find("#").unwrap_or(file.len() - start);

    let bulletpoint_regex = Regex::new("-(?<ingredient>.+)").unwrap();
    let participants_regex =
        Regex::new(r"(?<participants>\d+) ?(Persons|Portionen|Personen)").unwrap();
    let dish_participants = participants_regex
        .captures(&file)
        .expect(format!("Persons have to be given for {}", path.display()).as_str())
        .name("participants")
        .unwrap()
        .as_str()
        .parse::<usize>()
        .expect("Can't pares participants");

    let zubereitung = file.find("## Zubereitung");

    let upscale = participants / dish_participants as f32;
    let amounts = bulletpoint_regex
        .captures_iter(&file[start..end])
        .map(|capture| {
            let ingredient = capture.name("ingredient").unwrap().as_str().trim();
            let parts = ingredient.split(' ').collect::<Vec<_>>();
            if parts.len() == 0 {
                panic!("no ingredient found")
            }

            if parts.len() == 1 {
                Amount {
                    amount: 1.0,
                    measure: "".to_string(),
                    name: parts[0].to_string(),
                    dish: dish.to_string(),
                }
            } else {
                let amount = match parts[0].parse::<f32>() {
                    Ok(amount) => amount,
                    Err(_) => {
                        println!("couldn't parse unit {}, defaulting to 0", parts[0]);
                        println!("parts: {parts:?}");
                        0.0
                    }
                };

                let (measure, name) = if MEASURES.contains(&parts[1]) {
                    (parts[1].to_string(), parts[2..].join(" "))
                } else {
                    ("".to_string(), parts[1..].join(" "))
                };

                Amount {
                    amount,
                    measure,
                    name,
                    dish: dish.to_string(),
                }
            }
        })
        // upscale the amounts
        .map(|amount| Amount {
            amount: amount.amount * upscale,
            ..amount
        })
        .collect();

    let zubereitung =
        zubereitung.map(|start| file[(start + "## Zubereitung".len())..file.len() - 1].to_string());
    (amounts, zubereitung)
}

/// Collect all dishes recursively from the given path.
fn collect_dishes(dishes: &mut Vec<PathBuf>, path: &Path) {
    if path.is_dir() {
        let entries = fs::read_dir(path).unwrap().filter_map(Result::ok);
        entries.for_each(|entry| collect_dishes(dishes, &entry.path()))
    } else {
        dishes.push(path.to_path_buf());
    }
}

fn main() {
    let mut dishes = vec![];
    let base_path = Path::new(BASE);
    collect_dishes(&mut dishes, &base_path);
    println!("collected {} dishes", dishes.len());
    let dishbase = dishes
        .iter()
        .map(|path| (path.file_stem().unwrap().to_str().unwrap(), path))
        .collect::<HashMap<_, _>>();

    // Extract dishes from a dish plan
    // Format: [[<dish>]](<amount>)
    let regex = Regex::new(r#"\[\[(?<name>.*?)\]\](\((?<num>\d+)\))?"#).unwrap();
    let dishes = regex
        .captures_iter(PLAN)
        .map(|c| {
            (
                c.name("name").unwrap().as_str(),
                c.name("num").map(|a| a.as_str().parse::<f32>().unwrap()),
            )
        })
        .collect::<Vec<_>>();

    // Calculate amount of ingredients for each dish and how many participants are planned
    let (calcs, rest): (Vec<_>, Vec<_>) = dishes
        .clone()
        .into_iter()
        .map(|(dish, participants)| {
            let participants = participants.unwrap_or(PARTICIPANTS);
            println!("Upscaling {dish} with {participants}");
            calculate(
                dishbase
                    .get(dish)
                    .expect(&format!("Dish {dish} not in dishes."))
                    .as_path(),
                dish,
                participants,
            )
        })
        .unzip();

    // Add all the amounts
    let accumulated_amounts = calcs
        .iter()
        .map(|calc| {
            calc.into_iter()
                .map(|amount| (amount.name.clone(), amount))
                .collect::<HashMap<_, _>>()
        })
        .fold(
            HashMap::new(),
            |mut acc: HashMap<String, Vec<Amount>>, elem| {
                elem.into_iter().for_each(|(name, amount)| {
                    if let Some(amounts) = acc.get_mut(&name) {
                        amounts.push(amount.clone())
                    } else {
                        acc.insert(name, vec![amount.clone()]);
                    }
                });
                acc
            },
        );

    // Print the ingredient list
    let mut all = vec![];
    for (name, amount) in accumulated_amounts {
        all.push(format!(
            "- {name} [{}] ({})",
            amount
                .iter()
                .map(|amount| format!("{}{}", amount.amount, amount.measure))
                .collect::<Vec<_>>()
                .join(", "),
            amount
                .iter()
                .map(|amount| amount.dish.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }

    all.sort();
    let shopping_list = all.join("\n");
    fs::write("ingredients.md", &shopping_list).unwrap();

    // Write scaled up lists.
    fs::remove_dir("./scaled").ok();
    fs::create_dir("./scaled").ok();

    let mut amounts = calcs.into_iter();
    let mut rests = rest.into_iter();

    let mut pdf = format!(
        r##"
{PLAN}

<div style="page-break-after: always;"></div>

## Shopping List
{shopping_list}"##
    );

    for (dish, amount) in dishes {
        let amount = amount.unwrap_or(PARTICIPANTS);
        let ingredients = amounts
            .next()
            .unwrap()
            .into_iter()
            .map(
                |Amount {
                     amount,
                     measure,
                     name,
                     dish,
                 }| format!("- {amount} {measure} {name}"),
            )
            .collect::<Vec<_>>()
            .join("\n");
        let rest = rests.next().unwrap().unwrap_or("JUST DO IT".to_string());
        let file = format!(
            r####"
<div style="page-break-after: always;"></div>
## {dish}
{amount} Persons

## Zutaten
{ingredients}

## Zubereitung
{rest}

"####
        );
        pdf.push_str(&file);
    }
    fs::write("./full.md", pdf).unwrap();
    Command::new("md2pdf")
        .arg("--css ")
        .arg("css.css")
        .arg("full.md")
        .arg("plan.md")
        .status()
        .unwrap();
}
