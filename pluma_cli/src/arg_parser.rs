use crate::command_error::{CommandError, CommandErrorKind};
use std::collections::{HashMap, HashSet};

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ParsedArgs {
  subcommand: String,
  positional_args: Vec<String>,
  flags: HashMap<String, Vec<String>>,
  additional_args: Vec<String>,
  retrieved_args: HashSet<usize>,
  retrieved_flags: HashSet<String>,
}

impl ParsedArgs {
  pub fn subcommand(&self) -> &String {
    &self.subcommand
  }

  pub fn get_positional_arg(&mut self, index: usize) -> Option<String> {
    self.retrieved_args.insert(index);

    match self.positional_args.get(index) {
      None => None,
      Some(arg) => Some(arg.clone()),
    }
  }

  pub fn get_flag_value(&mut self, flag: &'static str) -> Option<String> {
    self.retrieved_flags.insert(flag.to_owned());

    match self.flags.get(flag) {
      None => None,
      Some(vals) if vals.is_empty() => None,
      Some(vals) => Some(vals.last().unwrap().clone()),
    }
  }

  pub fn is_flag_present(&mut self, flag: &'static str) -> bool {
    self.retrieved_flags.insert(flag.to_owned());

    match self.flags.get(flag) {
      None => false,
      _ => true,
    }
  }

  pub fn is_help_requested(&mut self) -> bool {
    self.is_flag_present("help") || self.is_flag_present("h")
  }

  pub fn check_valid(&self) -> Result<(), CommandError> {
    let mut index = 0;

    while index < self.positional_args.len() {
      if self.retrieved_args.get(&index).is_none() {
        return Err(CommandError {
          command: self.subcommand.clone(),
          kind: CommandErrorKind::UnexpectedArgument(
            self.positional_args.get(index).unwrap().to_owned(),
          ),
        });
      }

      index += 1;
    }

    for (given_flag, _) in &self.flags {
      if !self.retrieved_flags.contains(given_flag) {
        return Err(CommandError {
          command: self.subcommand.clone(),
          kind: CommandErrorKind::UnexpectedFlag(given_flag.to_owned()),
        });
      }
    }

    Ok(())
  }
}

pub fn parse_args(args: Vec<String>) -> ParsedArgs {
  let mut parsed = ParsedArgs {
    subcommand: "".to_owned(),
    positional_args: Vec::new(),
    flags: HashMap::new(),
    additional_args: Vec::new(),
    retrieved_args: HashSet::new(),
    retrieved_flags: HashSet::new(),
  };

  let mut i = 0;
  let mut found_additional_separator = false;

  while i < args.len() {
    let arg = &args[i];

    if found_additional_separator {
      parsed.additional_args.push(arg.clone());
      i += 1;
      continue;
    }

    if arg.starts_with("-") {
      if arg == "--" {
        found_additional_separator = true;
        i += 1;
        continue;
      }

      let is_long_flag = arg.len() > 1 && arg.bytes().nth(1).unwrap() == b'-';
      let name_start = if is_long_flag { 2 } else { 1 };
      let arg_name = arg[name_start..].to_owned();

      let next_value = args.get(i + 1);

      if next_value.is_none() || next_value.unwrap().starts_with("-") {
        if !parsed.flags.contains_key(&arg_name) {
          parsed.flags.insert(arg_name, vec![]);
        }

        i += 1;
        continue;
      }

      if parsed.flags.contains_key(&arg_name) {
        let entry = parsed.flags.get_mut(&arg_name).unwrap();
        entry.push(next_value.unwrap().clone());
      } else {
        parsed
          .flags
          .insert(arg_name, vec![next_value.unwrap().clone()]);
      }

      i += 2;
      continue;
    }

    if parsed.subcommand.is_empty() {
      parsed.subcommand = arg.clone();
      i += 1;
      continue;
    }

    parsed.positional_args.push(arg.clone());

    i += 1;
  }

  parsed
}
