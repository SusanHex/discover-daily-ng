struct SpotifyApplication {
    client_id: String,
    response_type: String,
    redirect_uri: String,
    scope: Option<String>
}

impl SpotifyApplication {

    pub fn new(client_id: String, redirect_uri: String, scope: Option<String>) -> SpotifyApplication {
        SpotifyApplication { client_id, response_type: "code".to_owned(), redirect_uri, scope }
    }
    
    
}