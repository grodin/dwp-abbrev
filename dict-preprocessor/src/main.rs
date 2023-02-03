use serde_json::Value;

use dwp_abbrev_rs::abbreviation::Entry;

fn main() -> color_eyre::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    let abbrevs: Value = serde_json::from_str(&buffer)?;
    if let Value::Object(abbrevs) = abbrevs {
        let mut entries: Vec<Entry> = Vec::new();
        for (abbrev, desc) in abbrevs.iter() {
            if let Value::String(desc) = desc {
                if !desc.is_empty() && !abbrev.is_empty() {
                    entries.push((Entry::new(abbrev.clone(), desc.clone())).into())
                }
            }
        }
        entries.sort_unstable_by(|val1, val2| {
            val1.abbreviation()
                .to_ascii_uppercase()
                .cmp(&val2.abbreviation().to_ascii_uppercase())
        });
        let json = serde_json::to_string_pretty(&entries)?;
        println!("{}", json)
    }
    Ok(())
}
