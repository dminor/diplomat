use core::panic;
use std::alloc::Layout;
use std::{cmp::max, collections::HashMap};

use diplomat_core::ast::{self, PrimitiveType, TypeName};

// TODO(#58): support non-32-bit platforms
use u32 as usize_target;

pub fn struct_offsets_size_max_align(
    strct: &ast::Struct,
    in_path: &ast::Path,
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
) -> (Vec<usize>, Layout) {
    let mut max_align = 0;
    let mut next_offset = 0;
    let mut offsets = vec![];

    for (_, typ, _) in &strct.fields {
        let size_align = type_size_alignment(typ, in_path, env);
        let size = size_align.size();
        let align = size_align.align();
        max_align = max(max_align, align);
        let padding = (align - (next_offset % align)) % align;
        next_offset += padding;
        offsets.push(next_offset);
        next_offset += size;
    }

    (
        offsets,
        Layout::from_size_align(next_offset, max_align).unwrap(),
    )
}

pub fn result_ok_offset_size_align(
    ok: &TypeName,
    err: &TypeName,
    in_path: &ast::Path,
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
) -> (usize, Layout) {
    let ok_size_align = type_size_alignment(ok, in_path, env);
    let err_size_align = type_size_alignment(err, in_path, env);
    let (offsets, size_max_align) = struct_offsets_size_max_align(
        &ast::Struct {
            name: "".to_string(),
            doc_lines: "".to_string(),
            fields: vec![
                if ok_size_align.size() > err_size_align.size() {
                    ("".to_string(), ok.clone(), "".to_string())
                } else {
                    ("".to_string(), err.clone(), "".to_string())
                },
                (
                    "".to_string(),
                    ast::TypeName::Primitive(PrimitiveType::bool),
                    "".to_string(),
                ),
            ],
            methods: vec![],
        },
        in_path,
        env,
    );
    (offsets[1], size_max_align)
}

pub fn type_size_alignment(
    typ: &ast::TypeName,
    in_path: &ast::Path,
    env: &HashMap<ast::Path, HashMap<String, ast::ModSymbol>>,
) -> Layout {
    match typ {
        // TODO(#58): support non-32-bit platforms
        // Actual:
        // ast::TypeName::Box(_) => Layout::new::<Box<()>>(),
        // ast::TypeName::Reference(_, _) => Layout::new::<&()>(),
        // Temporary:
        ast::TypeName::Box(_) => Layout::new::<usize_target>(),
        ast::TypeName::Reference(_, _) => Layout::new::<usize_target>(),
        ast::TypeName::Option(underlying) => match underlying.as_ref() {
            ast::TypeName::Box(_) => type_size_alignment(underlying, in_path, env),
            _ => todo!(),
        },
        ast::TypeName::Result(ok, err) => {
            let (_, size_align) = result_ok_offset_size_align(ok, err, in_path, env);
            size_align
        }
        ast::TypeName::Named(_) => match typ.resolve(in_path, env) {
            ast::CustomType::Struct(strct) => {
                let (_, size_max_align) = struct_offsets_size_max_align(strct, in_path, env);
                size_max_align
            }

            ast::CustomType::Enum(_) => {
                // repr(C) fieldless enums use the default platform representation: isize
                Layout::new::<usize_target>()
            }

            ast::CustomType::Opaque(_) => {
                panic!("Size of opaque types is unknown")
            }
        },
        ast::TypeName::Primitive(p) => primitive_size_alignment(*p),
        // TODO(#58): support non-32-bit platforms
        // Actual:
        // ast::TypeName::StrReference => Layout::new::<&str>(),
        // ast::TypeName::PrimitiveSlice(_) => Layout::new::<&[u8]>(),
        // Temporary:
        ast::TypeName::StrReference => Layout::new::<(usize_target, usize_target)>(),
        ast::TypeName::PrimitiveSlice(_) => Layout::new::<(usize_target, usize_target)>(),
        ast::TypeName::Writeable => panic!(),
        ast::TypeName::Unit => Layout::new::<()>(),
    }
}

pub fn primitive_size_alignment(prim: PrimitiveType) -> Layout {
    match prim {
        ast::PrimitiveType::bool => Layout::new::<bool>(),
        ast::PrimitiveType::char => Layout::new::<char>(),
        ast::PrimitiveType::i8 | ast::PrimitiveType::u8 => Layout::new::<u8>(),
        ast::PrimitiveType::i16 | ast::PrimitiveType::u16 => Layout::new::<u16>(),
        ast::PrimitiveType::i32 | ast::PrimitiveType::u32 => Layout::new::<u32>(),
        ast::PrimitiveType::i64 | ast::PrimitiveType::u64 => Layout::new::<u64>(),
        ast::PrimitiveType::i128 | ast::PrimitiveType::u128 => Layout::new::<u128>(),
        ast::PrimitiveType::isize | ast::PrimitiveType::usize => Layout::new::<usize_target>(),
        ast::PrimitiveType::f32 => Layout::new::<f32>(),
        ast::PrimitiveType::f64 => Layout::new::<f64>(),
    }
}
