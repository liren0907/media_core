use crate::benchmark::runner::{BenchmarkResult, SuiteResult};
use std::fs::File;
use std::io::Write;

pub enum ReportFormat {
    Console,
    Json,
}

pub trait Report {
    fn to_console(&self);
    fn to_json(&self, path: &str) -> Result<(), std::io::Error>;
}

impl Report for BenchmarkResult {
    fn to_console(&self) {
        self.print_summary();
    }

    fn to_json(&self, path: &str) -> Result<(), std::io::Error> {
        let mut file = File::create(path)?;

        writeln!(file, "{{")?;
        writeln!(file, "  \"name\": \"{}\",", self.name)?;
        writeln!(file, "  \"runs\": {},", self.runs)?;
        writeln!(
            file,
            "  \"average_ms\": {:.3},",
            self.average.as_secs_f64() * 1000.0
        )?;
        writeln!(
            file,
            "  \"min_ms\": {:.3},",
            self.min.as_secs_f64() * 1000.0
        )?;
        writeln!(
            file,
            "  \"max_ms\": {:.3},",
            self.max.as_secs_f64() * 1000.0
        )?;
        writeln!(
            file,
            "  \"std_dev_ms\": {:.3},",
            self.std_dev.as_secs_f64() * 1000.0
        )?;
        writeln!(file, "  \"durations_ms\": [")?;
        for (i, duration) in self.durations.iter().enumerate() {
            let comma = if i < self.durations.len() - 1 {
                ","
            } else {
                ""
            };
            writeln!(file, "    {:.3}{}", duration.as_secs_f64() * 1000.0, comma)?;
        }
        writeln!(file, "  ]")?;
        writeln!(file, "}}")?;

        Ok(())
    }
}

impl Report for SuiteResult {
    fn to_console(&self) {
        self.print_summary();
    }

    fn to_json(&self, path: &str) -> Result<(), std::io::Error> {
        let mut file = File::create(path)?;

        writeln!(file, "{{")?;
        writeln!(file, "  \"suite_name\": \"{}\",", self.name)?;
        writeln!(file, "  \"results\": [")?;

        for (i, result) in self.results.iter().enumerate() {
            let comma = if i < self.results.len() - 1 { "," } else { "" };
            writeln!(file, "    {{")?;
            writeln!(file, "      \"name\": \"{}\",", result.name)?;
            writeln!(file, "      \"runs\": {},", result.runs)?;
            writeln!(
                file,
                "      \"average_ms\": {:.3},",
                result.average.as_secs_f64() * 1000.0
            )?;
            writeln!(
                file,
                "      \"min_ms\": {:.3},",
                result.min.as_secs_f64() * 1000.0
            )?;
            writeln!(
                file,
                "      \"max_ms\": {:.3}",
                result.max.as_secs_f64() * 1000.0
            )?;
            writeln!(file, "    }}{}", comma)?;
        }

        writeln!(file, "  ]")?;
        writeln!(file, "}}")?;

        Ok(())
    }
}
