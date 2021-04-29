use std::{fs, path::Path, io::Write};
use serde::Deserialize;
use toml;
use rss::{ItemBuilder, ChannelBuilder};
use crate::metadata::Metadata;

#[derive(Deserialize)]
pub struct RssConfig {
    title:       String,
    url:         String,
    description: String,
}

impl RssConfig {
    pub fn new(file: &Path) -> Option<RssConfig> {
        let contents = fs::read_to_string(file).ok()?;
        let config = toml::from_str(&contents)
            .expect("RSS exists, but not valid");
        return Some(config);
    }

    pub fn write_feed(&self, root: &Path, mut metadata: Vec<Metadata>) {
        metadata.sort();

        let mut items = vec![];
        for raw_item in metadata {
            let item = ItemBuilder::default()
                .title(raw_item.title)
                .link(format!("{}/{}.html", self.url, raw_item.link))
                .pub_date(raw_item.published)
                .build()
                .expect("Could not build RSS item");
            items.push(item);
        }

        let channel = ChannelBuilder::default()
            .title(self.title.clone())
            .link(self.url.clone())
            .description(self.description.clone())
            .generator("RSS Dumped by Dump".to_string())
            .items(items)
            .build()
            .expect("Could not build RSS channel");

        let output = fs::File::create(root.join("dump.rss"))
            .expect("Could not create RSS output file");
        channel.pretty_write_to(output, b' ', 2)
            .expect("Could not write RSS feed to file");
    }
}
