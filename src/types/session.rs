use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OAuthSessionWrapper {
    pub client_id: matrix_sdk::authentication::oauth::ClientId,
    pub user_session: matrix_sdk::authentication::oauth::UserSession,
}

impl From<matrix_sdk::authentication::oauth::OAuthSession> for OAuthSessionWrapper {
    fn from(value: matrix_sdk::authentication::oauth::OAuthSession) -> Self {
        Self {
            client_id: value.client_id,
            user_session: value.user,
        }
    }
}

impl Into<matrix_sdk::authentication::oauth::OAuthSession> for OAuthSessionWrapper {
    fn into(self) -> matrix_sdk::authentication::oauth::OAuthSession {
        matrix_sdk::authentication::oauth::OAuthSession {
            client_id: self.client_id,
            user: self.user_session,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AuthSession {
    MatrixSession(matrix_sdk::authentication::matrix::MatrixSession),
    OAuthSession(OAuthSessionWrapper),
}

impl Into<matrix_sdk::AuthSession> for AuthSession {
    fn into(self) -> matrix_sdk::AuthSession {
        match self {
            Self::MatrixSession(v) => matrix_sdk::AuthSession::Matrix(v),
            Self::OAuthSession(v) => matrix_sdk::AuthSession::OAuth(Box::new(v.into())),
        }
    }
}

impl From<matrix_sdk::AuthSession> for AuthSession {
    fn from(value: matrix_sdk::AuthSession) -> Self {
        match value {
            matrix_sdk::AuthSession::Matrix(v) => AuthSession::MatrixSession(v),
            matrix_sdk::AuthSession::OAuth(v) => {
                AuthSession::OAuthSession(OAuthSessionWrapper::from(*v))
            }
            _ => {
                unimplemented!()
            }
        }
    }
}
