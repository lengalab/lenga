use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

use proto::lenga_server::{Lenga, LengaServer};
use proto::{
    AssignmentExpr, Ast, CallExpr, CompStmt, DeclNode, DeclRefExpr, EditRequest, ExprNode,
    FuncDecl, IdentifierExpr, IncludeDecl, InitRequest, LiteralExpr, Node, OpenRequest, ParamDecl,
    ReturnStmt, StmtNode, UnknownNode, VarDecl, Void, decl_node, expr_node, node, stmt_node,
};
use tonic::{Request, Response, Status, transport::Server};

mod proto {
    tonic::include_proto!("lenga");
}

pub fn read_file_to_string(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_default()
}

#[derive(Debug, Default)]
pub struct LengaService {
    asts: Arc<Mutex<HashMap<String, Ast>>>,
}

#[tonic::async_trait]
impl Lenga for LengaService {
    async fn initialize(&self, _request: Request<InitRequest>) -> Result<Response<Void>, Status> {
        Ok(Response::new(Void {}))
    }

    async fn open_file(&self, request: Request<OpenRequest>) -> Result<Response<Ast>, Status> {
        let req = request.into_inner();

        let mut asts = self.asts.lock().unwrap();
        let ast = match asts.get(&req.path) {
            Some(file_ast) => file_ast.clone(),
            None => {
                let file_ast = read_ast_mock();
                asts.insert(req.path.clone(), file_ast.clone());
                file_ast
            }
        };

        Ok(Response::new(ast))
    }

    async fn edit(&self, request: Request<EditRequest>) -> Result<Response<Node>, Status> {
        let req = request.into_inner();

        let res = Node {
            node: Some(node::Node::UnknownNode(UnknownNode {
                id: "edit".to_string(),
                contents: req.edit_data,
            })),
        };

        Ok(Response::new(res))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:49200".parse()?;
    let lenga_service = LengaService::default();

    println!("Starting gRPC server on {}", addr);

    Server::builder()
        .add_service(LengaServer::new(lenga_service))
        .serve(addr)
        .await?;

    Ok(())
}

// ================================== //

fn read_ast_mock() -> Ast {
    // #include <stdio.h>
    let include = Node {
        node: Some(node::Node::DeclNode(DeclNode {
            declaration: Some(decl_node::Declaration::IncludeDecl(IncludeDecl {
                id: "0".into(),
                directive: "<stdio.h>".into(),
            })),
        })),
    };

    // int first(int a, int b, int c) { ... }
    let params = vec![
        ParamDecl {
            id: "10".into(),
            name: "a".into(),
            data_type: "int".into(),
        },
        ParamDecl {
            id: "11".into(),
            name: "b".into(),
            data_type: "int".into(),
        },
        ParamDecl {
            id: "12".into(),
            name: "c".into(),
            data_type: "int".into(),
        },
    ];

    let var_first = Node {
        node: Some(node::Node::DeclNode(DeclNode {
            declaration: Some(decl_node::Declaration::VarDecl(VarDecl {
                id: "13".into(),
                name: "first_parameter".into(),
                data_type: "int".into(),
                initializer: Some(ExprNode {
                    expression: Some(expr_node::Expression::DeclRefExpr(DeclRefExpr {
                        id: "130".into(),
                        decl_ref_id: "10".into(),
                    })),
                }),
            })),
        })),
    };

    let ret_first = Node {
        node: Some(node::Node::StmtNode(StmtNode {
            statement: Some(stmt_node::Statement::ReturnStmt(ReturnStmt {
                id: "14".into(),
                expression: Some(ExprNode {
                    expression: Some(expr_node::Expression::DeclRefExpr(DeclRefExpr {
                        id: "140".into(),
                        decl_ref_id: "13".into(),
                    })),
                }),
            })),
        })),
    };

    let func_first = Node {
        node: Some(node::Node::DeclNode(DeclNode {
            declaration: Some(decl_node::Declaration::FuncDecl(FuncDecl {
                id: "1".into(),
                name: "first".into(),
                return_type: "int".into(),
                params: params.into_iter().collect(),
                body: Some(CompStmt {
                    id: "100".into(),
                    statements: vec![var_first, ret_first],
                }),
            })),
        })),
    };

    // int main() { ... }
    let var_a = make_var("20", "a", "int", "1");
    let var_b = make_var("21", "b", "int", "2");
    let var_c = make_var("22", "c", "int", "3");

    let call_first = ExprNode {
        expression: Some(expr_node::Expression::CallExpr(Box::new(CallExpr {
            id: "230".into(),
            calle: Some(Box::new(ExprNode {
                expression: Some(expr_node::Expression::DeclRefExpr(DeclRefExpr {
                    id: "231".into(),
                    decl_ref_id: "1".into(),
                })),
            })),
            args: vec![
                ExprNode {
                    expression: Some(expr_node::Expression::DeclRefExpr(DeclRefExpr {
                        id: "232".into(),
                        decl_ref_id: "20".into(),
                    })),
                },
                ExprNode {
                    expression: Some(expr_node::Expression::DeclRefExpr(DeclRefExpr {
                        id: "233".into(),
                        decl_ref_id: "21".into(),
                    })),
                },
                ExprNode {
                    expression: Some(expr_node::Expression::DeclRefExpr(DeclRefExpr {
                        id: "234".into(),
                        decl_ref_id: "22".into(),
                    })),
                },
            ],
        }))),
    };

    let var_result = Node {
        node: Some(node::Node::DeclNode(DeclNode {
            declaration: Some(decl_node::Declaration::VarDecl(VarDecl {
                id: "23".into(),
                name: "result".into(),
                data_type: "int".into(),
                initializer: Some(call_first),
            })),
        })),
    };

    let func_main = Node {
        node: Some(node::Node::DeclNode(DeclNode {
            declaration: Some(decl_node::Declaration::FuncDecl(FuncDecl {
                id: "2".into(),
                name: "main".into(),
                return_type: "int".into(),
                params: vec![],
                body: Some(CompStmt {
                    id: "200".into(),
                    statements: vec![var_a, var_b, var_c, var_result],
                }),
            })),
        })),
    };

    Ast {
        nodes: vec![include, func_first, func_main],
    }
}

/// Helper to build a VarDecl with a Literal initializer
fn make_var(id: &str, name: &str, data_type: &str, value: &str) -> Node {
    Node {
        node: Some(node::Node::DeclNode(DeclNode {
            declaration: Some(decl_node::Declaration::VarDecl(VarDecl {
                id: id.into(),
                name: name.into(),
                data_type: data_type.into(),
                initializer: Some(ExprNode {
                    expression: Some(expr_node::Expression::LiteralExpr(LiteralExpr {
                        id: format!("{}_lit", id),
                        data_type: data_type.into(),
                        value: value.into(),
                    })),
                }),
            })),
        })),
    }
}
