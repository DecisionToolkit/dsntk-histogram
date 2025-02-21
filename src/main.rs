extern crate hdrhistogram;
extern crate regex;

use hdrhistogram::Histogram;
use regex::Regex;
use std::fmt::Write;
use std::fs;

/// Name of the input file with measured benchmarks.
const DSNTK_VERSION: &str = "0.1.0";

/// Name of the input file with measured benchmarks.
const INPUT_FILE_NAME: &str = "data/benchmarks.txt";

/// Name of the output file with summary info data.
const OUTPUT_MD_FILE_NAME: &str = "data/README.md";

/// Name of the output file with histogram data.
const OUTPUT_HISTOGRAM_FILE_NAME: &str = "data/histogram.md";

/// Regular expression pattern for parsing single report line.
const LINE_PATTERN: &str = r#"^test\s+(?P<name>\S+)[^:]+:\s+(?P<duration>[0-9,.]+)\s+(?P<unit>[^/]+)"#;

/// Flag indicating if benchmarks of calling external Java functions should be omitted.
/// These benchmarks call external Java server and in fact do not measure
/// strictly performance of DecisionToolkit.
const SKIP_EXTERNAL_JAVA_FUNCTIONS: bool = true;

/// Main entrypoint of benchmarks tool.
fn main() {
  // Prepare regular expression for parsing single benchmark line.
  let re = Regex::new(LINE_PATTERN).unwrap();

  // Prepare histogram data.
  let input = fs::read_to_string(INPUT_FILE_NAME).expect("reading input file failed");
  let mut histogram = Histogram::<u64>::new(1).unwrap();
  for line in input.lines() {
    let Some(captures) = re.captures(line) else {
      println!("input line was not matched: {}", line);
      return;
    };
    if let Some(name) = captures.name("name") {
      if SKIP_EXTERNAL_JAVA_FUNCTIONS && name.as_str().contains("dmn_3_0076") {
        continue;
      }
      if let Some(duration) = captures.name("duration") {
        if let Some(unit) = captures.name("unit") {
          let duration_str = duration.as_str().replace(',', "");
          let Ok(d) = duration_str.parse::<f64>() else {
            println!("duration not parsed: {}", duration_str);
            return;
          };
          let value = match unit.as_str() {
            "ns" => d,
            other => panic!("unexpected unit: '{}'", other),
          };
          histogram += (value.round() as u64) / 1000;
        }
      }
    }
  }

  // Generate summary report.
  let mut buffer = String::new();
  let _ = writeln!(&mut buffer, "**dsntk** | DecisionToolkit\n");
  let _ = writeln!(&mut buffer, "# Performance report\n");
  let _ = writeln!(&mut buffer, "## v{}\n", DSNTK_VERSION);
  let _ = writeln!(&mut buffer, "### Total number of samples: {}\n", histogram.len());
  let _ = writeln!(&mut buffer, "| Statistic |   Time |");
  let _ = writeln!(&mut buffer, "|----------:|-------:|");
  let _ = writeln!(&mut buffer, "|       Min |{:>4.0} µs |", histogram.min());
  let _ = writeln!(&mut buffer, "|      Mean |{:>4.0} µs |", histogram.mean());
  let _ = writeln!(&mut buffer, "|    StdDev |{:>4.0} µs |", histogram.stdev());
  let _ = writeln!(&mut buffer, "|       Max |{:>4.0} µs |", histogram.max());
  let _ = writeln!(&mut buffer, "\n### Percentage of samples executed in less than specified time\n");
  let _ = writeln!(&mut buffer, "| Percentage |   Time | Samples |");
  let _ = writeln!(&mut buffer, "|-----------:|-------:|:--------|");
  let mut total = 0u64;
  for v in histogram.iter_recorded() {
    total += v.count_at_value();
    let _ = writeln!(&mut buffer, "|{:>10.2}% |{:>4.0} µs | {:<7} |", v.percentile(), v.value_iterated_to(), total,);
  }
  fs::write(OUTPUT_MD_FILE_NAME, &buffer).expect("writing .md file failed");

  // Generate histogram file.
  buffer.clear();
  let _ = writeln!(&mut buffer, "| {:>8} | {:>10} | {:>10} |", "Time", "Percentile", "TotalCount");
  let _ = writeln!(&mut buffer, "|---------:|-----------:|-----------:|");
  let mut total = 0_u64;
  for v in histogram.iter_recorded() {
    total += v.count_since_last_iteration();
    let perc = v.percentile() / 100.0;
    let _ = writeln!(&mut buffer, "| {:>5} µs | {:>10.2} | {:>10} |", v.value_iterated_to(), perc * 100.0, total);
  }
  fs::write(OUTPUT_HISTOGRAM_FILE_NAME, buffer).expect("writing .hgrm file failed");
}
