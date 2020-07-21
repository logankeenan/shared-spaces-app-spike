use crate::models::file::File;
use crate::repositories::file_repository::insert_file;
use crate::services::file_location_service::read_file_contents;
use crate::log;
use crate::models::file_part::FilePart;
use uuid::Uuid;
use crate::repositories::file_part_repository::insert_file_part;

pub async fn save_file(file: File) {
    let file_id = file.id;

    insert_file(file.clone()).await;

    let file_contents = read_file_contents(file).await;
    let total_length = file_contents.len();
    let offset = 31999;
    let mut order = 0;

    let mut current_position = 0;
    let mut content_left_to_read = true;
    while content_left_to_read {
        let mut file_part_content = "";

        if current_position + offset > total_length {
            file_part_content = &file_contents[current_position..];
        } else {
            file_part_content = &file_contents[current_position..current_position + offset];
        }

        let file_part_content_hash = md5::compute(file_part_content);

        let file_part = FilePart {
            id: Uuid::new_v4(),
            order,
            file_id,
            md5_hash: format!("{:x}", file_part_content_hash),
        };

        insert_file_part(file_part).await;

        order = order + 1;
        current_position = current_position + offset;

        if current_position > total_length {
            content_left_to_read = false;
        }
    }
}