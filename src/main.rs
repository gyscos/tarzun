extern crate zstd;
extern crate tar;

use std::io::{self, Write};
use std::sync::mpsc;
use std::thread;

fn uncompress(filename: &str) -> io::Result<(Vec<u8>, tar::Header)> {

        let file = std::fs::File::open(&filename)?;

        let mut headers = tar::Header::new_gnu();
        headers.set_path(filename.trim_right_matches(".zst"))?;
        headers.set_metadata(&file.metadata()?);

        let uncompressed = zstd::stream::decode_all(file)?;
        headers.set_size(uncompressed.len() as u64);

        headers.set_cksum();

        Ok((uncompressed, headers))
}

fn make_tar<W: Write>(files: Vec<String>, out: W) -> io::Result<()> {
    let mut builder = tar::Builder::new(out);

    let (tx, rx) = mpsc::sync_channel(3);

    thread::spawn(move || {
        for filename in files {
            tx.send(uncompress(&filename).unwrap()).unwrap();
        }
    });

    for (item, mut headers) in rx {
        builder.append(
            &mut headers,
            item.as_slice(),
        )?;
    }
    builder.finish()
}


fn main() {
    let filenames = std::env::args().skip(1).collect();
    make_tar(filenames, io::stdout()).unwrap();
}
