//! ## Task Description
//!
//! The goal is to develop a backend service for shortening URLs using CQRS
//! (Command Query Responsibility Segregation) and ES (Event Sourcing)
//! approaches. The service should support the following features:
//!
//! ## Functional Requirements
//!
//! ### Creating a short link with a random slug
//!
//! The user sends a long URL, and the service returns a shortened URL with a
//! random slug.
//!
//! ### Creating a short link with a predefined slug
//!
//! The user sends a long URL along with a predefined slug, and the service
//! checks if the slug is unique. If it is unique, the service creates the short
//! link.
//!
//! ### Counting the number of redirects for the link
//!
//! - Every time a user accesses the short link, the click count should
//!   increment.
//! - The click count can be retrieved via an API.
//!
//! ### CQRS+ES Architecture
//!
//! CQRS: Commands (creating links, updating click count) are separated from
//! queries (retrieving link information).
//!
//! Event Sourcing: All state changes (link creation, click count update) must be
//! recorded as events, which can be replayed to reconstruct the system's state.
//!
//! ### Technical Requirements
//!
//! - The service must be built using CQRS and Event Sourcing approaches.
//! - The service must be possible to run in Rust Playground (so no database like
//!   Postgres is allowed)
//! - Public API already written for this task must not be changed (any change to
//!   the public API items must be considered as breaking change).

#![allow(unused_variables, dead_code)]

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use crate::commands::CommandHandler;
use crate::queries::QueryHandler;

/// All possible errors of the [`UrlShortenerService`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ShortenerError {
    /// This error occurs when an invalid [`Url`] is provided for shortening.
    InvalidUrl,

    /// This error occurs when an attempt is made to use a slug (custom alias)
    /// that already exists.
    SlugAlreadyInUse,

    /// This error occurs when the provided [`Slug`] does not map to any existing
    /// short link.
    SlugNotFound,
}

/// A unique string (or alias) that represents the shortened version of the
/// URL.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Slug(pub String);

/// The original URL that the short link points to.
#[derive(Clone, Debug, PartialEq)]
pub struct Url(pub String);

/// Shortened URL representation.
#[derive(Debug, Clone, PartialEq)]
pub struct ShortLink {
    /// A unique string (or alias) that represents the shortened version of the
    /// URL.
    pub slug: Slug,

    /// The original URL that the short link points to.
    pub url: Url,
}

/// Statistics of the [`ShortLink`].
#[derive(Debug, Clone, PartialEq)]
pub struct Stats {
    /// [`ShortLink`] to which this [`Stats`] are related.
    pub link: ShortLink,

    /// Count of redirects of the [`ShortLink`].
    pub redirects: u64,
}

/// Commands for CQRS.
pub mod commands {
    use super::{ShortLink, ShortenerError, Slug, Url};

    /// Trait for command handlers.
    pub trait CommandHandler {
        /// Creates a new short link. It accepts the original url and an
        /// optional [`Slug`]. If a [`Slug`] is not provided, the service will generate
        /// one. Returns the newly created [`ShortLink`].
        ///
        /// ## Errors
        ///
        /// See [`ShortenerError`].
        fn handle_create_short_link(
            &mut self,
            url: Url,
            slug: Option<Slug>,
        ) -> Result<ShortLink, ShortenerError>;

        /// Processes a redirection by [`Slug`], returning the associated
        /// [`ShortLink`] or a [`ShortenerError`].
        fn handle_redirect(
            &mut self,
            slug: Slug,
        ) -> Result<ShortLink, ShortenerError>;

        /// Changes the URL of a [`ShortLink`] with a provided [`Slug`].
        fn handle_change_short_link(
            &mut self,
            slug: Slug,
            new_url: Url,
        ) -> Result<ShortLink, ShortenerError>;
    }
}

/// Queries for CQRS
pub mod queries {
    use super::{ShortenerError, Slug, Stats};

    /// Trait for query handlers.
    pub trait QueryHandler {
        /// Returns the [`Stats`] for a specific [`ShortLink`], such as the
        /// number of redirects (clicks).
        ///
        /// [`ShortLink`]: super::ShortLink
        fn get_stats(&self, slug: Slug) -> Result<Stats, ShortenerError>;
    }
}

/// CQRS and Event Sourcing-based service implementation
pub struct UrlShortenerService {
    // TODO: add needed fields
    links: HashMap<Slug, (ShortLink, u64)>, // Stores the ShortLink and the count of redirects
}

impl UrlShortenerService {
    /// Creates a new instance of the service
    pub fn new() -> Self {
        Self {
            links: HashMap::new(),
        }
    }
    
    /// Generates a random slug (basic version)
    fn generate_random_slug() -> Slug {
        use rand::{distributions::Alphanumeric, Rng}; // Ensure import is in the function scope
        let slug: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6) // Adjust the length as needed
            .map(char::from)
            .collect();
        Slug(slug)
    }
}

impl commands::CommandHandler for UrlShortenerService {
    fn handle_create_short_link(
        &mut self,
        url: Url,
        slug: Option<Slug>,
    ) -> Result<ShortLink, ShortenerError> {
        // todo!("Implement the logic for creating a short link")

        let slug = match slug {
            Some(s) => s,
            None => Self::generate_random_slug(),
        };

        // Check if the slug already exists
        match self.links.entry(slug.clone()) {
            Entry::Occupied(_) => Err(ShortenerError::SlugAlreadyInUse),
            Entry::Vacant(entry) => {
                let short_link = ShortLink {
                    slug: slug.clone(),
                    url: url.clone(),
                };
                entry.insert((short_link.clone(), 0)); // Insert the link with initial redirect count of 0
                Ok(short_link)
            }
        }
    }

    fn handle_redirect(
        &mut self,
        slug: Slug,
    ) -> Result<ShortLink, ShortenerError> {
        // todo!("Implement the logic for redirection and incrementing the click counter")
        
        match self.links.get_mut(&slug) {
            Some((link, redirects)) => {
                *redirects += 1;
                Ok(link.clone())
            }
            None => Err(ShortenerError::SlugNotFound),
        }
    }

    fn handle_change_short_link(
        &mut self,
        slug: Slug,
        new_url: Url,
    ) -> Result<ShortLink, ShortenerError> {
        match self.links.get_mut(&slug) {
            Some((link, _)) => {
                link.url = new_url.clone();
                Ok(link.clone())
            }
            None => Err(ShortenerError::SlugNotFound),
        }
    }
}

impl queries::QueryHandler for UrlShortenerService {
    fn get_stats(&self, slug: Slug) -> Result<Stats, ShortenerError> {
        // todo!("Implement the logic for retrieving link statistics")

        match self.links.get(&slug) {
            Some((link, redirects)) => Ok(Stats {
                link: link.clone(),
                redirects: *redirects,
            }),
            None => Err(ShortenerError::SlugNotFound),
        }
    }
}




fn main() {
    let mut service = UrlShortenerService::new();

    // Create a short link with a random slug
    let url = Url("https://original-url.com".to_string());
    let short_link = service
        .handle_create_short_link(url.clone(), None)
        .expect("Failed to create short link");

    // Create a short link with a predefined slug
    let slug = Slug("custom.xyz".to_string());
    let custom_link = service
        .handle_create_short_link(url.clone(), Some(slug.clone()))
        .expect("Failed to create short link with custom slug");

    // Change the URL for an existing short link
    let new_url = Url("https://new-original-url.com".to_string());
    let updated_link = service
        .handle_change_short_link(slug.clone(), new_url.clone())
        .expect("Failed to change the URL for the short link");

    // Redirect using a slug
    let redirected_link = service
        .handle_redirect(slug.clone())
        .expect("Failed to redirect using the slug");

    // Get stats for the link
    let stats = service.get_stats(slug.clone()).expect("Failed to get stats");

    println!("Stats: {:?}", stats);
}
