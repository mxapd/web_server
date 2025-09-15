pub fn handle_list_articles_request(
    request: HttpRequest,
    actor_directory: Arc<ActorDirectory>,
) -> Result<HttpResponse, Box<dyn Error>> {

    // create response channel for database communication
    // send query to database
    // wait for response recv()?
    // if Ok take rows and build html
    // return HttpResponse from Html
    // if Err build error html
}
