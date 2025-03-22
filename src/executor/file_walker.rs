use std::{collections::HashMap, error::Error, process::{ChildStdout, Command, Stdio}};

use itertools::Itertools;
use walkdir::{DirEntry, WalkDir};

use crate::{feature::feature_builder::AppFeatures, parser::elf_arch::ElfArch};

pub type Library = String;
pub type ScanOutput = HashMap<Option<ElfArch>, Vec<(Library, Vec<DirEntry>)>>;

pub fn walk(features: &AppFeatures) -> ScanOutput {
    WalkDir::new(&features.path.value)
        .into_iter()
        .map(|res| res.map(|e| get_entry_info(e))?)
        .filter_map(|x| x.ok())
        .flat_map(|(dir, arch, libs)| libs.into_iter().map(move |lib| (dir.clone(), arch, lib)))
        .into_group_map_by(|(_, arch, _)| arch.clone())
        .into_iter()
        .map(|(arch, entries)| {
            let libs_to_files = entries
                .into_iter()
                .map(|(elf, _, lib)| (elf, lib))
                .into_group_map_by(|(_, lib)| lib.clone())
                .into_iter()
                .map(|(lib, elfs)| {
                    (lib, elfs.into_iter().map(|(elf, _)| elf).collect::<Vec<_>>())
                })
                .sorted_by(|(_, elfs1), (_, elfs2)| elfs1.len().cmp(&elfs2.len()))
                .rev()
                .collect();

            (arch, libs_to_files)
        })
        .collect()
}

#[inline]
fn get_entry_info(entry: DirEntry) -> Result<(DirEntry, Option<ElfArch>, Vec<Library>), Box<dyn Error>> {
    let arch = get_arch(&entry);
    let libs = get_libs(&entry)?;
    Ok((entry, arch, libs))
}

#[inline]
fn objdump_out(entry: &DirEntry) -> Result<ChildStdout, Box<dyn Error>> {
    let process = Command::new("objdump")
        .arg("-p")
        .arg(entry.path().to_string_lossy().to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;

    Ok(process.stdout.unwrap())
}

#[inline]
fn get_arch(entry: &DirEntry) -> Option<ElfArch> {
    fn get_grep(entry: &DirEntry) -> Result<String, Box<dyn Error>> {
        let grep = Command::new("grep")
            .arg("elf")
            .stdin(objdump_out(entry)?)
            .stderr(Stdio::null())
            .output()
            .map(|x| x.stdout)
            .unwrap_or_default();

        let elf_str = String::from_utf8(grep)?
            .split_whitespace()
            .find(|s| s.starts_with("elf"))
            .unwrap_or_default()
            .to_string();

        Ok(elf_str)
    }

    ElfArch::parse(&get_grep(entry).unwrap_or_default()).ok()
}

#[inline]
fn get_libs(entry: &DirEntry) -> Result<Vec<Library>, Box<dyn Error>> {
    let libs_grep = Command::new("grep")
        .arg("NEEDED")
        .stdin(objdump_out(entry)?)
        .stderr(Stdio::null())
        .output()?
        .stdout;

    let libs_out = String::from_utf8(libs_grep)?;

    let libs = libs_out
        .split_whitespace()
        .filter(|&s| {
            let out = s.trim();
            !out.is_empty() && out != "NEEDED"
        })
        .map(|s| s.to_string())
        .collect();

    Ok(libs)
}
