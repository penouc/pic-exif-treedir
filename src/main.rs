use exif::{Tag,Value, In};
use exif::DateTime;
use std::fs;
use std::path::Path;

fn get_exif_time_info(dir_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(dir_path);

    // 遍历文件夹中的所有文件
    for entry in fs::read_dir(path)? {
        let path = entry?.path();
        let file_type = path.extension().unwrap();

        // 只处理 jpeg 和 heic 格式的文件
        if file_type == "jpeg" || file_type == "jpg" || file_type == "heic" {
            // 读取文件并解析 exif 信息
            let file = std::fs::File::open(path)?;
            let mut bufreader = std::io::BufReader::new(&file);
            let exifreader = exif::Reader::new();
            // let exif_data = exifreader.read_from_container(&mut bufreader)?;

            match exifreader.read_from_container(&mut bufreader) {
                Ok(exif_data) => {
                    match exif_data.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
                        Some(date_time_original) => {
                            
                            println!("获取 exif 信息成功：{:?}", date_time_original.value);
                            // match &date_time_original.value {
                            //     Ascii(datetime_val) => {
                            //         // let dt  = DateTime::from_ascii(dt_val)?;

                            //         println!("获取 exif 信息成功：{:?}", datetime_val);
                            //     },
                            //     _ => {
                            //         println!("获取 exif 信息为其他");
                            //     }
                            // }
                            // let dt  = DateTime::from_ascii(date_time_original.value.Ascii)?;
                            // println!("获取 exif 信息成功：{:?}", dt);
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
    let resu = get_exif_time_info("./pics");
    println!("{:?}", resu);
}