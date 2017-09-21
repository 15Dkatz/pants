// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Status {
    // message fields
    pub code: i32,
    pub message: ::std::string::String,
    pub details: ::protobuf::RepeatedField<::protobuf::well_known_types::Any>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Status {}

impl Status {
    pub fn new() -> Status {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Status {
        static mut instance: ::protobuf::lazy::Lazy<Status> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Status,
        };
        unsafe {
            instance.get(Status::new)
        }
    }

    // int32 code = 1;

    pub fn clear_code(&mut self) {
        self.code = 0;
    }

    // Param is passed by value, moved
    pub fn set_code(&mut self, v: i32) {
        self.code = v;
    }

    pub fn get_code(&self) -> i32 {
        self.code
    }

    fn get_code_for_reflect(&self) -> &i32 {
        &self.code
    }

    fn mut_code_for_reflect(&mut self) -> &mut i32 {
        &mut self.code
    }

    // string message = 2;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.message, ::std::string::String::new())
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    fn get_message_for_reflect(&self) -> &::std::string::String {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }

    // repeated .google.protobuf.Any details = 3;

    pub fn clear_details(&mut self) {
        self.details.clear();
    }

    // Param is passed by value, moved
    pub fn set_details(&mut self, v: ::protobuf::RepeatedField<::protobuf::well_known_types::Any>) {
        self.details = v;
    }

    // Mutable pointer to the field.
    pub fn mut_details(&mut self) -> &mut ::protobuf::RepeatedField<::protobuf::well_known_types::Any> {
        &mut self.details
    }

    // Take field
    pub fn take_details(&mut self) -> ::protobuf::RepeatedField<::protobuf::well_known_types::Any> {
        ::std::mem::replace(&mut self.details, ::protobuf::RepeatedField::new())
    }

    pub fn get_details(&self) -> &[::protobuf::well_known_types::Any] {
        &self.details
    }

    fn get_details_for_reflect(&self) -> &::protobuf::RepeatedField<::protobuf::well_known_types::Any> {
        &self.details
    }

    fn mut_details_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::protobuf::well_known_types::Any> {
        &mut self.details
    }
}

impl ::protobuf::Message for Status {
    fn is_initialized(&self) -> bool {
        for v in &self.details {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.code = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.message)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.details)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.code != 0 {
            my_size += ::protobuf::rt::value_size(1, self.code, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.message.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.message);
        }
        for value in &self.details {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.code != 0 {
            os.write_int32(1, self.code)?;
        }
        if !self.message.is_empty() {
            os.write_string(2, &self.message)?;
        }
        for v in &self.details {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Status {
    fn new() -> Status {
        Status::new()
    }

    fn descriptor_static(_: ::std::option::Option<Status>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "code",
                    Status::get_code_for_reflect,
                    Status::mut_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    Status::get_message_for_reflect,
                    Status::mut_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Any>>(
                    "details",
                    Status::get_details_for_reflect,
                    Status::mut_details_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Status>(
                    "Status",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Status {
    fn clear(&mut self) {
        self.clear_code();
        self.clear_message();
        self.clear_details();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Status {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Status {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17google/rpc/status.proto\x12\ngoogle.rpc\x1a\x19google/protobuf/any\
    .proto\"f\n\x06Status\x12\x12\n\x04code\x18\x01\x20\x01(\x05R\x04code\
    \x12\x18\n\x07message\x18\x02\x20\x01(\tR\x07message\x12.\n\x07details\
    \x18\x03\x20\x03(\x0b2\x14.google.protobuf.AnyR\x07detailsB^\n\x0ecom.go\
    ogle.rpcB\x0bStatusProtoP\x01Z7google.golang.org/genproto/googleapis/rpc\
    /status;status\xa2\x02\x03RPCJ\xcc\x20\n\x06\x12\x04\x0e\0[\x01\n\xbd\
    \x04\n\x01\x0c\x12\x03\x0e\0\x122\xb2\x04\x20Copyright\x202017\x20Google\
    \x20Inc.\n\n\x20Licensed\x20under\x20the\x20Apache\x20License,\x20Versio\
    n\x202.0\x20(the\x20\"License\");\n\x20you\x20may\x20not\x20use\x20this\
    \x20file\x20except\x20in\x20compliance\x20with\x20the\x20License.\n\x20Y\
    ou\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20License\x20at\n\n\x20\
    \x20\x20\x20\x20http://www.apache.org/licenses/LICENSE-2.0\n\n\x20Unless\
    \x20required\x20by\x20applicable\x20law\x20or\x20agreed\x20to\x20in\x20w\
    riting,\x20software\n\x20distributed\x20under\x20the\x20License\x20is\
    \x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\x20WITHOUT\x20WA\
    RRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20either\x20express\
    \x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20the\x20specific\
    \x20language\x20governing\x20permissions\x20and\n\x20limitations\x20unde\
    r\x20the\x20License.\n\n\x08\n\x01\x02\x12\x03\x10\x08\x12\n\t\n\x02\x03\
    \0\x12\x03\x12\x07\"\n\x08\n\x01\x08\x12\x03\x14\0N\n\x0b\n\x04\x08\xe7\
    \x07\0\x12\x03\x14\0N\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x14\x07\x11\
    \n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x14\x07\x11\n\x0e\n\x07\x08\xe7\
    \x07\0\x02\0\x01\x12\x03\x14\x07\x11\n\x0c\n\x05\x08\xe7\x07\0\x07\x12\
    \x03\x14\x14M\n\x08\n\x01\x08\x12\x03\x15\0\"\n\x0b\n\x04\x08\xe7\x07\
    \x01\x12\x03\x15\0\"\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\x15\x07\x1a\
    \n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x15\x07\x1a\n\x0e\n\x07\x08\xe7\
    \x07\x01\x02\0\x01\x12\x03\x15\x07\x1a\n\x0c\n\x05\x08\xe7\x07\x01\x03\
    \x12\x03\x15\x1d!\n\x08\n\x01\x08\x12\x03\x16\0,\n\x0b\n\x04\x08\xe7\x07\
    \x02\x12\x03\x16\0,\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\x16\x07\x1b\
    \n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\x16\x07\x1b\n\x0e\n\x07\x08\xe7\
    \x07\x02\x02\0\x01\x12\x03\x16\x07\x1b\n\x0c\n\x05\x08\xe7\x07\x02\x07\
    \x12\x03\x16\x1e+\n\x08\n\x01\x08\x12\x03\x17\0'\n\x0b\n\x04\x08\xe7\x07\
    \x03\x12\x03\x17\0'\n\x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\x17\x07\x13\
    \n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\x03\x17\x07\x13\n\x0e\n\x07\x08\xe7\
    \x07\x03\x02\0\x01\x12\x03\x17\x07\x13\n\x0c\n\x05\x08\xe7\x07\x03\x07\
    \x12\x03\x17\x16&\n\x08\n\x01\x08\x12\x03\x18\0!\n\x0b\n\x04\x08\xe7\x07\
    \x04\x12\x03\x18\0!\n\x0c\n\x05\x08\xe7\x07\x04\x02\x12\x03\x18\x07\x18\
    \n\r\n\x06\x08\xe7\x07\x04\x02\0\x12\x03\x18\x07\x18\n\x0e\n\x07\x08\xe7\
    \x07\x04\x02\0\x01\x12\x03\x18\x07\x18\n\x0c\n\x05\x08\xe7\x07\x04\x07\
    \x12\x03\x18\x1b\x20\n\xcd\x13\n\x02\x04\0\x12\x04O\0[\x01\x1a\xc0\x13\
    \x20The\x20`Status`\x20type\x20defines\x20a\x20logical\x20error\x20model\
    \x20that\x20is\x20suitable\x20for\x20different\n\x20programming\x20envir\
    onments,\x20including\x20REST\x20APIs\x20and\x20RPC\x20APIs.\x20It\x20is\
    \x20used\x20by\n\x20[gRPC](https://github.com/grpc).\x20The\x20error\x20\
    model\x20is\x20designed\x20to\x20be:\n\n\x20-\x20Simple\x20to\x20use\x20\
    and\x20understand\x20for\x20most\x20users\n\x20-\x20Flexible\x20enough\
    \x20to\x20meet\x20unexpected\x20needs\n\n\x20#\x20Overview\n\n\x20The\
    \x20`Status`\x20message\x20contains\x20three\x20pieces\x20of\x20data:\
    \x20error\x20code,\x20error\x20message,\n\x20and\x20error\x20details.\
    \x20The\x20error\x20code\x20should\x20be\x20an\x20enum\x20value\x20of\n\
    \x20[google.rpc.Code][google.rpc.Code],\x20but\x20it\x20may\x20accept\
    \x20additional\x20error\x20codes\x20if\x20needed.\x20\x20The\n\x20error\
    \x20message\x20should\x20be\x20a\x20developer-facing\x20English\x20messa\
    ge\x20that\x20helps\n\x20developers\x20*understand*\x20and\x20*resolve*\
    \x20the\x20error.\x20If\x20a\x20localized\x20user-facing\n\x20error\x20m\
    essage\x20is\x20needed,\x20put\x20the\x20localized\x20message\x20in\x20t\
    he\x20error\x20details\x20or\n\x20localize\x20it\x20in\x20the\x20client.\
    \x20The\x20optional\x20error\x20details\x20may\x20contain\x20arbitrary\n\
    \x20information\x20about\x20the\x20error.\x20There\x20is\x20a\x20predefi\
    ned\x20set\x20of\x20error\x20detail\x20types\n\x20in\x20the\x20package\
    \x20`google.rpc`\x20that\x20can\x20be\x20used\x20for\x20common\x20error\
    \x20conditions.\n\n\x20#\x20Language\x20mapping\n\n\x20The\x20`Status`\
    \x20message\x20is\x20the\x20logical\x20representation\x20of\x20the\x20er\
    ror\x20model,\x20but\x20it\n\x20is\x20not\x20necessarily\x20the\x20actua\
    l\x20wire\x20format.\x20When\x20the\x20`Status`\x20message\x20is\n\x20ex\
    posed\x20in\x20different\x20client\x20libraries\x20and\x20different\x20w\
    ire\x20protocols,\x20it\x20can\x20be\n\x20mapped\x20differently.\x20For\
    \x20example,\x20it\x20will\x20likely\x20be\x20mapped\x20to\x20some\x20ex\
    ceptions\n\x20in\x20Java,\x20but\x20more\x20likely\x20mapped\x20to\x20so\
    me\x20error\x20codes\x20in\x20C.\n\n\x20#\x20Other\x20uses\n\n\x20The\
    \x20error\x20model\x20and\x20the\x20`Status`\x20message\x20can\x20be\x20\
    used\x20in\x20a\x20variety\x20of\n\x20environments,\x20either\x20with\
    \x20or\x20without\x20APIs,\x20to\x20provide\x20a\n\x20consistent\x20deve\
    loper\x20experience\x20across\x20different\x20environments.\n\n\x20Examp\
    le\x20uses\x20of\x20this\x20error\x20model\x20include:\n\n\x20-\x20Parti\
    al\x20errors.\x20If\x20a\x20service\x20needs\x20to\x20return\x20partial\
    \x20errors\x20to\x20the\x20client,\n\x20\x20\x20\x20\x20it\x20may\x20emb\
    ed\x20the\x20`Status`\x20in\x20the\x20normal\x20response\x20to\x20indica\
    te\x20the\x20partial\n\x20\x20\x20\x20\x20errors.\n\n\x20-\x20Workflow\
    \x20errors.\x20A\x20typical\x20workflow\x20has\x20multiple\x20steps.\x20\
    Each\x20step\x20may\n\x20\x20\x20\x20\x20have\x20a\x20`Status`\x20messag\
    e\x20for\x20error\x20reporting.\n\n\x20-\x20Batch\x20operations.\x20If\
    \x20a\x20client\x20uses\x20batch\x20request\x20and\x20batch\x20response,\
    \x20the\n\x20\x20\x20\x20\x20`Status`\x20message\x20should\x20be\x20used\
    \x20directly\x20inside\x20batch\x20response,\x20one\x20for\n\x20\x20\x20\
    \x20\x20each\x20error\x20sub-response.\n\n\x20-\x20Asynchronous\x20opera\
    tions.\x20If\x20an\x20API\x20call\x20embeds\x20asynchronous\x20operation\
    \n\x20\x20\x20\x20\x20results\x20in\x20its\x20response,\x20the\x20status\
    \x20of\x20those\x20operations\x20should\x20be\n\x20\x20\x20\x20\x20repre\
    sented\x20directly\x20using\x20the\x20`Status`\x20message.\n\n\x20-\x20L\
    ogging.\x20If\x20some\x20API\x20errors\x20are\x20stored\x20in\x20logs,\
    \x20the\x20message\x20`Status`\x20could\n\x20\x20\x20\x20\x20be\x20used\
    \x20directly\x20after\x20any\x20stripping\x20needed\x20for\x20security/p\
    rivacy\x20reasons.\n\n\n\n\x03\x04\0\x01\x12\x03O\x08\x0e\nd\n\x04\x04\0\
    \x02\0\x12\x03Q\x02\x11\x1aW\x20The\x20status\x20code,\x20which\x20shoul\
    d\x20be\x20an\x20enum\x20value\x20of\x20[google.rpc.Code][google.rpc.Cod\
    e].\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04Q\x02O\x10\n\x0c\n\x05\x04\0\x02\
    \0\x05\x12\x03Q\x02\x07\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03Q\x08\x0c\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03Q\x0f\x10\n\xeb\x01\n\x04\x04\0\x02\
    \x01\x12\x03V\x02\x15\x1a\xdd\x01\x20A\x20developer-facing\x20error\x20m\
    essage,\x20which\x20should\x20be\x20in\x20English.\x20Any\n\x20user-faci\
    ng\x20error\x20message\x20should\x20be\x20localized\x20and\x20sent\x20in\
    \x20the\n\x20[google.rpc.Status.details][google.rpc.Status.details]\x20f\
    ield,\x20or\x20localized\x20by\x20the\x20client.\n\n\r\n\x05\x04\0\x02\
    \x01\x04\x12\x04V\x02Q\x11\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03V\x02\
    \x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03V\t\x10\n\x0c\n\x05\x04\0\x02\
    \x01\x03\x12\x03V\x13\x14\ny\n\x04\x04\0\x02\x02\x12\x03Z\x02+\x1al\x20A\
    \x20list\x20of\x20messages\x20that\x20carry\x20the\x20error\x20details.\
    \x20\x20There\x20is\x20a\x20common\x20set\x20of\n\x20message\x20types\
    \x20for\x20APIs\x20to\x20use.\n\n\x0c\n\x05\x04\0\x02\x02\x04\x12\x03Z\
    \x02\n\n\x0c\n\x05\x04\0\x02\x02\x06\x12\x03Z\x0b\x1e\n\x0c\n\x05\x04\0\
    \x02\x02\x01\x12\x03Z\x1f&\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03Z)*b\x06\
    proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
