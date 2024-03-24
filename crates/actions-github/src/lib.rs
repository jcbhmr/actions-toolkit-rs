//!
//! 
//! 

pub struct Context {
    pub payload: (),
    pub event_name: String,
    pub sha: String,
    pub ref_: String,
    pub workflow: String,
    pub action: String,
    pub actor: String,
    pub job: String,
    pub run_attempt: u32,
    pub run_number: u32,
    pub run_id: u32,
    pub api_url: String,
    pub server_url: String,
    pub graphql_url: String,
}

pub struct ContextIssue {
    pub owner: String,
    pub repo: String,
    pub number: u32,
}

pub struct ContextRepo {
    pub owner: String,
    pub repo: String,
}

impl Context {
    pub fn new() -> Self {
        Context {
            payload: (),
            event_name: std::env::var("GITHUB_EVENT_NAME").unwrap(),
            sha: std::env::var("GITHUB_SHA").unwrap(),
            ref_: std::env::var("GITHUB_REF").unwrap(),
            workflow: std::env::var("GITHUB_WORKFLOW").unwrap(),
            action: std::env::var("GITHUB_ACTION").unwrap(),
            actor: std::env::var("GITHUB_ACTOR").unwrap(),
            job: std::env::var("GITHUB_JOB").unwrap(),
            run_attempt: std::env::var("GITHUB_RUN_ATTEMPT").unwrap().parse().unwrap(),
            run_number: std::env::var("GITHUB_RUN_NUMBER").unwrap().parse().unwrap(),
            run_id: std::env::var("GITHUB_RUN_ID").unwrap().parse().unwrap(),
            api_url: std::env::var("GITHUB_API_URL").unwrap_or_else(|_| "https://api.github.com".to_string()),
            server_url: std::env::var("GITHUB_SERVER_URL").unwrap_or_else(|_| "https://github.com".to_string()),
            graphql_url: std::env::var("GITHUB_GRAPHQL_URL").unwrap_or_else(|_| "https://api.github.com/graphql".to_string()),
        }
    }

    pub fn issue(&self) -> ContextIssue {
        let ContextRepo { owner, repo } = self.repo();
        let number = match 1 {
            1 => 1,
            _ => panic!(),
        };
        ContextIssue { owner, repo, number }
    }

    pub fn repo(&self) -> ContextRepo {
        if let Ok(github_repository) = std::env::var("GITHUB_REPOSITORY") {
            let mut parts = github_repository.split('/');
            let owner = parts.next().unwrap().to_string();
            let repo = parts.next().unwrap().to_string();
            ContextRepo { owner, repo }
        } else if (false) {
            panic!();
        } else {
            panic!();
        }
    }
}

pub fn context() -> Context {
    Context::new()
}

pub fn get_octokit(token: String) -> () {
    ()
}