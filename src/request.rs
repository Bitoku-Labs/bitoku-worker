/*
  Copyright 2023 Bitoku Labs

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use bitoku_sdk_agent_native::instruction::{unpack_request, Request};
use std::fs::{self, remove_file, File, OpenOptions};
use std::io::Seek;
use std::io::Write;
use std::path::Path;
use std::str;

pub fn process_request(data: Vec<u8>) -> Result<String, anyhow::Error> {
    let client = data.first().unwrap();
    let req_bytes = &data[33..675];
    let req = unpack_request(req_bytes)?;

    let path_string = format!("client-{}", client);
    let path = Path::new(&path_string);

    match req {
        Request::CreateBucket { name } => {
            println!("create bucket");
            let bucket_name = decode_name(name)?;
            let new_path = path.join(bucket_name);

            if new_path.is_dir() {
                println!("directory already exists");
            }
            fs::create_dir_all(new_path).unwrap();
        }
        Request::CreateFile { name, data } => {
            let file_name = decode_name(name)?;
            let new_path = path.join(file_name);

            if new_path.is_file() {
                println!("directory already exists");
            }

            let non_zero_data = get_non_zeros(&data)?;
            let mut file = File::create(new_path).unwrap();

            file.write_all(&non_zero_data.as_slice()).unwrap();
        }
        Request::WriteFile {
            name,
            file_id,
            data,
        } => {
            let file_name = decode_name(name)?;
            let new_path = path.join(file_name);

            let non_zero_data = get_non_zeros(&data)?;
            let mut file = OpenOptions::new().append(true).open(&new_path).unwrap();

            file.write_all(&non_zero_data.as_slice()).unwrap();
        }
        Request::DeleteFile { name, file_id } => {
            println!("delete file");

            let file_name = decode_name(name)?;
            let new_path = path.join(file_name);

            if new_path.is_file() {
                remove_file(path).unwrap();
            }
            println!("directory already exists");
        }
        Request::SetPosition {
            name,
            file_id,
            position,
        } => {
            println!("set position");
            let file_name = decode_name(name)?;
            let path = Path::new(&file_name);

            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(path)
                .unwrap();

            file.seek(std::io::SeekFrom::Start(position)).unwrap();
            println!(
                "file seek position is {:?}",
                file.stream_position().unwrap()
            );
            let data = String::from(" writing seeked data");
            file.write_all(data.as_bytes()).unwrap();
        }
        Request::CloseFile { name, file_id } => {
            println!("close file");
        }
        Request::OpenFile { name, file_id } => {
            println!("open file");
        }
        Request::ReadFile { name, file_id } => {
            println!("read file");
        }
    }
    Ok(String::from("success"))
}

pub fn decode_name(name: [u8; 128]) -> Result<String, anyhow::Error> {
    let non_zero_bytes = get_non_zeros(&name)?;

    let output = str::from_utf8(&non_zero_bytes)?;
    Ok(output.to_string())
}

pub fn get_non_zeros(data: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
    let non_zeros = data.iter().take_while(|&b| *b != 0).copied().collect();

    Ok(non_zeros)
}
