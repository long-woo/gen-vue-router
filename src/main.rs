// use glob::glob;

use clap::{Command, Arg, ValueHint};
use clap_complete::{generate, Generator, Shell};
use std::{env, io};

fn main() {
  let matches = build_cli().get_matches();

  if let Some(output) = matches.get_one::<Shell>("output").copied() {
    let mut cmd = build_cli();

    print_completions(output, &mut cmd);
  }
}


// 获取 views 目录所有 vue 文件
// fn get_views(input: String) -> Vec<String> {
//     let mut views = Vec::new();
//     for entry in glob(input +"views/**/*.vue").unwrap() {
//         let entry = entry.unwrap();
//         views.push(entry.to_str().unwrap().to_string());
//     }
//     views
// }

fn build_cli() -> Command {
  let default_output = env::current_dir().unwrap().to_str().unwrap().to_owned();
  println!("output: {}", default_output);
  Command::new("gen-vue-router")
    .about("Vue 视图文件生成路由配置")
    .version("1.0.0")
    .author("loong.woo <longwunet@outlook.com>")
    .arg(
      Arg::new("input")
        .help("Views 目录")
        .required(true)
        .value_hint(ValueHint::DirPath)
        .index(1)
        // .action(ArgAction::Set)
        // .value_parser(value_parser!(Shell))
    )
    .arg(
      Arg::new("output")
      .short('o')
      .help("输出文件目录")
      .value_hint(ValueHint::DirPath)
    )
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
  generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}
