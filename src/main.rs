extern crate clap;
extern crate reqwest;

mod github_api;
mod http;
mod json;
use clap::{ App, Arg };
use github_api::GithubAPI;
use http::HttpRequest;
use json::JSON;


pub struct Checkhub{}

impl Checkhub{
    pub fn new()->Checkhub{
        Checkhub{}
    }
    pub fn run(&self){
        let tool = App::new("checkhub")
        .version("0.0.1")
        .author("Itinose <dhelitus@gmail.com>")
        .about("CLI tool which can check github user informations")
        .usage("./check-hub [INFO NAME]")
        .arg(Arg::with_name("INFO NAME")
        .help("GitHub user informations(current version supporting)
        - name 
        - login
        - bio 
        - gist-count 
        - follow-count
        - follower-count
        - location") 
       .required(true)          
    );
    self.parse_command(tool);      
    }                                                    
    fn parse_command(&self,tool: App){
        let github = GithubAPI::new();
        let client = HttpRequest::new();
        let url = github.profile();
        let json = client.get_request_json(url);
        let json_decoder = JSON::new();
        let maches = tool.get_matches();
      
        if let Some(arg) = maches.value_of("INFO NAME"){
            let name :&str = "name";
            let login :&str = "login";
            let bio :&str = "bio";
            let gist_count :&str = "gist-count";
            let follow_count :&str = "follow-count";
            let follower_count :&str = "follower-count";
            let repository_count :&str = "repository-count";
            let location :&str = "location";

            match &arg {
                name => json_decoder.name(json),
                login => json_decoder.login(json),
                bio => json_decoder.bio(json),
                gist_count => json_decoder.gist(json),
                follow_count => json_decoder.follow_count(json),
                follower_count => json_decoder.follower_count(json),
                repository_count => json_decoder.repository_count(json),
                location => json_decoder.location(json),
            };
        }
    }
}
fn main(){
    let checkhub = Checkhub::new();
    checkhub.run();
}
