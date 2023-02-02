use exif::{Tag, In};
use std::{fs, env};
use std::path::Path;
use chrono::{NaiveDateTime};

fn create_dir_if_not_exists(dir_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(dir_path);

    // 判断文件夹是否存在
    if !path.exists() {
        // 新建文件夹
        fs::create_dir_all(path)?;
    }

    Ok(())
}

fn get_exif_time_info(dir_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(dir_path);

    // 遍历文件夹中的所有文件
    for entry in fs::read_dir(path)? {
        let path = entry?.path();
        let file_type = path.extension().unwrap();

        // 只处理 jpeg 和 heic 格式的文件
        if file_type == "jpeg" || file_type == "jpg" || file_type == "heic" {
            // 读取文件并解析 exif 信息
            let file_pwd_path = path.clone();
            let file = std::fs::File::open(path)?;
            let mut bufreader = std::io::BufReader::new(&file);
            let exifreader = exif::Reader::new();
            // let exif_data = exifreader.read_from_container(&mut bufreader)?;

            match exifreader.read_from_container(&mut bufreader) {
                Ok(exif_data) => {
                    match exif_data.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
                        Some(date_time_original) => {
                            
                            let dt_str = date_time_original.value.display_as(Tag::DateTimeOriginal).to_string();
                            println!("获取 exif 信息成功：{:?}", dt_str);
                            let no_timezone = NaiveDateTime::parse_from_str(&dt_str, "%Y-%m-%d %H:%M:%S")?;
                            let year = no_timezone.format("%Y");
                            let month = no_timezone.format("%m");
                            let day = no_timezone.format("%d");
                            println!("{}-{}-{}", year, month, day);
                            let current_dir = env::current_dir()?;
                            println!(
                                "Entries modified in the last 24 hours in {:?}:",
                                current_dir
                            );
                            let new_path = format!("{}/{}/{}-{}", current_dir.display(), year, month, day);
                            println!("new_path {}", new_path);
                            let create_result = create_dir_if_not_exists(&new_path);
                            match create_result {
                                Ok(_) => {
                                    let file_name = file_pwd_path.file_name().unwrap().to_str();
                                    match file_name {
                                        Some(file_name) => {
                                            let new_file_path = format!("{}/{}", new_path, file_name);
                                            println!("输出新文件路径：{:?}", new_file_path);
                                            fs::rename(file_pwd_path, new_file_path)?;
                                        },
                                        None => {
                                            println!("输出新文件路径错误");
                                        }
                                    }
                                },
                                Err(err) => {
                                    println!("{:?}", err);
                                }
                            }
                        },
                        None => eprintln!("DateTimeOriginal tag is missing"),
                    }
                },
                Err(err) => {
                    println!("获取 exif 信息失败： {:?}", err)
                }
            }
        }
    }

    Ok(())
}


fn main() {
    
    for argument in env::args() {
        get_exif_time_info(&argument);
    }
    // println!("{:?}", resu);
}
