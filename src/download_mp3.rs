use std::{fs, fs::DirEntry, io, io::Write, path::PathBuf, process::Command};

fn create_youtube_dl_command(output_path: &String, url: &String) -> Command {
    let mut command = Command::new("youtube-dl");

    command
        .arg("-o")
        .arg(output_path)
        .arg("--format")
        .arg("mp4")
        .arg(url);

    command
}

fn create_ffmpeg_command(input_path: &String, output_path: &String) -> Command {
    let mut command = Command::new("ffmpeg");

    command
        .arg("-i")
        .arg(input_path)
        .arg("-b:a")
        .arg("320K")
        .arg("-vn")
        .arg(output_path);

    command
}

macro_rules! slugify {
    ($s:expr) => {
        $s.replace(' ', "").replace("./", "").replace('&', "")
    };
}

fn create_mp3_path(tmp_dir: &PathBuf, mp4_path: &String) -> String {
    let mut mp4_path_split = mp4_path
        .split('/')
        .last()
        .unwrap()
        .split('.')
        .collect::<Vec<&str>>();

    if mp4_path_split.len() < 2 {
        panic!("Improper downloaded path format.");
    }

    mp4_path_split.pop();

    let mut mp3_path = tmp_dir
        .clone()
        .join(mp4_path_split.into_iter().collect::<String>());

    mp3_path.set_extension("mp3");

    slugify!(mp3_path.into_os_string().into_string().unwrap())
}

pub fn download_mp3(tmp_dir: &PathBuf, url: &String) -> Result<String, std::io::Error> {
    print!("Downloading... ");

    io::stdout().flush().unwrap();

    {
        let mut cmd = create_youtube_dl_command(
            &tmp_dir
                .join("%(title)s.%(ext)s")
                .into_os_string()
                .into_string()
                .unwrap(),
            url,
        );

        cmd.output().expect("Failed to download link.");
    }

    println!("Complete.");
    print!("Converting... ");

    io::stdout().flush().unwrap();

    let mp4_path = fs::read_dir(&tmp_dir)
        .unwrap()
        .into_iter()
        .map(|x| x.unwrap())
        .collect::<Vec<DirEntry>>()[0]
        .path()
        .into_os_string()
        .into_string()
        .unwrap();
    let mp3_path = create_mp3_path(&tmp_dir, &mp4_path);

    {
        let mut cmd = create_ffmpeg_command(&mp4_path, &mp3_path);

        cmd.output().expect("Failed to convert file to mp3.");
    }

    fs::remove_file(&mp4_path).unwrap();

    println!("Complete.");

    Ok(mp3_path)
}
