pub enum ServerEvents {
    UpdateProjects,
    UpdateFreeCodeCampConfig,
    UpdateLocale,
    UpdateTests,
    UpdateTest,
    ResetTests,
    UpdateError,
    UpdateProject,
    UpdateDescription,
    UpdateConsole,
    UpdateHints,
    UpdateLoader,
    HandleProjectFinish,
    Response,
    Connect,
}

// kebab-case enum names
impl ToString for ServerEvents {
    fn to_string(&self) -> String {
        match self {
            ServerEvents::UpdateProjects => "update-projects".to_string(),
            ServerEvents::UpdateFreeCodeCampConfig => "update-free-code-camp-config".to_string(),
            ServerEvents::UpdateLocale => "update-locale".to_string(),
            ServerEvents::UpdateTests => "update-tests".to_string(),
            ServerEvents::UpdateTest => "update-test".to_string(),
            ServerEvents::ResetTests => "reset-tests".to_string(),
            ServerEvents::UpdateError => "update-error".to_string(),
            ServerEvents::UpdateProject => "update-project".to_string(),
            ServerEvents::UpdateDescription => "update-description".to_string(),
            ServerEvents::UpdateConsole => "update-console".to_string(),
            ServerEvents::UpdateHints => "update-hints".to_string(),
            ServerEvents::UpdateLoader => "update-loader".to_string(),
            ServerEvents::HandleProjectFinish => "handle-project-finish".to_string(),
            ServerEvents::Response => "response".to_string(),
            ServerEvents::Connect => "connect".to_string(),
        }
    }
}

pub enum ClientEvents {
    Connect,
    Disconnect,
    ToggleLoaderAnimation,
    ResetTests,
    RunTests,
    RequestData(String),
    GoToNextLesson,
    GoToPrevLesson,
    SelectProject(u16),
    CancelTests,
    ChangeLanguage(String),
}

// kebab-case enum names
impl ToString for ClientEvents {
    fn to_string(&self) -> String {
        match self {
            ClientEvents::Connect => "connect".to_string(),
            ClientEvents::Disconnect => "disconnect".to_string(),
            ClientEvents::ToggleLoaderAnimation => "toggle-loader-animation".to_string(),
            ClientEvents::ResetTests => "reset-tests".to_string(),
            ClientEvents::RunTests => "run-tests".to_string(),
            ClientEvents::RequestData(data) => "request-data".to_string(),
            ClientEvents::GoToNextLesson => "go-to-next-lesson".to_string(),
            ClientEvents::GoToPrevLesson => "go-to-prev-lesson".to_string(),
            ClientEvents::SelectProject(id) => "select-project".to_string(),
            ClientEvents::CancelTests => "cancel-tests".to_string(),
            ClientEvents::ChangeLanguage(lang) => "change-language".to_string(),
        }
    }
}
