use crate::models::file::File;
use crate::adapters::localforage_adapter::get_by_id;
use crate::log;

pub async fn read_file_contents(file: File) -> String {
    let file_contents_as_string = get_by_id(file.location).await;

    file_contents_as_string
}