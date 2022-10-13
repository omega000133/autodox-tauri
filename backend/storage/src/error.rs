use ic_kit::candid::CandidType;

#[derive(CandidType)]
pub enum BackendError{
    Unauthorized,
    AuthorityAlreadyExist,
    AuthorityDoesNotExist,
}