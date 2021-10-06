pub mod chess_gui_error {
    use trait_enum::*;
    use ggez::GameError;
    #[allow(dead_code)]
    pub type ChessGuiResult<T> = std::result::Result<T, ChessGuiError>;

    pub trait ChessGuiErrorCommonTrait {
        fn chess_gui_error_to_string(&self) -> String;
    }

    // Make it easy to convert strings to an error message
    pub trait ToChessGuiError {
        fn to_chess_gui_error(&self) -> ChessGuiError;
    }
    impl ToChessGuiError for str {
        fn to_chess_gui_error(&self) -> ChessGuiError {
            return ChessGuiError::String(self.to_string());
        }
    }
    impl ToChessGuiError for String {
        fn to_chess_gui_error(&self) -> ChessGuiError {
            return ChessGuiError::String(self.clone());
        }
    }

    trait_enum!{
        #[derive(Debug)]
        pub enum ChessGuiError: ChessGuiErrorCommonTrait {
            CouldNotLoadImageError,
            String,
            GameError
        }
    }

    impl std::fmt::Display for ChessGuiError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({})", self.chess_gui_error_to_string())
        }
    }

    impl ChessGuiErrorCommonTrait for ggez::GameError {
        fn chess_gui_error_to_string(&self) -> String {
            return self.to_string();
        }
    }
    impl std::convert::From<ggez::GameError> for ChessGuiError {
        fn from(err: ggez::GameError) -> Self {
            return ChessGuiError::GameError(err);
        }
    }
    impl std::convert::From<ChessGuiError> for ggez::GameError {
        fn from(err: ChessGuiError) -> Self {
            match err {
                ChessGuiError::GameError(game_error) => {
                    return game_error;
                }
                _ => {
                    return ggez::GameError::CustomError(err.chess_gui_error_to_string());
                }
            }
        }
    }

    impl ChessGuiErrorCommonTrait for String {
        fn chess_gui_error_to_string(&self) -> String {
            return self.clone();
        }
    }
    impl std::convert::From<String> for ChessGuiError {
        fn from(err: String) -> Self {
            return ChessGuiError::String(err);
        }
    }
    impl std::convert::From<&str> for ChessGuiError {
        fn from(err: &str) -> Self {
            return ChessGuiError::String(err.to_string());
        }
    }


    #[derive(Debug)]
    pub struct CouldNotLoadImageError {

    }
    impl ChessGuiErrorCommonTrait for CouldNotLoadImageError {
        fn chess_gui_error_to_string(&self) -> String {
            return "Could not load image".to_string();
        }
    }
}
