use tokio_postgres::Client;

use crate::{
    form,
    model::{Url, UrlID, UrlTarget},
    Result,
};

pub async fn create(client: &Client, cu: form::CreateUrl) -> Result<UrlID> {
    unimplemented!()
}

pub async fn goto_url(client: &mut Client, id: String) -> Result<UrlTarget> {
    unimplemented!()
}

pub async fn rank(client: &Client) -> Result<Vec<Url>> {
    unreachable!()
}
