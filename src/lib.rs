use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;

const DEFAULT_FILE: &str = "/proc/cpuinfo";
const KIB: usize = 1024;
const MIB: usize = 1024 * KIB;
const GIB: usize = 1024 * MIB;

#[derive(Debug, Eq, PartialEq)]
pub struct CpuInfo {
    text: String,
}

impl CpuInfo {
    /// Reads CPU information from `/proc/cpuinfo`.
    /// # Errors
    /// Returns an [`std::io::Error`] if the file could not be read
    pub fn read() -> Result<Self, std::io::Error> {
        Self::read_from(DEFAULT_FILE)
    }

    /// Reads CPU information from the given file.
    /// # Errors
    /// Returns an [`std::io::Error`] if the file could not be read
    pub fn read_from(filename: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        read_to_string(filename).map(|text| Self { text })
    }

    #[must_use]
    pub fn cpu(&self, index: usize) -> Option<Cpu> {
        self.cpus()
            .filter_map(|cpu| cpu.processor().map(|processor| (processor, cpu)))
            .find_map(|(processor, cpu)| if processor == index { Some(cpu) } else { None })
    }

    pub fn cpus(&self) -> impl Iterator<Item = Cpu> {
        self.text
            .split("\n\n")
            .filter(|text| !text.is_empty())
            .map(Cpu::from_str)
    }

    pub fn iter(&self) -> impl Iterator<Item = Cpu> {
        self.cpus()
    }
}

impl From<&str> for CpuInfo {
    fn from(s: &str) -> Self {
        Self::from(s.to_string())
    }
}

impl From<String> for CpuInfo {
    fn from(text: String) -> Self {
        Self { text }
    }
}

impl FromStr for CpuInfo {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Cpu<'cpu_info>(HashMap<&'cpu_info str, &'cpu_info str>);

impl<'cpu_info> Cpu<'cpu_info> {
    fn from_str(s: &'cpu_info str) -> Self {
        Self(
            s.lines()
                .filter_map(|line| line.split_once(':'))
                .map(|(key, value)| (key.trim(), value.trim()))
                .collect(),
        )
    }

    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).copied()
    }

    #[must_use]
    pub fn processor(&self) -> Option<usize> {
        self.get("processor").and_then(|s| s.parse().ok())
    }

    #[must_use]
    pub fn vendor_id(&self) -> Option<&str> {
        self.get("vendor_id")
    }

    #[must_use]
    pub fn cpu_family(&self) -> Option<u8> {
        self.get("cpu family").and_then(|s| s.parse().ok())
    }

    #[must_use]
    pub fn model(&self) -> Option<usize> {
        self.get("model").and_then(|s| s.parse().ok())
    }

    #[must_use]
    pub fn model_name(&self) -> Option<&str> {
        self.get("model name")
    }

    #[must_use]
    pub fn stepping(&self) -> Option<usize> {
        self.get("stepping").and_then(|s| s.parse().ok())
    }

    #[must_use]
    pub fn microcode(&self) -> Option<usize> {
        self.get("microcode")
            .and_then(|s| usize::from_str_radix(s.trim_start_matches("0x"), 16).ok())
    }

    #[must_use]
    pub fn cpu_mhz(&self) -> Option<f32> {
        self.get("cpu MHz").and_then(|s| s.parse().ok())
    }

    /// Returns the CPU's cache size in bytes
    #[must_use]
    pub fn cache_size(&self) -> Option<usize> {
        self.get("cache size")
            .and_then(|s| match s.split_once(' ') {
                Some((value, unit)) => {
                    let value: usize = value.parse().ok()?;
                    match unit {
                        "B" => Some(value),
                        "KB" => Some(value * KIB),
                        "MB" => Some(value * MIB),
                        "GB" => Some(value * GIB),
                        _ => None,
                    }
                }
                None => s.parse().ok(),
            })
    }

    #[must_use]
    pub fn physical_id(&self) -> Option<usize> {
        self.get("physical id").and_then(|s| s.parse().ok())
    }

    #[must_use]
    pub fn siblings(&self) -> Option<usize> {
        self.get("siblings").and_then(|s| s.parse().ok())
    }

    #[must_use]
    pub fn core_id(&self) -> Option<usize> {
        self.get("core id").and_then(|s| s.parse().ok())
    }

    #[must_use]
    pub fn cpu_cores(&self) -> Option<usize> {
        self.get("cpu cores").and_then(|s| s.parse().ok())
    }

    #[must_use]
    pub fn apicid(&self) -> Option<usize> {
        self.get("apicid").and_then(|s| s.parse().ok())
    }

    #[must_use]
    pub fn initial_apicid(&self) -> Option<usize> {
        self.get("initial apicid").and_then(|s| s.parse().ok())
    }

    #[must_use]
    pub fn fpu(&self) -> Option<bool> {
        self.get("fpu").map(|s| s == "yes")
    }

    #[must_use]
    pub fn fpu_exception(&self) -> Option<bool> {
        self.get("fpu_exception").map(|s| s == "yes")
    }

    #[must_use]
    pub fn cpuid_level(&self) -> Option<usize> {
        self.get("cpuid level").and_then(|s| s.parse().ok())
    }

    #[must_use]
    pub fn wp(&self) -> Option<bool> {
        self.get("wp").map(|s| s == "yes")
    }

    #[must_use]
    pub fn flags(&self) -> HashSet<&str> {
        self.get("flags")
            .map_or_else(HashSet::default, |s| s.split(' ').collect())
    }

    #[must_use]
    pub fn vmx_flags(&self) -> HashSet<&str> {
        self.get("vmx flags")
            .map_or_else(HashSet::default, |s| s.split(' ').collect())
    }

    #[must_use]
    pub fn bugs(&self) -> HashSet<&str> {
        self.get("bugs")
            .map_or_else(HashSet::default, |s| s.split(' ').collect())
    }

    #[must_use]
    pub fn bogomips(&self) -> Option<f32> {
        self.get("bogomips").and_then(|s| s.parse().ok())
    }

    #[must_use]
    pub fn clflush_size(&self) -> Option<usize> {
        self.get("clflush size").and_then(|s| s.parse().ok())
    }

    #[must_use]
    pub fn cache_alignment(&self) -> Option<usize> {
        self.get("cache_alignment").and_then(|s| s.parse().ok())
    }

    #[must_use]
    pub fn address_sizes(&self) -> Option<(usize, usize)> {
        self.get("address sizes")
            .and_then(|s| s.split_once(','))
            .map(|(lhs, rhs)| {
                (
                    lhs.trim().trim_end_matches(" bits physical"),
                    rhs.trim().trim_end_matches(" bits virtual"),
                )
            })
            .and_then(|(phy, vir)| {
                phy.parse()
                    .ok()
                    .and_then(|phy| vir.parse().ok().map(|vir| (phy, vir)))
            })
    }

    #[must_use]
    pub fn power_management(&self) -> Option<&str> {
        self.get("power management")
    }
}
