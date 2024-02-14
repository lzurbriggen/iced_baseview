/// An error that occurred while running an application.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The futures executor could not be created.
    #[error("the futures executor could not be created")]
    ExecutorCreationFailed(iced::futures::io::Error),

    /// The application window could not be created.
    #[error("the application window could not be created")]
    WindowCreationFailed,

    /// The application graphics context could not be created.
    #[error("the application graphics context could not be created")]
    GraphicsCreationFailed(iced_graphics::Error),
}

impl From<iced_graphics::Error> for Error {
    fn from(error: iced_graphics::Error) -> Error {
        Error::GraphicsCreationFailed(error)
    }
}
