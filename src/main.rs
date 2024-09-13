use std::fs;
use std::io::{self, BufReader, Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::fs::File;
use url_escape::decode;

fn main() -> io::Result<()> {
    // Start the TCP listener on the specified address
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server running on http://127.0.0.1:8080");

    // Define the root directory for the file server
    let root_dir = Path::new("study/").canonicalize().expect("Failed to canonicalize root directory");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 1024];
                stream.read(&mut buffer)?;

                let request = String::from_utf8_lossy(&buffer[..]);

                // Get the requested path
                let path = get_requested_path(&request, &root_dir);

                if path.is_dir() {
                    // Serve directory listing
                    let response = list_directory_contents(&path);
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                        response.len(),
                        response
                    );
                    stream.write_all(response.as_bytes())?;
                } else if path.exists() {
                    // Serve the file
                    serve_file(&path, &mut stream)?;
                } else {
                    // Return 404
                    let response = not_found_page();
                    let response = format!(
                        "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                        response.len(),
                        response
                    );
                    stream.write_all(response.as_bytes())?;
                }

                stream.flush()?;
            },
            Err(e) => eprintln!("Failed to handle connection: {}", e),
        }
    }

    Ok(())
}

// Extract the requested path from the HTTP request
fn get_requested_path(request: &str, root: &Path) -> PathBuf {
    let start = match request.find("GET ") {
        Some(pos) => pos + 4,
        None => return root.to_path_buf(),
    };

    let end = match request.find(" HTTP/1.1") {
        Some(pos) => pos,
        None => return root.to_path_buf(),
    };

    let raw_path = &request[start..end];
    let decoded_path = decode(raw_path).to_string(); // Convert Cow<'_, str> to String

    // Avoid potential directory traversal attacks
    sanitize_path(root.join(decoded_path.trim_start_matches('/')))
}

fn sanitize_path(path: PathBuf) -> PathBuf {
    let root = Path::new("study/").canonicalize().unwrap();
    let full_path = path.canonicalize().unwrap_or_else(|_| root.clone());
    
    // Ensure path is within the allowed root directory
    if full_path.starts_with(&root) {
        full_path
    } else {
        root
    }
}

// List the directory contents in an HTML format
fn list_directory_contents(dir: &Path) -> String {
    let mut html = String::new();

    // Current and parent directory links
    let current_dir = dir.display().to_string().replace(r"\\?\", "");
    let parent_dir = dir.parent().unwrap_or(dir);
    let parent_dir_link = if parent_dir == dir {
        "/".to_string()  // Root directory, go back to root.
    } else {
        "../".to_string()  // Go one level up.
    };

    // HTML header and navigation links
    html.push_str(&format!(r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Directory Listing</title>
        <style>
            body {{ font-family: Arial, sans-serif; background-color: #f4f4f4; margin: 20px; }}
            h1 {{ color: #333; }}
            a {{ text-decoration: none; color: #007bff; }}
            a:hover {{ text-decoration: underline; }}
            ul {{ list-style-type: none; padding-left: 0; }}
            li {{ margin: 10px 0; }}
            .file {{ padding: 10px; background-color: #fff; border: 1px solid #ddd; border-radius: 5px; display: inline-block; }}
            hr {{ margin-top: 20px; border: none; height: 2px; background-color: #ddd; }}
        </style>
    </head>
    <body>
        <h1>Currently in {}</h1>
        <a href="{}" class="file">Go back to directory</a>
        <hr>
        <ul>
    "#, current_dir, parent_dir_link));

    // List all files and directories
    for entry in fs::read_dir(dir).unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();
            let file_name = entry.file_name().into_string().unwrap();
            let file_url = url_escape::encode_component(&file_name);

            // If it’s a directory, show folder icon, else show file icon
            if path.is_dir() {
                html.push_str(&format!(r#"<li><a href="{}/" class="file">📁 {}</a></li>"#, file_url, file_name));
            } else {
                html.push_str(&format!(r#"<li><a href="{}" class="file">📄 {}</a></li>"#, file_url, file_name));
            }
        }
    }

    html.push_str(r#"
        </ul>
    </body>
    </html>
    "#);

    html
}

// Serve the requested file with correct headers and binary/text differentiation
fn serve_file(path: &Path, stream: &mut std::net::TcpStream) -> io::Result<()> {
    let mime_type = get_mime_type(path);
    let mut file = BufReader::new(File::open(path)?);

    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    // HTTP response with appropriate headers
    let header = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
        mime_type,
        contents.len()
    );

    // Send headers followed by the actual file content
    stream.write_all(header.as_bytes())?;
    stream.write_all(&contents)?;

    Ok(())
}

// Get MIME type based on file extension
fn get_mime_type(path: &Path) -> &'static str {
    match path.extension().and_then(std::ffi::OsStr::to_str) {
        Some("html") => "text/html",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("webm") => "video/webm",
        Some("mp4") => "video/mp4",
        Some("txt") => "text/plain",
        Some("pdf") => "application/pdf",
        Some(_) => "application/octet-stream", // Fallback for any other file type.
        None => "application/octet-stream",    // Handle files without extension.
    }
}

// 404 Not Found page
fn not_found_page() -> String {
    r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>404 Not Found</title>
        <style>
            body { font-family: Arial, sans-serif; background-color: #f4f4f4; text-align: center; margin: 20px; }
            h1 { color: #e74c3c; font-size: 48px; }
            p { color: #333; font-size: 18px; }
        </style>
    </head>
    <body>
        <h1>404 - Not Found</h1>
        <p>The requested resource could not be found on this server.</p>
    </body>
    </html>
    "#.to_string()
}








