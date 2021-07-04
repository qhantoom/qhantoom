mod analyzer;
mod initcheck;
mod maincheck;
mod typecheck;

pub use self::analyzer::{
  analyze, analyze_capsule_from_file, analyze_capsule_from_source, Analyzer,
};
