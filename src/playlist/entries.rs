use super::PlaylistEntry;

use std::default::Default;
use std::io::{BufReader, BufRead, BufWriter, LineWriter, Read, Write};
use std::path::PathBuf;


use once_cell::sync::OnceCell;
use regex::Regex;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlaylistEntries {
    entries: Vec<PlaylistEntry>,
}



static BLANK_RE: OnceCell<Regex> = OnceCell::new();
static EXT_RE: OnceCell<Regex> = OnceCell::new();


impl PlaylistEntries {
    pub fn read<R: Read>(r: R) -> Self 
     {
        let mut r = BufReader::new(r);
        let mut entries: Vec<PlaylistEntry> = Vec::new();
        const EXT_PATTERN:&str = r##"^[^# ].*"##;
        EXT_RE.get_or_try_init(|| Regex::new(EXT_PATTERN)).unwrap();
        const BLANK_PATTERN:&str = r##"^\s.*"##;
        BLANK_RE.get_or_try_init( || Regex::new(BLANK_PATTERN)).unwrap();

        for line in r.lines(){
            let line = line.unwrap();
            if EXT_RE.get().unwrap().is_match(&line) {
                entries.push(PlaylistEntry{
                    path: PathBuf::from(&line),
                    ..PlaylistEntry::default()
                });
            }
        }
        PlaylistEntries {
            entries: entries,
        }
        
    }
    pub fn write<W: Write>(&self, w: W) -> Result<(), std::io::Error> {
        let mut l = LineWriter::new(w);
        for entry in &self.entries {
            l.write_all(entry.path.to_str().unwrap().as_bytes());
            l.flush()?;
        }
        Ok(())
    }
}

impl IntoIterator for PlaylistEntries {
    type Item = PlaylistEntry;
    type IntoIter = std::vec::IntoIter<PlaylistEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl<'a> IntoIterator for &'a PlaylistEntries {
    type Item = &'a PlaylistEntry;
    type IntoIter = std::slice::Iter<'a, PlaylistEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn readwrite() {
        fn test_read_write( s: &str, e: PlaylistEntries) {
            
            assert_eq!(PlaylistEntries::read(s.as_bytes()), e);
            let mut write: Vec<u8> = Vec::new();
            e.write(&mut write);

            assert_eq!(String::from_utf8(write).unwrap(), s.to_string());

        }
        const PLAIN_M3U: &str = r"album/track.mp3";
        let mut result = PlaylistEntries::default();
        result.entries.push(PlaylistEntry{path: PathBuf::from("album/track.mp3"), ..PlaylistEntry::default()});
        test_read_write(PLAIN_M3U, result);
    }
}