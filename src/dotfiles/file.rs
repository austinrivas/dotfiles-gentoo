use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct Asset;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file() {
        let test_sh = Asset::get("test.sh").unwrap();
        let content = std::str::from_utf8(test_sh.as_ref())
            .expect("could not read file");

        assert_eq!(content, "#!/bin/bash\n\necho \"this is a test shell script\"");

        for file in Asset::iter() {
            println!("{}", file.as_ref());
        }
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn fail_not_found() {
        Asset::get("does-not-exist.sh").unwrap();
    }
    
    #[test]
    fn read_dir() {
        let mut dir = vec![];
        for file in Asset::iter() {
            dir.push(file);
        }

        assert_eq!(dir, vec!["test.toml", "test.sh", "test"]);
    }
}