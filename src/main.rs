use std::fs;
use std::io;

/// A simple program to decompress a zip file
/// Usage: decompress_yt <filename> <Destination>
fn main() {
    match real_main() {
        Ok(code) => std::process::exit(code),
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}

/// The main logic of the program is in this function.
/// It returns an exit code.
fn real_main() -> io::Result<i32> {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return Ok(1);
    }
// Open the zip file
    let fname = std::path::Path::new(&*args[1]);
    let file = fs::File::open(&fname)?;
    let mut archive = zip::ZipArchive::new(file)?;

// Iterate through the files in the zip archive
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

// Display file information
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

// Create directories and files as needed
        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", file.name(), outpath.display());
            fs::create_dir_all(&outpath)?;
        } else {
            println!("File {} extracted to \"{}\" ({} bytes)", file.name(), outpath.display(), file.size());
        };

// Create parent directories if they don't exist
        if let Some(p) = outpath.parent() {
            if !p.exists() {
                fs::create_dir_all(&p)?;
            }
        }

// Write the file to disk
        let mut outfile = fs::File::create(&outpath)?;
        io::copy(&mut file, &mut outfile)?;    
        
// Set permissions on Unix systems
         #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
        
    };
    Ok(0)
}
