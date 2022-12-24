use crate::hub::Hub;
use bytes::Bytes;
use flate2::read::GzDecoder;
use std::{env, fs, os::unix::fs::chroot, path::Path, process::Command};
use tar::Archive;
use tempdir::TempDir;

pub struct Image {
    pub name: String,
    pub revision: String,
    pub layers: Option<Vec<Bytes>>,
}

impl Image {
    pub fn new(image: &str) -> Self {
        let (name, revision) = image.split_once(':').map_or((image, "latest"), |v| v);

        Self {
            layers: None,
            name: name.to_string(),
            revision: revision.to_string(),
        }
    }

    pub async fn get_layers(&mut self, hub: &Hub) {
        let result = hub.get_manifest(&self.name, &self.revision).await;

        let mut layers: Vec<Bytes> = Vec::new();
        let mut futures = Vec::new();
        for layer in result["fsLayers"].as_array().unwrap() {
            let layer = layer["blobSum"].as_str().unwrap();

            futures.push(hub.get_layer(&self.name, layer));
        }

        for future in futures {
            layers.push(future.await);
        }

        self.layers = Some(layers);
    }

    pub fn run(&self, command: &mut Command) -> std::process::ExitStatus {
        let root = TempDir::new("sandbox").unwrap();

        for layer in self.layers.as_ref().unwrap() {
            Archive::new(GzDecoder::new(layer.as_ref()))
                .unpack(root.path())
                .expect("Failed to extract layer");
        }

        chroot(root.path()).expect("could not chroot into sandbox");
        env::set_current_dir("/").expect("could not navigate to root");

        #[cfg(target_os = "linux")]
        unsafe {
            libc::unshare(libc::CLONE_NEWPID);
        }

        // If /dev/null doesn't exist, Rust's Command implementation will fail to spawn the process.
        if !Path::new("/dev/").is_dir() {
            fs::create_dir("/dev/").unwrap();
        }
        fs::File::create("/dev/null").unwrap();

        let mut child_process = command.spawn().unwrap();
        child_process.wait().unwrap()
    }
}
