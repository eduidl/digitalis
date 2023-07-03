// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc 3.12.4
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `foxglove/PointsAnnotation.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:foxglove.PointsAnnotation)
pub struct PointsAnnotation {
    // message fields
    // @@protoc_insertion_point(field:foxglove.PointsAnnotation.timestamp)
    pub timestamp: ::protobuf::MessageField<::protobuf::well_known_types::timestamp::Timestamp>,
    // @@protoc_insertion_point(field:foxglove.PointsAnnotation.type)
    pub type_: ::protobuf::EnumOrUnknown<points_annotation::Type>,
    // @@protoc_insertion_point(field:foxglove.PointsAnnotation.points)
    pub points: ::std::vec::Vec<super::Point2::Point2>,
    // @@protoc_insertion_point(field:foxglove.PointsAnnotation.outline_color)
    pub outline_color: ::protobuf::MessageField<super::Color::Color>,
    // @@protoc_insertion_point(field:foxglove.PointsAnnotation.outline_colors)
    pub outline_colors: ::std::vec::Vec<super::Color::Color>,
    // @@protoc_insertion_point(field:foxglove.PointsAnnotation.fill_color)
    pub fill_color: ::protobuf::MessageField<super::Color::Color>,
    // @@protoc_insertion_point(field:foxglove.PointsAnnotation.thickness)
    pub thickness: f64,
    // special fields
    // @@protoc_insertion_point(special_field:foxglove.PointsAnnotation.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PointsAnnotation {
    fn default() -> &'a PointsAnnotation {
        <PointsAnnotation as ::protobuf::Message>::default_instance()
    }
}

impl PointsAnnotation {
    pub fn new() -> PointsAnnotation {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, ::protobuf::well_known_types::timestamp::Timestamp>(
            "timestamp",
            |m: &PointsAnnotation| { &m.timestamp },
            |m: &mut PointsAnnotation| { &mut m.timestamp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "type",
            |m: &PointsAnnotation| { &m.type_ },
            |m: &mut PointsAnnotation| { &mut m.type_ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "points",
            |m: &PointsAnnotation| { &m.points },
            |m: &mut PointsAnnotation| { &mut m.points },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Color::Color>(
            "outline_color",
            |m: &PointsAnnotation| { &m.outline_color },
            |m: &mut PointsAnnotation| { &mut m.outline_color },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "outline_colors",
            |m: &PointsAnnotation| { &m.outline_colors },
            |m: &mut PointsAnnotation| { &mut m.outline_colors },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Color::Color>(
            "fill_color",
            |m: &PointsAnnotation| { &m.fill_color },
            |m: &mut PointsAnnotation| { &mut m.fill_color },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "thickness",
            |m: &PointsAnnotation| { &m.thickness },
            |m: &mut PointsAnnotation| { &mut m.thickness },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PointsAnnotation>(
            "PointsAnnotation",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PointsAnnotation {
    const NAME: &'static str = "PointsAnnotation";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.timestamp)?;
                },
                16 => {
                    self.type_ = is.read_enum_or_unknown()?;
                },
                26 => {
                    self.points.push(is.read_message()?);
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.outline_color)?;
                },
                42 => {
                    self.outline_colors.push(is.read_message()?);
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.fill_color)?;
                },
                57 => {
                    self.thickness = is.read_double()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.timestamp.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.type_ != ::protobuf::EnumOrUnknown::new(points_annotation::Type::UNKNOWN) {
            my_size += ::protobuf::rt::int32_size(2, self.type_.value());
        }
        for value in &self.points {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.outline_color.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.outline_colors {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.fill_color.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.thickness != 0. {
            my_size += 1 + 8;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.timestamp.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if self.type_ != ::protobuf::EnumOrUnknown::new(points_annotation::Type::UNKNOWN) {
            os.write_enum(2, ::protobuf::EnumOrUnknown::value(&self.type_))?;
        }
        for v in &self.points {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        if let Some(v) = self.outline_color.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        for v in &self.outline_colors {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        if let Some(v) = self.fill_color.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        if self.thickness != 0. {
            os.write_double(7, self.thickness)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> PointsAnnotation {
        PointsAnnotation::new()
    }

    fn clear(&mut self) {
        self.timestamp.clear();
        self.type_ = ::protobuf::EnumOrUnknown::new(points_annotation::Type::UNKNOWN);
        self.points.clear();
        self.outline_color.clear();
        self.outline_colors.clear();
        self.fill_color.clear();
        self.thickness = 0.;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PointsAnnotation {
        static instance: PointsAnnotation = PointsAnnotation {
            timestamp: ::protobuf::MessageField::none(),
            type_: ::protobuf::EnumOrUnknown::from_i32(0),
            points: ::std::vec::Vec::new(),
            outline_color: ::protobuf::MessageField::none(),
            outline_colors: ::std::vec::Vec::new(),
            fill_color: ::protobuf::MessageField::none(),
            thickness: 0.,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PointsAnnotation {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PointsAnnotation").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PointsAnnotation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PointsAnnotation {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `PointsAnnotation`
pub mod points_annotation {
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:foxglove.PointsAnnotation.Type)
    pub enum Type {
        // @@protoc_insertion_point(enum_value:foxglove.PointsAnnotation.Type.UNKNOWN)
        UNKNOWN = 0,
        // @@protoc_insertion_point(enum_value:foxglove.PointsAnnotation.Type.POINTS)
        POINTS = 1,
        // @@protoc_insertion_point(enum_value:foxglove.PointsAnnotation.Type.LINE_LOOP)
        LINE_LOOP = 2,
        // @@protoc_insertion_point(enum_value:foxglove.PointsAnnotation.Type.LINE_STRIP)
        LINE_STRIP = 3,
        // @@protoc_insertion_point(enum_value:foxglove.PointsAnnotation.Type.LINE_LIST)
        LINE_LIST = 4,
    }

    impl ::protobuf::Enum for Type {
        const NAME: &'static str = "Type";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<Type> {
            match value {
                0 => ::std::option::Option::Some(Type::UNKNOWN),
                1 => ::std::option::Option::Some(Type::POINTS),
                2 => ::std::option::Option::Some(Type::LINE_LOOP),
                3 => ::std::option::Option::Some(Type::LINE_STRIP),
                4 => ::std::option::Option::Some(Type::LINE_LIST),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [Type] = &[
            Type::UNKNOWN,
            Type::POINTS,
            Type::LINE_LOOP,
            Type::LINE_STRIP,
            Type::LINE_LIST,
        ];
    }

    impl ::protobuf::EnumFull for Type {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("PointsAnnotation.Type").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for Type {
        fn default() -> Self {
            Type::UNKNOWN
        }
    }

    impl Type {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<Type>("PointsAnnotation.Type")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1ffoxglove/PointsAnnotation.proto\x12\x08foxglove\x1a\x14foxglove/Co\
    lor.proto\x1a\x15foxglove/Point2.proto\x1a\x1fgoogle/protobuf/timestamp.\
    proto\"\xb6\x03\n\x10PointsAnnotation\x128\n\ttimestamp\x18\x01\x20\x01(\
    \x0b2\x1a.google.protobuf.TimestampR\ttimestamp\x123\n\x04type\x18\x02\
    \x20\x01(\x0e2\x1f.foxglove.PointsAnnotation.TypeR\x04type\x12(\n\x06poi\
    nts\x18\x03\x20\x03(\x0b2\x10.foxglove.Point2R\x06points\x124\n\routline\
    _color\x18\x04\x20\x01(\x0b2\x0f.foxglove.ColorR\x0coutlineColor\x126\n\
    \x0eoutline_colors\x18\x05\x20\x03(\x0b2\x0f.foxglove.ColorR\routlineCol\
    ors\x12.\n\nfill_color\x18\x06\x20\x01(\x0b2\x0f.foxglove.ColorR\tfillCo\
    lor\x12\x1c\n\tthickness\x18\x07\x20\x01(\x01R\tthickness\"M\n\x04Type\
    \x12\x0b\n\x07UNKNOWN\x10\0\x12\n\n\x06POINTS\x10\x01\x12\r\n\tLINE_LOOP\
    \x10\x02\x12\x0e\n\nLINE_STRIP\x10\x03\x12\r\n\tLINE_LIST\x10\x04b\x06pr\
    oto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::Color::file_descriptor().clone());
            deps.push(super::Point2::file_descriptor().clone());
            deps.push(::protobuf::well_known_types::timestamp::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PointsAnnotation::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(points_annotation::Type::generated_enum_descriptor_data());
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}