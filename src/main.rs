use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};
use serde_json;
use clap::Parser;
#[cfg(feature = "hyphenation")]
use hyphenation::{Language, Load, Standard};
use textwrap::{Options, termwidth, fill, indent};
use console::style;

mod mw_json;
use mw_json::Synonyms;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    word: Option<String>,
}

fn check_spelling(resp: String) -> Result<Vec<Synonyms>, Box<dyn std::error::Error>>{
    let results = match serde_json::from_str::<Vec<Synonyms>>(&resp) {
        Ok(r) => r,
        Err(_) => {
            let spellings = &serde_json::from_str::<Vec<String>>(&resp)?;
            let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                .with_prompt("Did you mean:")
                .default(0)
                .items(&spellings[..])
                .interact()
                .unwrap();
            let resp = reqwest::blocking::get(
                format!(
                    "https://www.dictionaryapi.com/api/v3/references/thesaurus/json/{}?key=a6fa785c-459b-49ff-a11c-32a074c140e9",
                    spellings[selection]
                    )
                )?.text()?;
            serde_json::from_str::<Vec<Synonyms>>(&resp)?
        }
    };
    Ok(results)
}

fn print_synonyms_and_antonyms(words_info: &Vec<Synonyms>, index: usize, options1: Options, options2: Options) {
    let w = &words_info[index];
    let word = format!("{} {}.", style(format!("{} ({}):", w.meta.id, w.fl)).magenta().bold(), w.shortdef.join("; "));
    println!("{}", fill(&word, &options1));
    println!("{}", style("Synonyms:").cyan().bold());
    for (i, syn_group) in words_info[index].meta.syns.iter().enumerate() {
        let syn_string = format!("{}) {}.", i + 1, syn_group.join(", "));
        let pretty_print = indent(&fill(&syn_string, &options2), "    ");
        println!("{}", pretty_print);
    }
    let antonym_groups = words_info[index].meta.ants.iter();
    if antonym_groups.len() > 0 {
        println!("{}", style("Antonyms:").yellow().bold());
        for (i, ant_group) in antonym_groups.enumerate() {
            let ant_string = format!("{}) {}.", i + 1, ant_group.join(", "));
            let pretty_print = indent(&fill(&ant_string, &options2), "    ");
            println!("{}", pretty_print);
        }
    }
}

fn lookup_word(word: String) -> Result<(), Box<dyn std::error::Error>>{
    let options1 = Options::new(termwidth() - 4).subsequent_indent("    ");
    let options2 = Options::new(termwidth() - 8).subsequent_indent("   ");

    let resp = reqwest::blocking::get(
            format!(
                "https://www.dictionaryapi.com/api/v3/references/thesaurus/json/{}?key=a6fa785c-459b-49ff-a11c-32a074c140e9",
                word
            )
        )?.text()?;
    let results = &check_spelling(resp)?;
    let selections: &Vec<String> = &results
        .into_iter()
        .map(|r| format!(
                "{} ({}): {}.", r.meta.id, r.fl, r.shortdef.join("; ")),
            )
        .collect();
    let n = selections.len();
    if n == 0 {
        println!("No results found for {:#?}.", word);
    } else if n == 1 {
        print_synonyms_and_antonyms(
            &results,
            0,
            options1,
            options2
        );
    } else {
        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a definition:")
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();

        print_synonyms_and_antonyms(
            &results,
            selection,
            options1,
            options2
        );
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = Args::parse();
    if let Some(word) = args.word {
        lookup_word(word)?;
    }
    loop {
        let word : String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter a word:")
            .interact_text()?;
        lookup_word(word)?;
    }
}
