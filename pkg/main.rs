//! iqra-pkg: أداة سطر أوامر لإدارة مكتبات اقرأ
//! عبقري، ثنائي اللغة، يدعم جميع أوامر الحزم الأساسية

use std::fs;
use std::env;
use std::process;
use std::path::Path;
use std::collections::HashMap;
use reqwest::blocking::Client;
// ...existing code...
use std::io::Write;

fn print_usage() {
    println!("\nأوامر مدير الحزم العبقري (iqra-pkg):");
    println!("  iqra-pkg install <package>     # تثبيت مكتبة");
    println!("  iqra-pkg update                # تحديث جميع المكتبات");
    println!("  iqra-pkg publish <package>     # نشر مكتبتك");
    println!("  iqra-pkg remove <package>      # إزالة مكتبة");
    println!("  iqra-pkg search <query>        # البحث عن مكتبة");
    println!("  iqra-pkg help                  # عرض المساعدة");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }
    let pkg_dir = "iqra_packages";
    let manifest_path = "iqra-pkg.json";
    // Read registry URL from environment variable for testing/mocking
    let registry_url = env::var("MOCKITO_SERVER_URL")
        .or_else(|_| env::var("IQRA_REGISTRY_URL"))
        .unwrap_or_else(|_| "https://iqra-registry.example.com".to_string());
    let client = Client::new();

    // Helper: load manifest
    fn load_manifest(path: &str) -> HashMap<String, String> {
        if Path::new(path).exists() {
            let data = fs::read_to_string(path).unwrap_or_default();
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            HashMap::new()
        }
    }
    // Helper: save manifest
    fn save_manifest(path: &str, manifest: &HashMap<String, String>) {
        let data = serde_json::to_string_pretty(manifest).unwrap();
        fs::write(path, data).unwrap();
    }

    match args[1].as_str() {
        "install" | "ثبت" => {
            if args.len() < 3 {
                println!("يرجى تحديد اسم المكتبة | Please specify a package name");
                process::exit(1);
            }
            let pkg = &args[2];
            println!("تثبيت المكتبة '{}' من السجل ... | Installing package '{}' from registry ...", pkg, pkg);
            fs::create_dir_all(pkg_dir).ok();
            let pkg_path = format!("{}/{}", pkg_dir, pkg);
            if Path::new(&pkg_path).exists() {
                println!("المكتبة مثبتة بالفعل | Package already installed");
                process::exit(0);
            }
            // Download from registry
            let url = format!("{}/packages/{}/latest/download", registry_url, pkg);
            match client.get(&url).send() {
                Ok(resp) => {
                    if resp.status().is_success() {
                        let mut file = fs::File::create(&pkg_path).unwrap();
                        let bytes = resp.bytes().unwrap();
                        file.write_all(&bytes).unwrap();
                        let mut manifest = load_manifest(manifest_path);
                        manifest.insert(pkg.clone(), "latest".to_string());
                        save_manifest(manifest_path, &manifest);
                        println!("تم التثبيت بنجاح | Installed successfully");
                    } else {
                        println!("فشل التحميل من السجل | Failed to download from registry");
                    }
                }
                Err(_) => println!("خطأ في الاتصال بالسجل | Registry connection error"),
            }
        }
        "update" | "حدث" => {
                println!("تحديث جميع المكتبات من السجل ... | Updating all packages from registry ...");
                let manifest = load_manifest(manifest_path);
                if manifest.is_empty() {
                    println!("لا توجد مكتبات مثبتة | No packages installed");
                    process::exit(0);
                }
                for (pkg, _ver) in manifest.iter() {
                    println!("تحديث '{}' ... | Updating '{}' ...", pkg, pkg);
                    let url = format!("{}/packages/{}/latest/download", registry_url, pkg);
                    match client.get(&url).send() {
                        Ok(resp) => {
                            if resp.status().is_success() {
                                let pkg_path = format!("{}/{}", pkg_dir, pkg);
                                let mut file = fs::File::create(&pkg_path).unwrap();
                                let bytes = resp.bytes().unwrap();
                                file.write_all(&bytes).unwrap();
                            } else {
                                println!("فشل تحديث '{}' | Failed to update '{}'", pkg, pkg);
                            }
                        }
                        Err(_) => println!("خطأ في الاتصال بالسجل | Registry connection error"),
                    }
                }
                println!("تم التحديث بنجاح | All packages updated successfully");
        }
        "publish" | "انشر" => {
                if args.len() < 3 {
                    println!("يرجى تحديد اسم المكتبة للنشر | Please specify a package to publish");
                    process::exit(1);
                }
                let pkg = &args[2];
                let pkg_path = format!("{}/{}", pkg_dir, pkg);
                if !Path::new(&pkg_path).exists() {
                    println!("المكتبة غير موجودة محلياً | Package not found locally");
                    process::exit(1);
                }
                println!("نشر المكتبة '{}' إلى السجل ... | Publishing package '{}' to registry ...", pkg, pkg);
                // TODO: Add authentication (token)
                let url = format!("{}/packages", registry_url);
                let file_bytes = fs::read(&pkg_path).unwrap();
                let metadata = serde_json::json!({
                    "name": pkg,
                    "version": "1.0.0", // TODO: Read from manifest
                    "author": "TODO", // TODO: Get author
                    "description": "TODO", // TODO: Get description
                    "lang": "ar"
                });
                let form = reqwest::blocking::multipart::Form::new()
                    .text("metadata", metadata.to_string())
                    .part("archive", reqwest::blocking::multipart::Part::bytes(file_bytes).file_name(pkg.clone()));
                match client.post(&url).multipart(form).send() {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            println!("تم النشر بنجاح | Published successfully");
                        } else {
                            println!("فشل النشر | Publish failed: {}", resp.status());
                        }
                    }
                    Err(_) => println!("خطأ في الاتصال بالسجل | Registry connection error"),
                }
        }
        "remove" | "احذف" => {
                if args.len() < 3 {
                    println!("يرجى تحديد اسم المكتبة للإزالة | Please specify a package to remove");
                    process::exit(1);
                }
                let pkg = &args[2];
                let pkg_path = format!("{}/{}", pkg_dir, pkg);
                if !Path::new(&pkg_path).exists() {
                    println!("المكتبة غير مثبتة | Package not installed");
                    process::exit(1);
                }
                // Remove locally
                fs::remove_file(&pkg_path).unwrap();
                let mut manifest = load_manifest(manifest_path);
                manifest.remove(pkg);
                save_manifest(manifest_path, &manifest);
                println!("تمت الإزالة محلياً | Removed locally");
                // Remove from registry (if owner)
                // TODO: Add authentication (token)
                let url = format!("{}/packages/{}/1.0.0", registry_url, pkg); // TODO: Use real version
                match client.delete(&url).send() {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            println!("تمت الإزالة من السجل | Removed from registry");
                        } else {
                            println!("فشل الإزالة من السجل | Failed to remove from registry: {}", resp.status());
                        }
                    }
                    Err(_) => println!("خطأ في الاتصال بالسجل | Registry connection error"),
                }
        }
        "search" | "ابحث" => {
                if args.len() < 3 {
                    println!("يرجى تحديد كلمة البحث | Please specify a search query");
                    process::exit(1);
                }
                let query = &args[2];
                println!("البحث في السجل عن '{}' ... | Searching registry for '{}' ...", query, query);
                let url = format!("{}/packages?q={}", registry_url, query);
                match client.get(&url).send() {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            let results: serde_json::Value = resp.json().unwrap_or_default();
                            println!("نتائج البحث | Search results:");
                            if let Some(arr) = results.as_array() {
                                for r in arr {
                                    let name = r.get("name").and_then(|v| v.as_str()).unwrap_or("");
                                    let desc = r.get("description").and_then(|v| v.as_str()).unwrap_or("");
                                    println!("  - {}: {}", name, desc);
                                }
                            } else {
                                println!("لا توجد نتائج | No results found");
                            }
                        } else {
                            println!("فشل البحث في السجل | Registry search failed: {}", resp.status());
                        }
                    }
                    Err(_) => println!("خطأ في الاتصال بالسجل | Registry connection error"),
                }
        }
        "help" | "مساعدة" => {
            print_usage();
        }
        _ => {
            println!("أمر غير معروف | Unknown command");
            print_usage();
            process::exit(1);
        }
    }
}
