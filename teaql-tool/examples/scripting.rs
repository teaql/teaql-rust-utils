use teaql_tool::T;

fn main() {
    println!("=== TeaQL Tool Scripting Examples ===");

    // 1. Shell Command Execution (with timeout)
    println!("1. Running Shell Command:");
    match T::cmd().run_with_timeout("echo Hello From CmdTool", 2) {
        Ok((stdout, stderr, code)) => println!("   Result: {} (Code: {})", stdout.trim(), code),
        Err(e) => println!("   Error: {:?}", e),
    }

    // 2. HTML Parsing & Scraping
    println!("2. Parsing HTML:");
    let html = r#"<div class="card"><a href="https://teaql.org">Link 1</a><a href="https://rust-lang.org">Link 2</a></div>"#;
    if let Ok(links) = T::html().select_attr(html, "a", "href") {
        println!("   Found links: {:?}", links);
    }

    // 3. Zip / Archive operations
    println!("3. Zipping files (Dry Run):");
    // T::archive().zip_dir("./src", "src_backup.zip").unwrap();
    println!("   T::archive().zip_dir(\"./src\", \"src_backup.zip\").unwrap();");

    // 4. Local DB (KV)
    println!("4. Sled KV DB:");
    if let Ok(db) = T::kv().open(".local_test.db") {
        let _ = db.insert("app_name", "TeaQL Script");
        if let Ok(Some(val)) = db.get("app_name") {
            println!("   Read from DB: {}", String::from_utf8_lossy(&val));
        }
        let _ = std::fs::remove_dir_all(".local_test.db"); // clean up
    }

    // 5. Chinese Pinyin
    println!("5. Pinyin Conversion:");
    let pinyin = T::pinyin().get_pinyin("你好世界");
    println!("   '你好世界' -> '{}'", pinyin);

    // 6. Barcode & QRCode (Generation)
    println!("6. Generating SVG Barcode:");
    if let Ok(svg) = T::barcode().generate_code128_svg("TEAQL-2026") {
        println!("   Generated SVG (first 30 chars): {}...", &svg[..30]);
    }

    // 7. Watcher (Uncomment to use)
    println!("7. File System Watcher:");
    println!("   T::watcher().watch(\"./\", |path| println!(\"Changed: {{}}\", path)).unwrap();");

    // 8. Static File Server (Uncomment to use)
    println!("8. Static Server:");
    println!("   T::server().serve_dir(\"./\", 8080).unwrap();");
    
    // 9. Proxy Pass (Uncomment to use)
    println!("9. Proxy Server:");
    println!("   T::proxy().start(8081, \"http://127.0.0.1:8080\").unwrap();");

    println!("=====================================");
}
