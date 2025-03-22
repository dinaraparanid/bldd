#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ElfArch {
    X86, X86_64, ARMV7, AARCH64
}

impl std::fmt::Display for ElfArch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ElfArch::X86 => "x86",
            ElfArch::X86_64 => "x86-64",
            ElfArch::ARMV7 => "armv7",
            ElfArch::AARCH64 => "aarch64",
        })
    }
}

impl ElfArch {
    pub fn parse(input: &str) -> Result<ElfArch, ()> {
        match input {
            "elf64-x86-64" => Ok(ElfArch::X86_64),
            "elf32-i386" => Ok(ElfArch::X86),
            "elf32-arm" | "elf32-littlearm" => Ok(ElfArch::ARMV7),
            "elf64-arm " | "elf64-littlearm" => Ok(ElfArch::AARCH64),
            _ => Err(()),
        }
    }
}
