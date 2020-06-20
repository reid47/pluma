use crate::colors;
use pluma_constants::*;
use std::fmt;

pub struct CommandInfo {
  pub name: &'static str,
  pub description: &'static str,
  pub args: Option<Vec<Arg>>,
  pub flags: Option<Vec<Flag>>,
}

pub struct Arg {
  pub name: &'static str,
  pub description: &'static str,
  pub default: Option<&'static str>,
}

impl Arg {
  pub fn new(name: &'static str, description: &'static str) -> Self {
    Arg {
      name,
      description,
      default: None,
    }
  }

  pub fn default(mut self, default: &'static str) -> Self {
    self.default = Some(default);
    self
  }
}

pub struct Flag {
  long_name: &'static str,
  description: &'static str,
  short_name: Option<&'static str>,
  value_name: Option<&'static str>,
  default: Option<&'static str>,
  possible_values: Option<Vec<&'static str>>,
}

impl Flag {
  pub fn with_names(long_name: &'static str, short_name: &'static str) -> Self {
    Flag {
      long_name,
      short_name: Some(short_name),
      description: "",
      value_name: None,
      default: None,
      possible_values: None,
    }
  }

  pub fn description(mut self, description: &'static str) -> Self {
    self.description = description;
    self
  }

  pub fn value_name(mut self, value_name: &'static str) -> Self {
    self.value_name = Some(value_name);
    self
  }

  pub fn default(mut self, default: &'static str) -> Self {
    self.default = Some(default);
    self
  }

  pub fn possible_values(mut self, possible_values: Vec<&'static str>) -> Self {
    self.possible_values = Some(possible_values);
    self
  }
}

impl fmt::Display for CommandInfo {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(
      f,
      "{} {}\n",
      colors::bold(BINARY_NAME),
      colors::bold(self.name)
    )?;
    writeln!(f, "{}\n", self.description)?;

    writeln!(f, "{}", colors::bold("Usage:"))?;
    write!(f, "  {} {}", BINARY_NAME, self.name)?;

    let mut max_arg_length = 0;
    if let Some(args) = &self.args {
      for arg in args {
        max_arg_length = std::cmp::max(max_arg_length, arg.name.len() + 2);

        if arg.default.is_some() {
          write!(f, " [<{}>]", arg.name)?;
        } else {
          write!(f, " <{}>", arg.name)?;
        }
      }
    }

    if self.flags.is_some() {
      write!(f, " [options]")?;
    }

    if let Some(args) = &self.args {
      write!(f, "\n\n{}", colors::bold("Arguments:"))?;

      for arg in args {
        write!(
          f,
          "\n  {:width$}   {}",
          format!("<{}>", arg.name),
          arg.description,
          width = max_arg_length
        )?;

        if let Some(default) = arg.default {
          write!(f, " (default: {})", default)?;
        }
      }
    }

    if let Some(flags) = &self.flags {
      let mut max_flag_length = 0;

      for flag in flags {
        max_flag_length = std::cmp::max(max_flag_length, flag.long_name.len());
      }

      write!(f, "\n\n{}", colors::bold("Options:"))?;

      for flag in flags {
        write!(f, "\n  ")?;

        if let Some(name) = flag.short_name {
          write!(f, "-{}, ", name)?;
        }

        write!(
          f,
          "--{:width$}   {}",
          flag.long_name,
          flag.description,
          width = max_flag_length
        )?;

        if let Some(values) = &flag.possible_values {
          write!(f, " (one of: {})", values.join(", "))?;
        }

        if let Some(default) = &flag.default {
          write!(f, " (default: {})", default)?;
        }
      }
    }

    Ok(())
  }
}
