use std::{error::Error, fs::File, hint::unreachable_unchecked, io::{BufWriter, Write}};

use chrono::{Datelike, Local, Timelike};

use crate::{
    executor::file_walker::ScanOutput,
    feature::{feature_builder::AppFeatures, format::Format, out::Out},
};

pub fn generate_report(features: AppFeatures, output: ScanOutput) {
    match features.out {
        Out::Console => console_report(features, output),
        Out::Directory { value: _ } => file_report(features, output),
    }
}

#[inline]
fn console_report(features: AppFeatures, output: ScanOutput) {
    let path = features.path.value.as_os_str().to_string_lossy().to_string();
    println!("\nReport on dynamic used libraries by ELF executables on {}\n", path);

    output.into_iter().for_each(|(arch, libs_to_files)| {
        println!("---------- {} ----------\n", arch.map(|x| x.to_string()).unwrap_or("Unknown architecture".to_string()));

        libs_to_files.into_iter().for_each(|(lib, entries)| {
            println!("{} ({} execs)\n", lib, entries.len());

            entries.into_iter().for_each(|e| {
                println!("\t-> {}\n", e.into_path().as_os_str().to_string_lossy().to_string())
            })
        });
    });
}

#[inline]
fn file_report(features: AppFeatures, output: ScanOutput) {
    match features.format {
        Format::TXT => txt_report(features, output),
        Format::MD => md_report(features, output),
    }.unwrap_or_else(|x| {
        eprintln!("Failed to produce the report: {:?}", x)
    })
}

#[inline]
fn txt_report(features: AppFeatures, output: ScanOutput) -> Result<(), Box<dyn Error>> {
    let mut out_path = match features.out {
        Out::Directory { value } => value,
        Out::Console => unsafe { unreachable_unchecked() },
    };

    let now = Local::now();
    let filename = format!("blld {}.{}.{} {}:{}:{}.txt", now.day(), now.month(), now.year(), now.hour(), now.minute(), now.second());
    out_path.push(filename);

    let display_path = out_path.as_path().as_os_str().to_string_lossy().to_string();

    let mut writer = BufWriter::new(File::create(out_path)?);

    let path = features.path.value.as_os_str().to_string_lossy().to_string();
    writeln!(writer, "Report on dynamic used libraries by ELF executables on {}\n", path)?;

    output.into_iter().fold(Ok(()), |_: Result<(), Box<dyn Error>>, (arch, libs_to_files)| {
        writeln!(writer, "---------- {} ----------\n", arch.map(|x| x.to_string()).unwrap_or("Unknown architecture".to_string()))?;

        libs_to_files.into_iter().fold(Ok(()), |_, (lib, entries)| {
            writeln!(writer, "{} ({} execs)\n", lib, entries.len())?;

            entries.into_iter().fold(Ok(()), |_, e| {
                writeln!(writer, "\t-> {}\n", e.into_path().as_os_str().to_string_lossy().to_string())?;
                Ok(())
            })
        })
    })?;

    println!("Report is generated to {}", display_path);
    Ok(())
}

#[inline]
fn md_report(features: AppFeatures, output: ScanOutput) -> Result<(), Box<dyn Error>> {
    let mut out_path = match features.out {
        Out::Directory { value } => value,
        Out::Console => unsafe { unreachable_unchecked() },
    };

    let now = Local::now();
    let filename = format!("blld {}.{}.{} {}:{}:{}.md", now.day(), now.month(), now.year(), now.hour(), now.minute(), now.second());
    out_path.push(filename);

    let display_path = out_path.as_path().as_os_str().to_string_lossy().to_string();

    let mut writer = BufWriter::new(File::create(out_path)?);

    let path = features.path.value.as_os_str().to_string_lossy().to_string();
    writeln!(writer, "# Report on dynamic used libraries by ELF executables on {}\n", path)?;

    output.into_iter().fold(Ok(()), |_: Result<(), Box<dyn Error>>, (arch, libs_to_files)| {
        writeln!(writer, "### {}\n", arch.map(|x| x.to_string()).unwrap_or("Unknown architecture".to_string()))?;

        libs_to_files.into_iter().fold(Ok(()), |_, (lib, entries)| {
            writeln!(writer, "*{} ({} execs)*\n", lib, entries.len())?;

            entries.into_iter().fold(Ok(()), |_, e| {
                writeln!(writer, "* {}\n", e.into_path().as_os_str().to_string_lossy().to_string())?;
                Ok(())
            })
        })
    })?;

    println!("Report is generated to {}", display_path);
    Ok(())
}
