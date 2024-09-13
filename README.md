## Simple File Server


The objective of this bounty is to create a simple file server that can serve files as a simple HTML document listing all directories and folders as links.


**You will be able to:**

* **Utilise the use of the path crate** 
* **Create your own local file server** 

### **Getting Started :**

**1. Install Rust:**  **Prerequisites:**  You'll need Rust and Cargo installed on your system. 
- Check Rust's version:  `rustc --version`

- **Install Rust using official installer:**  
	Link:  [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)  

**2. Clone Simple-File-Server:**
```
git clone https://github.com/Agihtaws/SIMPLE-FILE-SERVER.git

cd SIMPLE-FILE-SERVER
```

**3. Build Simple-File-Server:**

```
cargo build
```

**4. Run Simple-File-Server:**

```
target/debug/simple-file-server
	or
cargo run
```

### **Usage Instructions:**

1. **Accessing the File Server:**
   Open your web browser and navigate to `http://localhost:8080`. You should see a directory listing of the current directory.

2. **Navigating Directories:**
   Click on directories to view their contents. Files and folders will be listed as clickable links.
   

### **Features:**

- **Directory Listing:** Automatically generates an HTML page listing all files and directories in the current directory.
  
- **Link Navigation:** Allows users to navigate through directories by clicking links.
  
- **File Serving:** Serves files directly for download or viewing.


### **Example Output:**

When you run the file server and access it via a web browser, you will see an HTML page like this:

![base64_tool](https://github.com/Agihtaws/SIMPLE-FILE-SERVER/blob/master/Screenshot_20240914_005923.png)


### **Video Demonstration:**

Watch a demonstration of the Simple File Server in action:

[![Watch the video](https://img.youtube.com/vi/YOUR_VIDEO_ID/0.jpg)](https://www.youtube.com/watch?v=YOUR_VIDEO_ID)


