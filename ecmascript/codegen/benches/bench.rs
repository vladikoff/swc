#![feature(box_syntax)]
#![feature(test)]

extern crate sourcemap;
extern crate swc_common;
extern crate swc_ecma_codegen;
extern crate swc_ecma_parser;
extern crate test;
extern crate testing;

use sourcemap::SourceMapBuilder;
use swc_common::FileName;
use swc_ecma_codegen::Emitter;
use swc_ecma_parser::{Parser, Session, SourceFileInput};
use test::Bencher;

const SOURCE: &str = r#"
'use strict';
/**
 * Extract red color out of a color integer:
 *
 * 0x00DEAD -> 0x00
 *
 * @param  {Number} color
 * @return {Number}
 */
function red( color )
{
    let foo = 3.14;
    return color >> 16;
}
/**
 * Extract green out of a color integer:
 *
 * 0x00DEAD -> 0xDE
 *
 * @param  {Number} color
 * @return {Number}
 */
function green( color )
{
    return ( color >> 8 ) & 0xFF;
}
/**
 * Extract blue color out of a color integer:
 *
 * 0x00DEAD -> 0xAD
 *
 * @param  {Number} color
 * @return {Number}
 */
function blue( color )
{
    return color & 0xFF;
}
/**
 * Converts an integer containing a color such as 0x00DEAD to a hex
 * string, such as '#00DEAD';
 *
 * @param  {Number} int
 * @return {String}
 */
function intToHex( int )
{
    const mask = '#000000';
    const hex = int.toString( 16 );
    return mask.substring( 0, 7 - hex.length ) + hex;
}
/**
 * Converts a hex string containing a color such as '#00DEAD' to
 * an integer, such as 0x00DEAD;
 *
 * @param  {Number} num
 * @return {String}
 */
function hexToInt( hex )
{
    return parseInt( hex.substring( 1 ), 16 );
}
module.exports = {
    red,
    green,
    blue,
    intToHex,
    hexToInt,
};
"#;

#[bench]
fn emit_colors(b: &mut Bencher) {
    b.bytes = SOURCE.len() as _;

    let _ = ::testing::run_test(|cm, handler| {
        let session = Session {
            handler: &handler,
            cfg: Default::default(),
        };
        let fm = cm.new_source_file(FileName::Anon(0), SOURCE.into());
        let mut parser = Parser::new(session, SourceFileInput::from(&*fm));
        let module = parser.parse_module().unwrap();

        b.iter(|| {
            let buf = vec![];
            let mut src_map_builder = SourceMapBuilder::new(None);
            {
                let handlers = box MyHandlers;
                let mut emitter = Emitter {
                    cfg: swc_ecma_codegen::Config {
                        ..Default::default()
                    },
                    cm: cm.clone(),
                    wr: box swc_ecma_codegen::text_writer::JsWriter::new(
                        cm.clone(),
                        "\n",
                        buf,
                        &mut src_map_builder,
                    ),
                    handlers,
                    pos_of_leading_comments: Default::default(),
                };

                emitter.emit_module(&module)
            }
        });
        Ok(())
    });
}

struct MyHandlers;

impl swc_ecma_codegen::Handlers for MyHandlers {}
