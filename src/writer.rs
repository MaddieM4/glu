use std::fs;
use std::path::Path;
use safe_path::scoped_join;
use crate::segment::Segment;

pub fn write_files<R: AsRef<Path>>(root: R, segments: &Vec<Segment>) -> std::io::Result<()> {
    fs::create_dir_all(&root)?;
    for segment in segments {
        let path = scoped_join(&root, &segment.file_name)?;
        fs::write(path, &segment.contents)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn create_dir() {
        let tmp = TempDir::new("glu-test").unwrap();
        let root = scoped_join(tmp.path(), "root").unwrap();
        write_files(&root, &vec![]).unwrap();

        let metadata = fs::metadata(&root);
        assert!(metadata.is_ok(), "Path {:?} does not exist", root);
        assert!(metadata.unwrap().is_dir(), "Path {:?} is not a directory", root);
    }

    #[test]
    fn create_file() {
        let tmp = TempDir::new("glu-test").unwrap();
        let root = scoped_join(tmp.path(), "root").unwrap();
        write_files(&root, &vec![
            Segment {
                file_name: "foo.txt".into(),
                file_type: "text".into(),
                contents: "Some data".into(),
            }
        ]).unwrap();

        let file_path = root.join("foo.txt");
        let contents: Vec<u8> = std::fs::read(file_path).expect("Making sure file was written");
        assert_eq!(String::from_utf8(contents), Ok("Some data".to_string()));
    }

    #[test]
    fn dir_exists() {
        let tmp = TempDir::new("glu-test").unwrap();
        let root = scoped_join(tmp.path(), "root").unwrap();
        write_files(&root, &vec![
            Segment {
                file_name: "foo.txt".into(),
                file_type: "text".into(),
                contents: "Some data".into(),
            }
        ]).expect("Establishing dir and files");
        write_files(&root, &vec![]).expect("Running write_files again");

        let file_path = root.join("foo.txt");
        let contents: Vec<u8> = std::fs::read(file_path).expect("Making sure file still exists");
        assert_eq!(String::from_utf8(contents), Ok("Some data".to_string()));
    }
}
