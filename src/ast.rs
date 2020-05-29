extern crate swc_ecma_ast;
extern crate swc_ecma_parser;

pub mod ast_printer {
    use std::vec::Vec;
    use swc_ecma_ast::ModuleItem;
    use swc_ecma_ast::{
        BlockStmt, BreakStmt, ContinueStmt, DebuggerStmt, Decl, DoWhileStmt, EmptyStmt, ExportAll,
        ExportDecl, ExportDefaultDecl, ExportDefaultExpr, ExprStmt, ForInStmt, ForOfStmt, ForStmt,
        IfStmt, ImportDecl, LabeledStmt, ModuleDecl, NamedExport, ReturnStmt, Stmt, SwitchStmt,
        ThrowStmt, TryStmt, TsExportAssignment, TsImportEqualsDecl, TsNamespaceExportDecl,
        WhileStmt, WithStmt,
    };

    pub fn expand_module_item_vector(module_vec: Vec<ModuleItem>) {
        for item in module_vec.iter() {
            if let ModuleItem::Stmt(stmt) = item {
                println!("{:?}", "Stmt: ");
                match stmt {
                    Stmt::Block(stmt_subtype) => {
                        println!("Block(BlockStmt): {:?}", stmt_subtype);
                    }
                    Stmt::Empty(stmt_subtype) => {
                        println!("Empty(EmptyStmt): {:?}", stmt_subtype);
                    }
                    Stmt::Debugger(stmt_subtype) => {
                        println!("Debugger(DebuggerStmt): {:?}", stmt_subtype);
                    }
                    Stmt::With(stmt_subtype) => {
                        println!("With(WithStmt): {:?}", stmt_subtype);
                    }
                    Stmt::Return(stmt_subtype) => {
                        println!("Return(ReturnStmt): {:?}", stmt_subtype);
                    }
                    Stmt::Labeled(stmt_subtype) => {
                        println!("Labeled(LabeledStmt): {:?}", stmt_subtype);
                    }
                    Stmt::Break(stmt_subtype) => {
                        println!("Break(BreakStmt): {:?}", stmt_subtype);
                    }
                    Stmt::Continue(stmt_subtype) => {
                        println!("Continue(ContinueStmt): {:?}", stmt_subtype);
                    }
                    Stmt::If(stmt_subtype) => {
                        println!("If(IfStmt): {:?}", stmt_subtype);
                    }
                    Stmt::Switch(stmt_subtype) => {
                        println!("Switch(SwitchStmt): {:?}", stmt_subtype);
                    }
                    Stmt::Throw(stmt_subtype) => {
                        println!("Throw(ThrowStmt): {:?}", stmt_subtype);
                    }
                    Stmt::Try(stmt_subtype) => {
                        println!("Try(TryStmt): {:?}", stmt_subtype);
                    }
                    Stmt::While(stmt_subtype) => {
                        println!("While(WhileStmt): {:?}", stmt_subtype);
                    }
                    Stmt::DoWhile(stmt_subtype) => {
                        println!("DoWhile(DoWhileStmt): {:?}", stmt_subtype);
                    }
                    Stmt::For(stmt_subtype) => {
                        println!("For(ForStmt): {:?}", stmt_subtype);
                    }
                    Stmt::ForIn(stmt_subtype) => {
                        println!("ForIn(ForInStmt): {:?}", stmt_subtype);
                    }
                    Stmt::ForOf(stmt_subtype) => {
                        println!("ForOf(ForOfStmt): {:?}", stmt_subtype);
                    }
                    Stmt::Decl(stmt_subtype) => {
                        println!("Decl(Decl): {:?}", stmt_subtype);
                    }
                    Stmt::Expr(stmt_subtype) => {
                        println!("Expr(ExprStmt): {:?}", stmt_subtype);
                    }
                }
            }
            if let ModuleItem::ModuleDecl(stmt) = item {
                // stmt is of enum type ModuleDecl
                println!("{:?}", "ModuleDecl: ");
                match stmt {
                    ModuleDecl::Import(stmt_subtype) => {
                        println!("ImportDecl: {:?}", stmt_subtype);
                    }
                    ModuleDecl::ExportDecl(stmt_subtype) => {
                        println!("ExportDecl: {:?}", stmt_subtype);
                    }
                    ModuleDecl::ExportNamed(stmt_subtype) => {
                        println!("NamedExport: {:?}", stmt_subtype);
                    }
                    ModuleDecl::ExportDefaultDecl(stmt_subtype) => {
                        println!("ExportDefaultDecl: {:?}", stmt_subtype);
                    }
                    ModuleDecl::ExportDefaultExpr(stmt_subtype) => {
                        println!("ExportDefaultExpr: {:?}", stmt_subtype);
                    }
                    ModuleDecl::ExportAll(stmt_subtype) => {
                        println!("ExportAll: {:?}", stmt_subtype);
                    }
                    ModuleDecl::TsImportEquals(stmt_subtype) => {
                        println!("TsImportEqualsDecl: {:?}", stmt_subtype);
                    }
                    ModuleDecl::TsExportAssignment(stmt_subtype) => {
                        println!("TsExportAssignment: {:?}", stmt_subtype);
                    }
                    ModuleDecl::TsNamespaceExport(stmt_subtype) => {
                        println!("TsNamespaceExportDecl: {:?}", stmt_subtype);
                    }
                }
            }
        }
    }
}

pub mod ast_graph {
    use petgraph::dot::{Config, Dot};
    use petgraph::graph::{DefaultIx, DiGraph, NodeIndex};
    use std::vec::Vec;
    use swc_ecma_ast::{ModuleItem, Stmt};

    pub fn module_item_vector_to_digraph<'a>(
        module_vec: &'a Vec<ModuleItem>,
    ) -> DiGraph<
        &'a swc_ecma_ast::ModuleItem,
        (
            &'a swc_ecma_ast::ModuleItem,
            &'a swc_ecma_ast::ModuleItem,
        ),
    > {
        let mut digraph_module_items = DiGraph::<
                &'a swc_ecma_ast::ModuleItem,
                (&'a swc_ecma_ast::ModuleItem, &'a swc_ecma_ast::ModuleItem),
            >::with_capacity(0, 0);
        let vertex_count = module_vec.len();
        if vertex_count > 0 {
            let edge_count = vertex_count - 1;
            let mut edges_vec: Vec<(&swc_ecma_ast::ModuleItem, &swc_ecma_ast::ModuleItem)> =
                Vec::with_capacity(edge_count);
            let mut node_index_vec: Vec<NodeIndex> = Vec::with_capacity(vertex_count);
            digraph_module_items = DiGraph::<
                &'a swc_ecma_ast::ModuleItem,
                (&'a swc_ecma_ast::ModuleItem, &'a swc_ecma_ast::ModuleItem),
            >::with_capacity(vertex_count, edge_count);
            let mut node_a = NodeIndex::new(0);
            let mut node_b = NodeIndex::new(0);
            if vertex_count > 1 {
                for i in 0..edge_count {
                    if i == 0 {
                        node_a = digraph_module_items.add_node(&module_vec[i]);
                        node_b = digraph_module_items.add_node(&module_vec[i + 1]);
                        digraph_module_items.add_edge(
                            node_a,
                            node_b,
                            (&module_vec[i], &module_vec[i + 1]),
                        );
                    } else {
                        node_a = node_b;
                        node_b = digraph_module_items.add_node(&module_vec[i + 1]);
                        digraph_module_items.add_edge(
                            node_a,
                            node_b,
                            (&module_vec[i], &module_vec[i + 1]),
                        );
                    }
                }
            } else {
                digraph_module_items.add_node(&module_vec[0]);
            }
            println!(
                "{:?}",
                Dot::with_config(&digraph_module_items, &[Config::EdgeNoLabel])
            );
        }
        digraph_module_items
    }
}
