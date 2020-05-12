use uuid::Uuid;

use std::collections::HashMap;

/// IdentityProvider represents any arbitrary provider of an authorization or
/// authentication service (i.e., a provider of an OpenID Connection-capable
/// identity API).
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum IdentityProvider {
    /// Google provides an OpenID connect OAuth 2.0 API: https://developers.google.com/identity/protocols/oauth2/openid-connect.
    /// As does twitch, Google returns IDs as "sub" claims--strings.
    Google,

    /// GitHub also provides an OAuth 2.0 API, but uses non-standard endpoints: https://fusionauth.io/docs/v1/tech/identity-providers/openid-connect/github
    /// Also, their docs are pretty unclear, which doesn't help. IDs are
    /// returned as integers in the GitHub oauth API.
    GitHub,

    /// Twitch has excellent OpenID connect integration: https://dev.twitch.tv/docs/authentication/getting-tokens-oidc.
    /// User IDs are returned in the "sub" OpenID connect claim, and are
    /// returned as strings.
    Twitch,

    /// Reddit doesn't have support for OpenID connect, but does have a
    /// /api/v1/me route that we can use to get the ID of a user. In the
    /// Reddit user API, IDs are stored as SERIAL strings.
    Reddit,

    /// Twitter's docs are pretty god-awful. Here's a route we can use to get
    /// a user ID from an access token: https://developer.twitter.com/en/docs/accounts-and-users/manage-account-settings/api-reference/get-account-verify_credentials.
    /// In the Twitter user API, IDs are stored as large unsigned integers.
    Twitter,

    /// In contrast to Twitter, Discord's docs are pretty top-tier. Here's how
    /// we can identify a user:
    /// https://discord.com/developers/docs/resources/user#get-current-user.
    /// For the discord identity provider, we'll want to use a string to store
    /// IDs.
    Discord,

    /// This register is identified by a Swaply username/id and password, not a
    /// third-party oauth integration.
    Swaply,
}

/// User represents a user of any one of the swaply products. A user may be
/// authenticated with swaply itself, or with one of the supported
/// authentication providers.
#[derive(Debug)]
pub struct User<'a> {
    /// The ID of the user - if a user is being inserted into the database,
    /// this field may be omitted, as an ID is assigned by the database to the
    /// user.
    id: Option<Uuid>,

    /// The username associated with this user - this field may not be omitted
    /// safely.
    username: &'a str,

    /// Each of the third party identity providers that the user chosen to
    /// connect to their account.
    identities: HashMap<IdentityProvider, &'a str>,
}

impl<'a> User<'a> {
    /// Creates a new instance of the user details struct.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the user
    /// * `username` - The username associated with the user
    ///
    /// # Examples
    ///
    /// ```
    /// use swaply_identity::schema::user::User;
    /// use std::collections::HashMap;
    ///
    /// let u = User::new(None, "test", HashMap::new());
    /// ```
    pub fn new(id: Option<Uuid>, username: &'a str, identities: HashMap<IdentityProvider, &'a str>) -> Self {
        Self { id, username, identities }
    }

    /// Gets the ID of the Swaply user.
    ///
    /// # Examples
    ///
    /// ```
    /// use swaply_identity::schema::user::User;
    /// use std::collections::HashMap;
    ///
    /// let u = User::new(None, "test", HashMap::new());
    /// assert_eq!(u.id(), None);
    /// ```
    pub fn id(&self) -> Option<&Uuid> {
        self.id.as_ref()
    }

    /// Gets the username of the Swaply user.
    ///
    /// # Examples
    ///
    /// ```
    /// use swaply_identity::schema::user::User;
    /// use std::collections::HashMap;
    ///
    /// let u = User::new(None, "test", HashMap::new());
    /// assert_eq!(u.username(), "test");
    /// ```
    pub fn username(&self) -> &str {
        self.username
    }

    /// Obtains a reference to the set of third party identity integrations associated with this
    /// user.
    ///
    /// # Examples
    ///
    /// ```
    /// use swaply_identity::schema::user::User;
    /// use std::collections::HashMap;
    ///
    /// let u = User::new(None, "test", HashMap::new());
    /// assert_eq!(*u.identities(), HashMap::new());
    /// ```
    pub fn identities(&self) -> &HashMap<IdentityProvider, &str> {
        &self.identities
    }
}