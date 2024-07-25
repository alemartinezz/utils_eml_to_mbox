use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn path_check(path: &str) -> PathBuf {
    let p = Path::new(path);
    if p.is_absolute() {
        p.to_path_buf()
    } else {
        std::env::current_dir().unwrap().join(p)
    }
}

fn add_eml_to_mbox(eml_file: &Path, mbox_file: &mut File) -> io::Result<()> {
    let mut reader = BufReader::new(File::open(eml_file)?);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    // Write the "From " line
    mbox_file.write_all(b"From nobody@example.com Sat Jan 01 00:00:00 2000\n")?;
    mbox_file.write_all(&buffer)?;
    mbox_file.write_all(b"\n\n")?;

    Ok(())
}

fn create_mbox_from_single_eml(eml_file: &Path, output_path: &Path) -> io::Result<PathBuf> {
    let dest_name = "output.mbox";
    let dest_mbox_path = output_path.join(dest_name);

    let mut mbox_file = OpenOptions::new().create(true).append(true).open(&dest_mbox_path)?;
    add_eml_to_mbox(eml_file, &mut mbox_file)?;

    Ok(dest_mbox_path)
}

fn create_mbox_from_multiple_emls(eml_path: &Path, output_path: &Path) -> io::Result<Vec<(String, PathBuf)>> {
    let dest_name = "output.mbox";
    let dest_mbox_path = output_path.join(dest_name);

    let mut mbox_file = OpenOptions::new().create(true).append(true).open(&dest_mbox_path)?;
    let mut converted_files = Vec::new();

    for entry in WalkDir::new(eml_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "eml") {
            add_eml_to_mbox(path, &mut mbox_file)?;
            converted_files.push((path.file_name().unwrap().to_str().unwrap().to_string(), dest_mbox_path.clone()));
        }
    }

    Ok(converted_files)
}

fn main() -> io::Result<()> {
    let mut input_path = String::new();
    let mut output_path = String::new();

    println!("Please enter the folder or file path: ");
    io::stdin().read_line(&mut input_path)?;
    println!("Please enter the destination folder path: ");
    io::stdin().read_line(&mut output_path)?;

    let input_path = path_check(input_path.trim());
    let output_path = path_check(output_path.trim());

    if !output_path.exists() {
        eprintln!("Error: Destination folder {:?} does not exist.", output_path);
        return Ok(());
    }

    println!("\nOrigin folder/file:\n{:?}\n", input_path);
    println!("Destination folder/file:\n{:?}\n", output_path);

    let converted_files = if input_path.is_dir() {
        create_mbox_from_multiple_emls(&input_path, &output_path)?
    } else if input_path.is_file() && input_path.extension().map_or(false, |ext| ext == "eml") {
        let dest_name = create_mbox_from_single_eml(&input_path, &output_path)?;
        vec![(input_path.file_name().unwrap().to_str().unwrap().to_string(), dest_name)]
    } else {
        eprintln!("Error: Provided path is neither a folder nor a valid .eml file.");
        return Ok(());
    };

    println!("Converted files:");
    for (original, converted) in converted_files {
        println!("{} -> {:?} ✅", original, converted);
    }

    Ok(())
}
