use actix::Message;

#[derive(Message)]
#[rtype(result = "String")]
pub(crate) struct HtmlAsString;
