use std::num::FromPrimitive;

use ffi;

pub type FtResult<T> = Result<T, Error>;

#[derive(Copy, Clone, PartialEq, Eq, Show)]
#[repr(i32)]
pub enum Error {
    Ok                          = ffi::FT_Err_Ok,
    CannotOpenResource          = ffi::FT_Err_Cannot_Open_Resource,
    UnknownFileFormat           = ffi::FT_Err_Unknown_File_Format,
    InvalidFileFormat           = ffi::FT_Err_Invalid_File_Format,
    InvalidVersion              = ffi::FT_Err_Invalid_Version,
    LowerModuleVersion          = ffi::FT_Err_Lower_Module_Version,
    InvalidArgument             = ffi::FT_Err_Invalid_Argument,
    UnimplementedFeature        = ffi::FT_Err_Unimplemented_Feature,
    InvalidTable                = ffi::FT_Err_Invalid_Table,
    InvalidOffset               = ffi::FT_Err_Invalid_Offset,
    ArrayTooLarge               = ffi::FT_Err_Array_Too_Large,
    MissingModule               = ffi::FT_Err_Missing_Module,
    MissingProperty             = ffi::FT_Err_Missing_Property,
    InvalidGlyphIndex           = ffi::FT_Err_Invalid_Glyph_Index,
    InvalidCharacterCode        = ffi::FT_Err_Invalid_Character_Code,
    InvalidGlyphFormat          = ffi::FT_Err_Invalid_Glyph_Format,
    CannotRenderGlyph           = ffi::FT_Err_Cannot_Render_Glyph,
    InvalidOutline              = ffi::FT_Err_Invalid_Outline,
    InvalidComposite            = ffi::FT_Err_Invalid_Composite,
    TooManyHints                = ffi::FT_Err_Too_Many_Hints,
    InvalidPixelSize            = ffi::FT_Err_Invalid_Pixel_Size,
    InvalidHandle               = ffi::FT_Err_Invalid_Handle,
    InvalidLibraryHandle        = ffi::FT_Err_Invalid_Library_Handle,
    InvalidDriverHandle         = ffi::FT_Err_Invalid_Driver_Handle,
    InvalidFaceHandle           = ffi::FT_Err_Invalid_Face_Handle,
    InvalidSizeHandle           = ffi::FT_Err_Invalid_Size_Handle,
    InvalidSlotHandle           = ffi::FT_Err_Invalid_Slot_Handle,
    InvalidCharMapHandle        = ffi::FT_Err_Invalid_CharMap_Handle,
    InvalidCacheHandle          = ffi::FT_Err_Invalid_Cache_Handle,
    InvalidStreamHandle         = ffi::FT_Err_Invalid_Stream_Handle,
    TooManyDrivers              = ffi::FT_Err_Too_Many_Drivers,
    TooManyExtensions           = ffi::FT_Err_Too_Many_Extensions,
    OutOfMemory                 = ffi::FT_Err_Out_Of_Memory,
    UnlistedObject              = ffi::FT_Err_Unlisted_Object,
    CannotOpenStream            = ffi::FT_Err_Cannot_Open_Stream,
    InvalidStreamSeek           = ffi::FT_Err_Invalid_Stream_Seek,
    InvalidStreamSkip           = ffi::FT_Err_Invalid_Stream_Skip,
    InvalidStreamRead           = ffi::FT_Err_Invalid_Stream_Read,
    InvalidStreamOperation      = ffi::FT_Err_Invalid_Stream_Operation,
    InvalidFrameOperation       = ffi::FT_Err_Invalid_Frame_Operation,
    NestedFrameAccess           = ffi::FT_Err_Nested_Frame_Access,
    InvalidFrameRead            = ffi::FT_Err_Invalid_Frame_Read,
    RasterUninitialized         = ffi::FT_Err_Raster_Uninitialized,
    RasterCorrupted             = ffi::FT_Err_Raster_Corrupted,
    RasterOverflow              = ffi::FT_Err_Raster_Overflow,
    RasterNegativeHeight        = ffi::FT_Err_Raster_Negative_Height,
    TooManyCaches               = ffi::FT_Err_Too_Many_Caches,
    InvalidOpcode               = ffi::FT_Err_Invalid_Opcode,
    TooFewArguments             = ffi::FT_Err_Too_Few_Arguments,
    StackOverflow               = ffi::FT_Err_Stack_Overflow,
    CodeOverflow                = ffi::FT_Err_Code_Overflow,
    BadArgument                 = ffi::FT_Err_Bad_Argument,
    DivideByZero                = ffi::FT_Err_Divide_By_Zero,
    InvalidReference            = ffi::FT_Err_Invalid_Reference,
    DebugOpCode                 = ffi::FT_Err_Debug_OpCode,
    ENDFInExecStream            = ffi::FT_Err_ENDF_In_Exec_Stream,
    NestedDEFS                  = ffi::FT_Err_Nested_DEFS,
    InvalidCodeRange            = ffi::FT_Err_Invalid_CodeRange,
    ExecutionTooLong            = ffi::FT_Err_Execution_Too_Long,
    TooManyFunctionDefs         = ffi::FT_Err_Too_Many_Function_Defs,
    TooManyInstructionDefs      = ffi::FT_Err_Too_Many_Instruction_Defs,
    TableMissing                = ffi::FT_Err_Table_Missing,
    HorizHeaderMissing          = ffi::FT_Err_Horiz_Header_Missing,
    LocationsMissing            = ffi::FT_Err_Locations_Missing,
    NameTableMissing            = ffi::FT_Err_Name_Table_Missing,
    CMapTableMissing            = ffi::FT_Err_CMap_Table_Missing,
    HmtxTableMissing            = ffi::FT_Err_Hmtx_Table_Missing,
    PostTableMissing            = ffi::FT_Err_Post_Table_Missing,
    InvalidHorizMetrics         = ffi::FT_Err_Invalid_Horiz_Metrics,
    InvalidCharMapFormat        = ffi::FT_Err_Invalid_CharMap_Format,
    InvalidPPem                 = ffi::FT_Err_Invalid_PPem,
    InvalidVertMetrics          = ffi::FT_Err_Invalid_Vert_Metrics,
    CouldNotFindContext         = ffi::FT_Err_Could_Not_Find_Context,
    InvalidPostTableFormat      = ffi::FT_Err_Invalid_Post_Table_Format,
    InvalidPostTable            = ffi::FT_Err_Invalid_Post_Table,
    SyntaxError                 = ffi::FT_Err_Syntax_Error,
    StackUnderflow              = ffi::FT_Err_Stack_Underflow,
    Ignore                      = ffi::FT_Err_Ignore,
    NoUnicodeGlyphName          = ffi::FT_Err_No_Unicode_Glyph_Name,
    MissingStartfontField       = ffi::FT_Err_Missing_Startfont_Field,
    MissingFontField            = ffi::FT_Err_Missing_Font_Field,
    MissingSizeField            = ffi::FT_Err_Missing_Size_Field,
    MissingFontboundingboxField = ffi::FT_Err_Missing_Fontboundingbox_Field,
    MissingCharsField           = ffi::FT_Err_Missing_Chars_Field,
    MissingStartcharField       = ffi::FT_Err_Missing_Startchar_Field,
    MissingEncodingField        = ffi::FT_Err_Missing_Encoding_Field,
    MissingBbxField             = ffi::FT_Err_Missing_Bbx_Field,
    BbxTooBig                   = ffi::FT_Err_Bbx_Too_Big,
    CorruptedFontHeader         = ffi::FT_Err_Corrupted_Font_Header,
    CorruptedFontGlyphs         = ffi::FT_Err_Corrupted_Font_Glyphs,
    Max                         = ffi::FT_Err_Max,
    UnknownError,
}

impl FromPrimitive for Error {
    fn from_i64(n: i64) -> Option<Error> {
        FromPrimitive::from_i32(n as i32)
    }

    fn from_u64(n: u64) -> Option<Error> {
        FromPrimitive::from_i32(n as i32)
    }

    #[allow(non_upper_case_globals)]
    fn from_i32(n: i32) -> Option<Error> {
        match n {
            ffi::FT_Err_Ok                            => Some(Error::Ok),
            ffi::FT_Err_Cannot_Open_Resource          => Some(Error::CannotOpenResource),
            ffi::FT_Err_Unknown_File_Format           => Some(Error::UnknownFileFormat),
            ffi::FT_Err_Invalid_File_Format           => Some(Error::InvalidFileFormat),
            ffi::FT_Err_Invalid_Version               => Some(Error::InvalidVersion),
            ffi::FT_Err_Lower_Module_Version          => Some(Error::LowerModuleVersion),
            ffi::FT_Err_Invalid_Argument              => Some(Error::InvalidArgument),
            ffi::FT_Err_Unimplemented_Feature         => Some(Error::UnimplementedFeature),
            ffi::FT_Err_Invalid_Table                 => Some(Error::InvalidTable),
            ffi::FT_Err_Invalid_Offset                => Some(Error::InvalidOffset),
            ffi::FT_Err_Array_Too_Large               => Some(Error::ArrayTooLarge),
            ffi::FT_Err_Missing_Module                => Some(Error::MissingModule),
            ffi::FT_Err_Missing_Property              => Some(Error::MissingProperty),
            ffi::FT_Err_Invalid_Glyph_Index           => Some(Error::InvalidGlyphIndex),
            ffi::FT_Err_Invalid_Character_Code        => Some(Error::InvalidCharacterCode),
            ffi::FT_Err_Invalid_Glyph_Format          => Some(Error::InvalidGlyphFormat),
            ffi::FT_Err_Cannot_Render_Glyph           => Some(Error::CannotRenderGlyph),
            ffi::FT_Err_Invalid_Outline               => Some(Error::InvalidOutline),
            ffi::FT_Err_Invalid_Composite             => Some(Error::InvalidComposite),
            ffi::FT_Err_Too_Many_Hints                => Some(Error::TooManyHints),
            ffi::FT_Err_Invalid_Pixel_Size            => Some(Error::InvalidPixelSize),
            ffi::FT_Err_Invalid_Handle                => Some(Error::InvalidHandle),
            ffi::FT_Err_Invalid_Library_Handle        => Some(Error::InvalidLibraryHandle),
            ffi::FT_Err_Invalid_Driver_Handle         => Some(Error::InvalidDriverHandle),
            ffi::FT_Err_Invalid_Face_Handle           => Some(Error::InvalidFaceHandle),
            ffi::FT_Err_Invalid_Size_Handle           => Some(Error::InvalidSizeHandle),
            ffi::FT_Err_Invalid_Slot_Handle           => Some(Error::InvalidSlotHandle),
            ffi::FT_Err_Invalid_CharMap_Handle        => Some(Error::InvalidCharMapHandle),
            ffi::FT_Err_Invalid_Cache_Handle          => Some(Error::InvalidCacheHandle),
            ffi::FT_Err_Invalid_Stream_Handle         => Some(Error::InvalidStreamHandle),
            ffi::FT_Err_Too_Many_Drivers              => Some(Error::TooManyDrivers),
            ffi::FT_Err_Too_Many_Extensions           => Some(Error::TooManyExtensions),
            ffi::FT_Err_Out_Of_Memory                 => Some(Error::OutOfMemory),
            ffi::FT_Err_Unlisted_Object               => Some(Error::UnlistedObject),
            ffi::FT_Err_Cannot_Open_Stream            => Some(Error::CannotOpenStream),
            ffi::FT_Err_Invalid_Stream_Seek           => Some(Error::InvalidStreamSeek),
            ffi::FT_Err_Invalid_Stream_Skip           => Some(Error::InvalidStreamSkip),
            ffi::FT_Err_Invalid_Stream_Read           => Some(Error::InvalidStreamRead),
            ffi::FT_Err_Invalid_Stream_Operation      => Some(Error::InvalidStreamOperation),
            ffi::FT_Err_Invalid_Frame_Operation       => Some(Error::InvalidFrameOperation),
            ffi::FT_Err_Nested_Frame_Access           => Some(Error::NestedFrameAccess),
            ffi::FT_Err_Invalid_Frame_Read            => Some(Error::InvalidFrameRead),
            ffi::FT_Err_Raster_Uninitialized          => Some(Error::RasterUninitialized),
            ffi::FT_Err_Raster_Corrupted              => Some(Error::RasterCorrupted),
            ffi::FT_Err_Raster_Overflow               => Some(Error::RasterOverflow),
            ffi::FT_Err_Raster_Negative_Height        => Some(Error::RasterNegativeHeight),
            ffi::FT_Err_Too_Many_Caches               => Some(Error::TooManyCaches),
            ffi::FT_Err_Invalid_Opcode                => Some(Error::InvalidOpcode),
            ffi::FT_Err_Too_Few_Arguments             => Some(Error::TooFewArguments),
            ffi::FT_Err_Stack_Overflow                => Some(Error::StackOverflow),
            ffi::FT_Err_Code_Overflow                 => Some(Error::CodeOverflow),
            ffi::FT_Err_Bad_Argument                  => Some(Error::BadArgument),
            ffi::FT_Err_Divide_By_Zero                => Some(Error::DivideByZero),
            ffi::FT_Err_Invalid_Reference             => Some(Error::InvalidReference),
            ffi::FT_Err_Debug_OpCode                  => Some(Error::DebugOpCode),
            ffi::FT_Err_ENDF_In_Exec_Stream           => Some(Error::ENDFInExecStream),
            ffi::FT_Err_Nested_DEFS                   => Some(Error::NestedDEFS),
            ffi::FT_Err_Invalid_CodeRange             => Some(Error::InvalidCodeRange),
            ffi::FT_Err_Execution_Too_Long            => Some(Error::ExecutionTooLong),
            ffi::FT_Err_Too_Many_Function_Defs        => Some(Error::TooManyFunctionDefs),
            ffi::FT_Err_Too_Many_Instruction_Defs     => Some(Error::TooManyInstructionDefs),
            ffi::FT_Err_Table_Missing                 => Some(Error::TableMissing),
            ffi::FT_Err_Horiz_Header_Missing          => Some(Error::HorizHeaderMissing),
            ffi::FT_Err_Locations_Missing             => Some(Error::LocationsMissing),
            ffi::FT_Err_Name_Table_Missing            => Some(Error::NameTableMissing),
            ffi::FT_Err_CMap_Table_Missing            => Some(Error::CMapTableMissing),
            ffi::FT_Err_Hmtx_Table_Missing            => Some(Error::HmtxTableMissing),
            ffi::FT_Err_Post_Table_Missing            => Some(Error::PostTableMissing),
            ffi::FT_Err_Invalid_Horiz_Metrics         => Some(Error::InvalidHorizMetrics),
            ffi::FT_Err_Invalid_CharMap_Format        => Some(Error::InvalidCharMapFormat),
            ffi::FT_Err_Invalid_PPem                  => Some(Error::InvalidPPem),
            ffi::FT_Err_Invalid_Vert_Metrics          => Some(Error::InvalidVertMetrics),
            ffi::FT_Err_Could_Not_Find_Context        => Some(Error::CouldNotFindContext),
            ffi::FT_Err_Invalid_Post_Table_Format     => Some(Error::InvalidPostTableFormat),
            ffi::FT_Err_Invalid_Post_Table            => Some(Error::InvalidPostTable),
            ffi::FT_Err_Syntax_Error                  => Some(Error::SyntaxError),
            ffi::FT_Err_Stack_Underflow               => Some(Error::StackUnderflow),
            ffi::FT_Err_Ignore                        => Some(Error::Ignore),
            ffi::FT_Err_No_Unicode_Glyph_Name         => Some(Error::NoUnicodeGlyphName),
            ffi::FT_Err_Missing_Startfont_Field       => Some(Error::MissingStartfontField),
            ffi::FT_Err_Missing_Font_Field            => Some(Error::MissingFontField),
            ffi::FT_Err_Missing_Size_Field            => Some(Error::MissingSizeField),
            ffi::FT_Err_Missing_Fontboundingbox_Field => Some(Error::MissingFontboundingboxField),
            ffi::FT_Err_Missing_Chars_Field           => Some(Error::MissingCharsField),
            ffi::FT_Err_Missing_Startchar_Field       => Some(Error::MissingStartcharField),
            ffi::FT_Err_Missing_Encoding_Field        => Some(Error::MissingEncodingField),
            ffi::FT_Err_Missing_Bbx_Field             => Some(Error::MissingBbxField),
            ffi::FT_Err_Bbx_Too_Big                   => Some(Error::BbxTooBig),
            ffi::FT_Err_Corrupted_Font_Header         => Some(Error::CorruptedFontHeader),
            ffi::FT_Err_Corrupted_Font_Glyphs         => Some(Error::CorruptedFontGlyphs),
            ffi::FT_Err_Max                           => Some(Error::Max),
            _ => Some(Error::UnknownError),
        }
    }
}

