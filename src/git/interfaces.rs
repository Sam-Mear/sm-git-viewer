pub trait DataProvider {
    fn get_repository(&self) -> Box<dyn Repository>;
    fn get_workflow(&self) -> Box<dyn Workflow>;
}

pub trait Repository {
    fn get_name(&self) -> &str;
    fn get_current_branch(&self) -> &str;
    fn get_branches(&self) -> Vec<String>;
    fn get_remotes(&self) -> Vec<String>;
    fn get_path(&self) -> &str;
    fn get_remote_urls(&self) -> Vec<String>;
}

pub trait Workflow {
    fn get_issues(&self) -> Vec<String>;
}

pub trait Issue {
    fn get_id(&self) -> &str;
    fn get_title(&self) -> &str;
    fn get_description(&self) -> &str;
    fn created_by(&self) -> &str;
    fn get_assignees(&self) -> Vec<String>;
    fn get_reporter(&self) -> &str;
    fn get_kind(&self) -> IssueKind;
    fn get_watchers(&self) -> Vec<String>;
    fn get_status(&self) -> IssueStatus;    
    fn get_comments(&self) -> Vec<Box<dyn Comment>>;
}

pub trait Comment {
    fn get_id(&self) -> &str;
    fn get_author(&self) -> &str;
    fn get_content(&self) -> &str;
    fn get_timestamp(&self) -> &str;
}

pub enum IssueStatus {
    Open,
    InProgress,
    Done,
    WontFix,
    Duplicate,
    Other(String),
}

pub enum IssueKind {
    Bug,
    Feature,
    Task,
    Documentation,
    Other(String),
}