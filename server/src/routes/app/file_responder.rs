use {
    rocket::{
        http::ContentType,
        request::Request,
        response::{self, Responder, Response},
    },
    std::io::Cursor,
};

pub struct FileResponder {
    file: &'static [u8],
    content_type: ContentType,
}

impl FileResponder {
    pub fn new(file: &'static [u8], content_type: ContentType) -> Self {
        Self { file, content_type }
    }
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for FileResponder {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .header(self.content_type)
            .sized_body(self.file.len(), Cursor::new(self.file))
            .ok()
    }
}
