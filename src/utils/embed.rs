
// lib
    // serenity
use serenity::builder::{
    CreateEmbed,
    CreateEmbedAuthor,
    CreateEmbedFooter,
};

use serenity::client::Context;
use serenity::utils::Color;

// utils
use crate::utils::other::Utils;


#[derive(Clone)]
pub struct Embed<'a> {
    context: &'a Context,
    author: Option<CreateEmbedAuthor>,
    color: Option<Color>,
    description: Option<String>,
    footer: Option<CreateEmbedFooter>,
    image: Option<String>,
    thumbnail: Option<String>,
    title: Option<String>,
}

impl<'a> Embed<'a> {

    /// Returns a default embed
    pub fn new(context: &'a Context) -> Self {
        let mut embed = Embed {
            context,
            author: None,
            color: None,
            description: None,
            footer: None,
            image: None,
            thumbnail: None,
            title: None,
        };

        let bot_avatar = Utils::bot_avatar_url(context);
        let bot_name = Utils::bot_name(context);
        let bot_color = Utils::bot_color();

        embed.author(bot_avatar.clone(), bot_name.clone(), String::from(""));
        embed.footer(bot_avatar.clone(), bot_name.clone());
        embed.color(bot_color.0.clone(), bot_color.1.clone(), bot_color.2.clone());

        embed
    }

    /// Returns an instance of CreateEmbed according to the fields that had 
    /// been set
    pub fn create(&self) -> CreateEmbed {
        let mut embed = CreateEmbed::default();

        if let Some(author) = self.author.clone() {
            embed.set_author(author);
        }

        if let Some(color) = self.color.clone() {
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

        if let Some(title) = self.title.clone() {
            embed.title(title);
        }

        embed
    }

    /// Sets the embed's author
    /// 
    /// ## Arguments:
    /// * icon_url - the author's icon url
    /// * name - the author's name
    /// * url - the author's url
    pub fn author(&mut self, icon_url: String, name: String, url: String) -> Self {
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
        if description.len() > 4096 {
            panic!("Error in Embed::description: The description is too long.")
        }
        
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
    pub fn footer(&mut self, icon_url: String, text: String) -> Self {
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

    /// Sets the embed's title
    /// 
    /// ## Arguments:
    /// * title - the embed's title
    pub fn title(&mut self, title: String) -> Self {
        self.title = Some(title);

        self.clone()
    }

}
