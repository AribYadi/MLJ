use std::io::Read;
use std::process;

mod assembler;
mod token;

fn main() {
  let in_file_path = match std::env::args().nth(1) {
    Some(in_file_path) => in_file_path,
    None => {
      eprintln!("Expected file path!");
      process::exit(1);
    },
  };
  let path = std::path::Path::new(&in_file_path).with_extension("obj");
  let out_file_path = path.to_string_lossy();

  let in_file = match std::fs::File::open(&in_file_path) {
    Ok(in_file) => in_file,
    Err(_) => {
      eprintln!("Cannot read file!");
      process::exit(1);
    },
  };
  let mut in_file = std::io::BufReader::new(in_file);
  let mut in_file_buf = String::new();
  let _ = in_file.read_to_string(&mut in_file_buf);

  let out_file =
    match std::fs::File::options().create(true).truncate(true).write(true).open(&*out_file_path) {
      Ok(out_file) => out_file,
      Err(_) => {
        eprintln!("Cannot write out file!");
        process::exit(1);
      },
    };
  let mut out_file = std::io::BufWriter::new(out_file);

  let errored = assembler::assemble(&in_file_path, &in_file_buf, &mut out_file);

  if errored {
    process::exit(1);
  }
}
