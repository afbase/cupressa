#[macro_use]
extern crate swc_common;
extern crate swc_ecma_parser;
extern crate cupressa;
use cupressa::ast::ast_printer::expand_module_item_vector;
use std::sync::Arc;
use swc_common::{
    errors::{ColorConfig, Handler},
    FileName, FilePathMapping, SourceMap,
};
use swc_ecma_parser::{lexer::Lexer, Parser, Session, SourceFileInput, Syntax};
use std::path::Path;

fn main() {
    swc_common::GLOBALS.set(&swc_common::Globals::new(), || {
        let cm: Arc<SourceMap> = Default::default();
        let handler =
            Handler::with_tty_emitter(ColorConfig::Auto, true, false,
Some(cm.clone()));

        let session = Session { handler: &handler };

        // Real usage
        let fm = cm
            .load_file(Path::new("test.js"))
            .expect("failed to load test.js");


        // let fm = cm.new_source_file(
        //     FileName::Custom("test.js".into()),
        //     "function foo() {}".into(),
        // );
        let lexer = Lexer::new(
            session,
            Syntax::Es(Default::default()),
             Default::default(),
            SourceFileInput::from(&*fm),
            None,
        );

        let mut parser = Parser::new_from(session, lexer);


        let module = parser
            .parse_module()
            .map_err(|mut e| {
                e.emit();
                ()
            })
            .expect("failed to parser module");
        let module_body = module.body;
        println!("{} {}","module length:", module_body.len());
        expand_module_item_vector(module_body);
        // for i in 0..module_body.len() {
        //     println!("{:?}",module_body[i]);
        // }
    });
}