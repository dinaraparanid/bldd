use crate::{
    executor::file_walker::ScanOutput,
    feature::{feature_builder::AppFeatures, out::Out},
};

pub fn generate_report(features: AppFeatures, output: ScanOutput) {
    match features.out {
        Out::Console => console_report(features, output),
        Out::Directory { value } => todo!(),
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
