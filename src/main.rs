extern crate charts;
extern crate hdrhistogram;
extern crate lazy_static;
extern crate regex;

use charts::{BarDatum, Chart, ScaleBand, ScaleLinear, VerticalBarView};
use hdrhistogram::Histogram;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::Write;
use std::fs;

/// Name of the input file with measured benchmarks.
const DSNTK_VERSION: &str = "0.0.4";

/// Name of the input file with measured benchmarks.
const INPUT_FILE_NAME: &str = "data/benchmarks.txt";

/// Name of the output file with summary info data.
const OUTPUT_MD_FILE_NAME: &str = "data/README.md";

/// Name of the output file with histogram data.
const OUTPUT_HGRM_FILE_NAME: &str = "data/benchmarks.hgrm";

/// Name of the output file with SVG histogram.
const OUTPUT_SVG_FILE_NAME: &str = "data/benchmarks.svg";

/// Regular expression pattern for parsing single report line.
const LINE_PATTERN: &str = r"test\s+(?P<name>[^\s+]+)[^:]+:\s+(?P<duration>[0-9,]+)\s+(?P<unit>[^/]+)";

/// Flag indicating if benchmarks of calling external Java functions should be omitted.
/// These benchmarks call external Java server and in fact do not measure strictly DSNTK performance.
const SKIP_EXTERNAL_JAVA_FUNCTIONS: bool = true;

lazy_static! {
  static ref RE_LINE: Regex = Regex::new(format!("^{LINE_PATTERN}").as_str()).unwrap();
}

struct Data(String, f32);

impl BarDatum for Data {
  fn get_category(&self) -> String {
    self.0.clone()
  }
  fn get_value(&self) -> f32 {
    self.1
  }
  fn get_key(&self) -> String {
    "".to_string()
  }
}

/// Generates the histogram chart in SVG format.
fn generate_chart(data: Vec<Data>, max_value: f32) {
  let width = 2000;
  let height = 600;
  let (top, right, bottom, left) = (90, 40, 50, 60);
  let labels = data.iter().map(|item| item.0.clone()).collect::<Vec<String>>();
  let x = ScaleBand::new()
    .set_domain(labels)
    .set_range(vec![0, width - left - right])
    .set_inner_padding(0.1)
    .set_outer_padding(0.1);
  let y = ScaleLinear::new().set_domain(vec![0.0, max_value]).set_range(vec![height - top - bottom, 0]);
  let view = VerticalBarView::new().set_x_scale(&x).set_y_scale(&y).load_data(&data).unwrap();
  Chart::new()
    .set_width(width)
    .set_height(height)
    .set_margins(top, right, bottom, left)
    .add_title(format!("DSNTK performance histogram, version {DSNTK_VERSION}"))
    .add_view(&view)
    .add_axis_bottom(&x)
    .add_axis_left(&y)
    .add_left_axis_label("Microseconds")
    .add_bottom_axis_label("Percentile")
    .save(OUTPUT_SVG_FILE_NAME)
    .unwrap();
}

/// Main entrypoint of benchmarks tool.
fn main() {
  // prepare histogram data
  let input = fs::read_to_string(INPUT_FILE_NAME).expect("reading input file failed");
  let mut histogram = Histogram::<u64>::new(1).unwrap();
  for line in input.lines() {
    if let Some(captures) = RE_LINE.captures(line) {
      if let Some(name) = captures.name("name") {
        if SKIP_EXTERNAL_JAVA_FUNCTIONS && name.as_str().contains("dmn_3_0076") {
          continue;
        }
        if let Some(duration) = captures.name("duration") {
          if let Some(unit) = captures.name("unit") {
            let d = duration.as_str().replace(',', "").parse::<u64>().unwrap();
            let value = match unit.as_str() {
              "ns" => d,
              other => panic!("unexpected unit: '{}'", other),
            };
            histogram += value / 1000;
          }
        }
      }
    }
  }

  // generate summary report
  let mut buffer = String::new();
  let mut data = vec![];
  let mut max_value = 0.0;
  let _ = writeln!(&mut buffer, "# DSNTK performance report\n");
  let _ = writeln!(&mut buffer, "### Version v{}\n", DSNTK_VERSION);
  let _ = writeln!(&mut buffer, "Total number of samples: **{}**\n", histogram.len());
  let _ = writeln!(&mut buffer, "| Statistic |   Time |");
  let _ = writeln!(&mut buffer, "|----------:|-------:|");
  let _ = writeln!(&mut buffer, "|       Min |{:>4.0} µs |", histogram.min());
  let _ = writeln!(&mut buffer, "|       Max |{:>4.0} µs |", histogram.max());
  let _ = writeln!(&mut buffer, "|      Mean |{:>4.0} µs |", histogram.mean());
  let _ = writeln!(&mut buffer, "|    StdDev |{:>4.0} µs |", histogram.stdev());
  let _ = writeln!(&mut buffer, "\nPercentage of samples executed in less than specified time\n");
  let _ = writeln!(&mut buffer, "| Percentage |   Time | Samples |");
  let _ = writeln!(&mut buffer, "|-----------:|-------:|:--------|");
  let mut total = 0u64;
  for v in histogram.iter_recorded() {
    total += v.count_at_value();
    let _ = writeln!(&mut buffer, "|{:>10.1}% |{:>4.0} µs | {:<7} |", v.percentile(), v.value_iterated_to(), total,);
    let label = format!("{:.2}", v.percentile());
    let value = v.value_iterated_to() as f32;
    if value > max_value {
      max_value = value;
    }
    data.push(Data(label, value));
  }
  max_value = (max_value / 100.0).round() * 100.0;
  let _ = writeln!(&mut buffer, "\n\n![benchmarks.png](benchmarks.png)\n");
  fs::write(OUTPUT_MD_FILE_NAME, &buffer).expect("writing .md file failed");

  // generate HGRM file
  buffer.clear();
  let _ = writeln!(
    &mut buffer,
    "{:>10}   {:>10}    {:>10}    {:>10}\n",
    "Value", "Percentile", "TotalCount", "1/(1-Percentile)"
  );
  let mut total = 0_u64;
  for v in histogram.iter_recorded() {
    total += v.count_since_last_iteration();
    let perc = v.percentile() / 100.0;
    let _ = writeln!(
      &mut buffer,
      "{:>10}   {:>10.6}    {:>10}    {:>10.2}",
      v.value_iterated_to(),
      perc,
      total,
      1.0 / (1.0 - perc)
    );
  }
  fs::write(OUTPUT_HGRM_FILE_NAME, buffer).expect("writing .hgrm file failed");

  // generate chart
  generate_chart(data, max_value);
}
