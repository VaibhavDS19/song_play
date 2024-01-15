use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;
use std::{env, fs};

struct SinkInst {
    current: usize,
    sink: Sink,
    stream_handle: OutputStreamHandle,
    queue: VecDeque<String>,
}

impl SinkInst {
    fn new(stream_handle: &OutputStreamHandle) -> SinkInst {
        let sink = Sink::try_new(stream_handle).unwrap();
        SinkInst {
            current: 0,
            sink,
            stream_handle: stream_handle.to_owned(),
            queue: VecDeque::new(),
        }
    }

    fn add_to_queue(&mut self, song: String) {
        self.queue.push_back(song);
    }

    fn play(&mut self) {
        self.sink = Sink::try_new(&self.stream_handle).unwrap();
        if !self.queue.is_empty() {
            // Decode that sound file into a source
            let file_name = self.queue.get(self.current).unwrap();
            println!("Now playing: {:?}", &file_name);
            let source = SinkInst::make_source(file_name);
            self.sink.append(source);
            self.sink.sleep_until_end();
        } else {
            println! {"No files found in playlist. Please add some files to play."};
        }
    }

    fn make_source(file_name: &String) -> Decoder<BufReader<File>> {
        // Load a sound from a file, using a path relative to Cargo.toml or using an absolute path
        let file = BufReader::new(File::open(file_name).unwrap());
        Decoder::new(file).unwrap()
    }

    fn repeat_all(&mut self) {
        loop {
            self.play();
            self.current = (self.current + 1) % self.queue.len();
        }
    }

    fn repeat_current(&mut self) {
        loop {
            self.play();
        }
    }
}

fn main() {
    // if user specifies file and the file has content, play the files in the user file
    let args: Vec<String> = env::args().collect();
    let mut path: String = String::from("");
    if args.len() > 1 {
        path = args[1].clone();
    }

    // create an output stream
    // the output stream has stream and a stream_handle
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut sink_inst = SinkInst::new(&stream_handle);
    let mut flag = true;
    let source_path = "/home/v19/Desktop/own utilities/song_downloader/Songs/";
    let song: String;

    if path.len() > 1 {
        if path.ends_with("mp3") || path.ends_with("aac") {
            sink_inst.add_to_queue(path);
            sink_inst.repeat_current();
            flag = false;
        } else {
            let files = fs::read_dir(&path);
            if !path.ends_with("/") {
                path += "/";
            }
            match files {
                Ok(paths) => {
                    flag = false;
                    paths.for_each(|file| {
                        sink_inst.add_to_queue(
                            path.to_owned() + file.unwrap().file_name().to_str().unwrap(),
                        )
                    });
                    sink_inst.repeat_all();
                }
                Err(_) => {
                    flag = true;
                }
            }
        }
    }

    if flag {
        song = source_path.to_owned() + "Alan Jackson - Remember When.mp3";
        sink_inst.add_to_queue(song.to_owned());
        sink_inst.play();
    }

    // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // let sink = Sink::try_new(&stream_handle).unwrap();
    // let source = SinkInst::make_source(song.to_owned());
    // sink.append(source);
    // sink.play();
    loop {}
    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    // sink.sleep_until_end();
}
