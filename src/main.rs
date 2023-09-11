use std::fs::File;
use std::io::copy;
use clap::Parser;

/// Retrieve the attachments from a .goodnotes file and save them to the specified directory
#[derive(Parser)]
struct Cli {
    /// The input .goodnotes file
    input: std::path::PathBuf,
    /// The output directory for the attachments
    output: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let input_path = &args.input;
    let file = File::open(input_path).expect("Could not open input file");
    let file_name = input_path.file_name().unwrap().to_str().unwrap()
        .strip_suffix(".goodnotes").expect("Invalid input file format");
    let mut archive = zip::ZipArchive::new(file).expect("Could not open zip archive");
    println!("Got archive with length: {:?}", archive.len());

    let mut attachment_num = 0;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).expect("Failed to get file");

        if file.name().starts_with("attachments/") {
            attachment_num += 1;

            println!("Got attachment: {:?}", file.name());

            let output_path = args.output.join(file_name.to_string()
                + "_attachment_" + &attachment_num.to_string() + ".pdf");

            // Ensure the parent directories exist
            if let Some(parent_dir) = std::path::Path::new(&output_path).parent() {
                std::fs::create_dir_all(parent_dir).expect("Failed to create output directory");
            }

            // Create the output file
            let mut output_file = File::create(&output_path).expect("Failed to create output file");

            // Copy the file contents - todo remove blank pages from the pdf
            copy(&mut file, &mut output_file).expect("Failed to copy attachment file");
        }
    }
}
