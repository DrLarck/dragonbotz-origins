
// lib
    // serenity
use serenity::builder::{
    CreateEmbed,
    CreateEmbedAuthor,
    CreateEmbedFooter,
};

use serenity::utils::Color;


#[derive(Clone)]
pub struct Embed {
    author: Option<CreateEmbedAuthor>,
    color: Option<Color>,
    description: Option<String>,
    footer: Option<CreateEmbedFooter>,
    image: Option<String>,
    thumbnail: Option<String>,
}

impl Embed {

    /// Returns an instance of CreateEmbed according to the fields that had 
    /// been set
    pub fn create(&self) -> CreateEmbed {
        let mut embed = CreateEmbed::default();

        if let Some(author) = self.author.clone() {
            embed.set_author(author);
        }

        if let Some(color) = self.color {
            embed.color(color);
        }

        if let Some(description) = self.description.clone() {
            embed.description(description);
        }

        if let Some(footer) = self.footer.clone() {
            embed.set_footer(footer);
        }

        if let Some(image) = self.image.clone() {
            embed.image(image);
        }

        if let Some(thumbnail) = self.thumbnail.clone() {
            embed.thumbnail(thumbnail);
        }

        embed
    }

    /// Sets the embed's author
    /// 
    /// ## Arguments:
    /// * icon_url - the author's icon url
    /// * name - the author's name
    /// * url - the author's url
    pub fn author(&mut self, icon_url: &str, name: &str, url: &str) -> Self {
        let mut author = CreateEmbedAuthor::default();

        author.icon_url(icon_url);
        author.name(name);
        author.url(url);

        self.author = Some(author);

        self.clone()
    }

    /// Sets the embed's color
    /// 
    /// ## Arguments:
    /// * red - the red value
    /// * green - the green value
    /// * blue - the blue value
    pub fn color(&mut self, red: u8, green: u8, blue: u8) -> Self {
        self.color = Some(Color::from_rgb(red, green, blue));

        self.clone()
    }

    /// Sets the embed's description
    /// 
    /// ## Arguments:
    /// * description - the description
    pub fn description(&mut self, description: String) -> Self {
        self.description = Some(description);

        self.clone()
    }

    /// Adds a field to the current embed
    /// 
    /// ## Arguments:
    /// * name - the field's name
    /// * value - the field's value
    /// * inline - tells if the field is inline or not
    pub fn add_field(&self, name: String, value: String, inline: bool) -> CreateEmbed {
        if name.len() > 256 || value.len() > 1024 {
            panic!("Error in Embed::add_field: The name or value length is too long.")
        }

        let mut embed = self.create();
        embed.field(name, value, inline);

        embed
    }

    /// Sets the embed's footer
    /// 
    /// ## Arguments:
    /// * icon_url - the footer's icon url
    /// * text - the footer's text
    pub fn footer(&mut self, icon_url: &str, text: &str) -> Self {
        let mut footer = CreateEmbedFooter::default();

        footer.icon_url(icon_url);
        footer.text(text);

        self.footer = Some(footer);

        self.clone()
    }

    /// Sets the embed's image url
    /// 
    /// ## Arguments:
    /// * url - the image url
    pub fn image(&mut self, url: String) -> Self {
        self.image = Some(url);

        self.clone()
    }

    /// Sets the embed's thumbnail url
    /// 
    /// ## Arguments:
    /// * url - the thumbnail url
    pub fn thumbnail(&mut self, url: String) -> Self {
        self.thumbnail = Some(url);

        self.clone()
    }

}
