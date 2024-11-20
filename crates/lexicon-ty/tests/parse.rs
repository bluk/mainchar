use std::{
    fs, io, iter,
    path::{Path, PathBuf},
};

fn json_iter<P>(path: P) -> impl Iterator<Item = io::Result<PathBuf>>
where
    P: AsRef<Path>,
{
    fn visit_dir(path: PathBuf, entries: &mut Vec<PathBuf>) -> io::Result<()> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            entries.push(path);
        }

        Ok(())
    }

    let mut entries: Vec<PathBuf> = Vec::new();
    entries.push(path.as_ref().to_path_buf());

    iter::from_fn(move || loop {
        let entry = entries.pop()?;
        if entry.is_dir() {
            if let Err(error) = visit_dir(entry, &mut entries) {
                entries.clear();
                return Some(Err(error));
            }
        } else if entry.is_file() && entry.extension().is_some_and(|ext| ext == "json") {
            return Some(Ok(entry));
        }
    })
}

#[test]
fn test_parse_files() -> io::Result<()> {
    for path in json_iter("../../resources/lexicons") {
        let path = path?;

        let contents = fs::read_to_string(&path)?;

        let lexicon_res: Result<lexicon_ty::Lexicon, serde_json::Error> =
            serde_json::from_str(&contents);

        if let Err(error) = lexicon_res {
            panic!("{} - {}", path.to_string_lossy(), error);
        }
    }

    Ok(())
}
