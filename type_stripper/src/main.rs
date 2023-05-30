use full_moon::{self, visitors::VisitorMut, ast::{self, punctuated::{Punctuated, Pair}, Expression, FunctionBody, types::TypeSpecifier, Parameter}, ast::{span::ContainedSpan, types::TypeInfo, Stmt}, tokenizer::{Symbol, Token, TokenType, TokenReference}, print};
use std::{fs::File, io::{Read, Write, self}, error::Error, thread::Builder, env};
use anyhow::{anyhow, Result};

struct TypeRemover;

impl TypeRemover {
    fn remove_local_fn_types(&self, node: ast::LocalFunction) -> ast::LocalFunction {
        let function_body = node.body();

        // Restore tokens after arg types
        let mut new_parameters = Punctuated::new();

        function_body.parameters().pairs().enumerate().for_each(|(index, parameter_pair)| {
            let parameter = parameter_pair.value().to_owned();
            let punctuation = parameter_pair.punctuation().map(|punctuation| punctuation.to_owned());

            let token_reference = match parameter {
                Parameter::Name(ref token_reference) => token_reference,

                Parameter::Ellipse(ref token_reference) => token_reference,

                _ => {
                    // error?
                    return new_parameters.push(Pair::new(parameter, punctuation))
                }
            };

            let Some(type_specifier) = function_body.type_specifiers().nth(index) else {
                return new_parameters.push(Pair::new(parameter, punctuation))
            };

            let Some(type_specifier) = type_specifier else {
                return new_parameters.push(Pair::new(parameter, punctuation))
            };

            if let TypeInfo::Basic(type_info) = type_specifier.type_info() {
                let mut leading_trivia = vec![];
                // Push old leading trivia first
                token_reference.leading_trivia().for_each(|token| leading_trivia.push(token.to_owned()));
                // Add new leading trivia
                type_info.leading_trivia().for_each(|token| leading_trivia.push(token.to_owned()));

        
                let mut trailing_trivia = vec![];
                // Push old trailing trivia first
                token_reference.trailing_trivia().for_each(|token| trailing_trivia.push(token.to_owned()));
                // Add new trailing trivia
                type_info.trailing_trivia().for_each(|token| trailing_trivia.push(token.to_owned()));

                let new_token_reference = TokenReference::new(leading_trivia, token_reference.token().to_owned(), trailing_trivia);
                
                // Push new parameter
                match parameter {
                    Parameter::Name(_) => {
                        new_parameters.push(Pair::new(Parameter::Name(new_token_reference), punctuation))
                    }

                    Parameter::Ellipse(_) => {
                        new_parameters.push(Pair::new(Parameter::Ellipse(new_token_reference), punctuation))
                    }

                    _ => {
                        // error?
                        return new_parameters.push(Pair::new(parameter, punctuation))
                    }
                };
            } else {
                new_parameters.push(Pair::new(parameter, punctuation))
            }
        });

        let mut new_function_body = function_body.to_owned().with_type_specifiers(vec![]).with_parameters(new_parameters).with_return_type(None);

        // Restore tokens after return
        if let Some(type_specifier_opt) = function_body.return_type() {
            if let TypeInfo::Basic(type_info) = type_specifier_opt.type_info() {
                let first_token_reference = new_function_body.parameters_parentheses().tokens().0.to_owned();
                let second_token_reference = new_function_body.parameters_parentheses().tokens().1.to_owned();
                
                let mut leading_trivia = vec![];
                // Push leading trivia from parameters parentheses
                second_token_reference.leading_trivia().for_each(|token| leading_trivia.push(token.to_owned()));
                // Add leading trivia from type info
                type_info.leading_trivia().for_each(|token| leading_trivia.push(token.to_owned()));

                let mut trailing_trivia = vec![];
                // Push leading trivia from parameters parentheses
                second_token_reference.trailing_trivia().for_each(|token| trailing_trivia.push(token.to_owned()));
                // Add leading trivia from type info
                type_info.trailing_trivia().for_each(|token| trailing_trivia.push(token.to_owned()));
                
                let new_second_token_reference = TokenReference::new(leading_trivia, second_token_reference.token().to_owned(), trailing_trivia);
                let new_parameters_parentheses = ContainedSpan::new(first_token_reference, new_second_token_reference);
                
                new_function_body = new_function_body.with_parameters_parentheses(new_parameters_parentheses);
            }
        }

        node.with_body(new_function_body)
    }

    fn remove_global_fn_types(&self, node: ast::FunctionDeclaration) -> ast::FunctionDeclaration {
        let function_body = node.body();

        // Restore tokens after arg types
        let mut new_parameters = Punctuated::new();

        function_body.parameters().pairs().enumerate().for_each(|(index, parameter_pair)| {
            let parameter = parameter_pair.value().to_owned();
            let punctuation = parameter_pair.punctuation().map(|punctuation| punctuation.to_owned());

            let token_reference = match parameter {
                Parameter::Name(ref token_reference) => token_reference,

                Parameter::Ellipse(ref token_reference) => token_reference,

                _ => {
                    // error?
                    return new_parameters.push(Pair::new(parameter, punctuation))
                }
            };

            let Some(type_specifier) = function_body.type_specifiers().nth(index) else {
                return new_parameters.push(Pair::new(parameter, punctuation))
            };

            let Some(type_specifier) = type_specifier else {
                return new_parameters.push(Pair::new(parameter, punctuation))
            };

            if let TypeInfo::Basic(type_info) = type_specifier.type_info() {
                let mut leading_trivia = vec![];
                // Push old leading trivia first
                token_reference.leading_trivia().for_each(|token| leading_trivia.push(token.to_owned()));
                // Add new leading trivia
                type_info.leading_trivia().for_each(|token| leading_trivia.push(token.to_owned()));

        
                let mut trailing_trivia = vec![];
                // Push old trailing trivia first
                token_reference.trailing_trivia().for_each(|token| trailing_trivia.push(token.to_owned()));
                // Add new trailing trivia
                type_info.trailing_trivia().for_each(|token| trailing_trivia.push(token.to_owned()));

                let new_token_reference = TokenReference::new(leading_trivia, token_reference.token().to_owned(), trailing_trivia);
                
                // Push new parameter
                match parameter {
                    Parameter::Name(_) => {
                        new_parameters.push(Pair::new(Parameter::Name(new_token_reference), punctuation))
                    }

                    Parameter::Ellipse(_) => {
                        new_parameters.push(Pair::new(Parameter::Ellipse(new_token_reference), punctuation))
                    }

                    _ => {
                        // error?
                        return new_parameters.push(Pair::new(parameter, punctuation))
                    }
                };
            } else {
                new_parameters.push(Pair::new(parameter, punctuation))
            }
        });

        let mut new_function_body = function_body.to_owned().with_type_specifiers(vec![]).with_parameters(new_parameters).with_return_type(None);

        // Restore tokens after return
        if let Some(type_specifier_opt) = function_body.return_type() {
            if let TypeInfo::Basic(type_info) = type_specifier_opt.type_info() {
                let first_token_reference = new_function_body.parameters_parentheses().tokens().0.to_owned();
                let second_token_reference = new_function_body.parameters_parentheses().tokens().1.to_owned();
                
                let mut leading_trivia = vec![];
                // Push leading trivia from parameters parentheses
                second_token_reference.leading_trivia().for_each(|token| leading_trivia.push(token.to_owned()));
                // Add leading trivia from type info
                type_info.leading_trivia().for_each(|token| leading_trivia.push(token.to_owned()));

                let mut trailing_trivia = vec![];
                // Push leading trivia from parameters parentheses
                second_token_reference.trailing_trivia().for_each(|token| trailing_trivia.push(token.to_owned()));
                // Add leading trivia from type info
                type_info.trailing_trivia().for_each(|token| trailing_trivia.push(token.to_owned()));
                
                let new_second_token_reference = TokenReference::new(leading_trivia, second_token_reference.token().to_owned(), trailing_trivia);
                let new_parameters_parentheses = ContainedSpan::new(first_token_reference, new_second_token_reference);
                
                new_function_body = new_function_body.with_parameters_parentheses(new_parameters_parentheses);
            }
        }

        node.with_body(new_function_body)
    }

    // TODO: Restore tokens which are lost after filtering out type decls
    fn remove_type_declarations(&self, node: ast::Block) -> ast::Block {
        let stmts = node.stmts_with_semicolon();

        let new_stmts: Vec<(Stmt, Option<TokenReference>)> = stmts.filter(|(stmt, _)| {
            if let ast::Stmt::TypeDeclaration(_) = stmt {
                false
            } else {
                true
            }
        }).map(|(stmt, token_reference)| (stmt.to_owned(), token_reference.to_owned())).collect();

        node.with_stmts(new_stmts)
    }

    fn remove_expression_types(&self, node: ast::Expression) -> ast::Expression {
        match node {
            ast::Expression::Value { value, type_assertion } => {
                let Some(type_assertion) = type_assertion else {
                    return ast::Expression::Value {
                        value,
                        type_assertion: None
                    }
                };

                let TypeInfo::Basic(token_reference) = type_assertion.cast_to() else {
                    return ast::Expression::Value {
                        value,
                        type_assertion: None
                    }
                };

                // Restore tokens
                let leading_trivia: Vec<Token> = token_reference.leading_trivia().map(|token| token.to_owned()).collect();
                let token = token_reference.token().to_owned();
                let trailing_trivia: Vec<Token> = token_reference.trailing_trivia().map(|token| token.to_owned()).collect();
                let new_token_reference = TokenReference::new(leading_trivia, token, trailing_trivia);

                match *value {
                    ast::Value::Function((_, function_body)) => {
                        return ast::Expression::Value {
                            value: Box::new(ast::Value::Function((new_token_reference, function_body.to_owned()))),
                            type_assertion: None
                        }
                    },

                    ast::Value::Number(_) => {
                        return ast::Expression::Value {
                            value: Box::new(ast::Value::Number(new_token_reference)),
                            type_assertion: None
                        }
                    },

                    ast::Value::String(_) => {
                        return ast::Expression::Value {
                            value: Box::new(ast::Value::String(new_token_reference)),
                            type_assertion: None
                        }
                    },

                    ast::Value::Symbol(_) => {
                        return ast::Expression::Value {
                            value: Box::new(ast::Value::Symbol(new_token_reference)),
                            type_assertion: None
                        }
                    },

                    ast::Value::ParenthesesExpression(expression) => {
                        return self.remove_expression_types(expression)
                    },

                    _ => {
                        return ast::Expression::Value {
                            value,
                            type_assertion: None
                        }
                    }
                }
            },

            ast::Expression::Parentheses { contained, expression } => {
                return ast::Expression::Parentheses {
                    contained,
                    expression: Box::new(self.remove_expression_types(*expression))
                }
            }

            _ => node
        }
    }

    // TODO: restore type assertion tokens
    fn remove_local_assignment_types(&self, node: ast::LocalAssignment) -> ast::LocalAssignment {
        let mut new_names = Punctuated::new();

        node.names().pairs().enumerate().for_each(|(index, token_reference_pair)| {
            let token_reference = token_reference_pair.value().to_owned();
            let punctuation = token_reference_pair.punctuation().map(|token_reference| token_reference.to_owned());

            let Some(type_specifier) = node.type_specifiers().nth(index) else {
                return new_names.push(Pair::new(token_reference, punctuation))
            };

            let Some(type_specifier) = type_specifier else {
                return new_names.push(Pair::new(token_reference, punctuation))
            };

            if let TypeInfo::Basic(type_info) = type_specifier.type_info() {
                let mut leading_trivia = vec![];
                // Push old leading trivia first
                token_reference.leading_trivia().for_each(|token| leading_trivia.push(token.to_owned()));
                // Add new leading trivia
                type_info.leading_trivia().for_each(|token| leading_trivia.push(token.to_owned()));

                
                let mut trailing_trivia = vec![];
                // Push old trailing trivia first
                token_reference.trailing_trivia().for_each(|token| trailing_trivia.push(token.to_owned()));
                // Add new trailing trivia
                type_info.trailing_trivia().for_each(|token| trailing_trivia.push(token.to_owned()));
                
                
                let new_token_reference = TokenReference::new(leading_trivia, token_reference.token().to_owned(), trailing_trivia);
        
                new_names.push(Pair::new(new_token_reference, punctuation))
            } else {
                new_names.push(Pair::new(token_reference, punctuation))
            }
        });

        node.with_type_specifiers(vec![]).with_names(new_names)
    }

    fn remove_forvar_types(&self, node: ast::GenericFor) -> ast::GenericFor {
        let mut new_names = Punctuated::new();

        node.names().pairs().enumerate().for_each(|(index, token_reference_pair)| {
            let token_reference = token_reference_pair.value().to_owned();
            let punctuation = token_reference_pair.punctuation().map(|token_reference| token_reference.to_owned());

            let Some(type_specifier) = node.type_specifiers().nth(index) else {
                return new_names.push(Pair::new(token_reference, punctuation))
            };

            let Some(type_specifier) = type_specifier else {
                return new_names.push(Pair::new(token_reference, punctuation))
            };

            if let TypeInfo::Basic(type_info) = type_specifier.type_info() {
                let mut leading_trivia = vec![];
                // Push old leading trivia first
                token_reference.leading_trivia().for_each(|token| leading_trivia.push(token.to_owned()));
                // Add new leading trivia
                type_info.leading_trivia().for_each(|token| leading_trivia.push(token.to_owned()));

                let mut trailing_trivia = vec![];
                // Push old trailing trivia first
                token_reference.trailing_trivia().for_each(|token| trailing_trivia.push(token.to_owned()));
                // Add new trailing trivia
                type_info.trailing_trivia().for_each(|token| trailing_trivia.push(token.to_owned()));
                
                let new_token_reference = TokenReference::new(leading_trivia, token_reference.token().to_owned(), trailing_trivia);
        
                new_names.push(Pair::new(new_token_reference, punctuation))
            } else {
                // Duct tape fix (prevents faulty code)
                // for example:
                // for v : string | number in next, {} do ... end
                // turns to
                // for vin next, {} do ... end

                let mut leading_trivia = vec![];
                // Push old leading trivia first
                token_reference.leading_trivia().for_each(|token| leading_trivia.push(token.to_owned()));

                let mut trailing_trivia = vec![];
                // Push old trailing trivia first
                token_reference.trailing_trivia().for_each(|token| trailing_trivia.push(token.to_owned()));
                // Push space
                trailing_trivia.push(Token::new(TokenType::Whitespace { characters: " ".into() }));

                let new_token_reference = TokenReference::new(leading_trivia, token_reference.token().to_owned(), trailing_trivia);

                new_names.push(Pair::new(new_token_reference, punctuation))
            }
        });

        node.with_type_specifiers(vec![]).with_names(new_names)
    }

    fn remove_arg_types(&self, node: ast::FunctionArgs) -> ast::FunctionArgs {
        match node {
            ast::FunctionArgs::Parentheses { parentheses, arguments } => {
                let Some(argument) = arguments.first() else {
                    return ast::FunctionArgs::Parentheses {
                        parentheses,
                        arguments
                    }
                };

                let Expression::Value { value, .. } = argument.value() else {
                    return ast::FunctionArgs::Parentheses {
                        parentheses,
                        arguments
                    }
                };

                let ast::Value::Function((token_reference, function_body)) = *value.to_owned() else {
                    return ast::FunctionArgs::Parentheses {
                        parentheses,
                        arguments
                    }
                };

                // Restore tokens after arg types
                let mut new_parameters = Punctuated::new();

                function_body.parameters().pairs().enumerate().for_each(|(index, parameter_pair)| {
                    let parameter = parameter_pair.value().to_owned();
                    let punctuation = parameter_pair.punctuation().map(|punctuation| punctuation.to_owned());

                    let token_reference = match parameter {
                        Parameter::Name(ref token_reference) => token_reference,

                        Parameter::Ellipse(ref token_reference) => token_reference,

                        _ => {
                            // error?
                            return new_parameters.push(Pair::new(parameter, punctuation))
                        }
                    };

                    let Some(type_specifier) = function_body.type_specifiers().nth(index) else {
                        return new_parameters.push(Pair::new(parameter, punctuation))
                    };

                    let Some(type_specifier) = type_specifier else {
                        return new_parameters.push(Pair::new(parameter, punctuation))
                    };

                    if let TypeInfo::Basic(type_info) = type_specifier.type_info() {
                        let mut leading_trivia = vec![];
                        // Push old leading trivia first
                        token_reference.leading_trivia().for_each(|token| leading_trivia.push(token.to_owned()));
                        // Add new leading trivia
                        type_info.leading_trivia().for_each(|token| leading_trivia.push(token.to_owned()));

                
                        let mut trailing_trivia = vec![];
                        // Push old trailing trivia first
                        token_reference.trailing_trivia().for_each(|token| trailing_trivia.push(token.to_owned()));
                        // Add new trailing trivia
                        type_info.trailing_trivia().for_each(|token| trailing_trivia.push(token.to_owned()));

                        let new_token_reference = TokenReference::new(leading_trivia, token_reference.token().to_owned(), trailing_trivia);
                        
                        // Push new parameter
                        match parameter {
                            Parameter::Name(_) => {
                                new_parameters.push(Pair::new(Parameter::Name(new_token_reference), punctuation));
                            }

                            Parameter::Ellipse(_) => {
                                new_parameters.push(Pair::new(Parameter::Ellipse(new_token_reference), punctuation));
                            }

                            _ => {
                                // error?
                                return new_parameters.push(Pair::new(parameter, punctuation))
                            }
                        };
                    } else {
                        new_parameters.push(Pair::new(parameter, punctuation))
                    }
                });

                let mut new_function_body = function_body.clone().with_type_specifiers(vec![]).with_parameters(new_parameters).with_return_type(None);

                // Restore tokens after return
                if let Some(type_specifier) = function_body.return_type() {
                    if let TypeInfo::Basic(type_info) = type_specifier.type_info() {
                        let first_token_reference = function_body.parameters_parentheses().tokens().0.to_owned();
                        let second_token_reference = function_body.parameters_parentheses().tokens().1.to_owned();
                        
                        let mut leading_trivia = vec![];
                        // Push leading trivia from parameters parentheses
                        second_token_reference.leading_trivia().for_each(|token| leading_trivia.push(token.to_owned()));
                        // Add leading trivia from type info
                        type_info.leading_trivia().for_each(|token| leading_trivia.push(token.to_owned()));

                        let mut trailing_trivia = vec![];
                        // Push leading trivia from parameters parentheses
                        second_token_reference.trailing_trivia().for_each(|token| trailing_trivia.push(token.to_owned()));
                        // Add leading trivia from type info
                        type_info.trailing_trivia().for_each(|token| trailing_trivia.push(token.to_owned()));

                        let new_second_token_reference = TokenReference::new(leading_trivia, second_token_reference.token().to_owned(), trailing_trivia);
                        let new_parameters_parentheses = ContainedSpan::new(first_token_reference, new_second_token_reference);

                        new_function_body = new_function_body.with_parameters_parentheses(new_parameters_parentheses);
                    }
                }

                // Re-construct the body
                let mut punctuated: Punctuated<Expression> = Punctuated::new();
                punctuated.push(Pair::End(Expression::Value {
                    value: Box::new(ast::Value::Function((token_reference, new_function_body))),
                    type_assertion: None
                }));

                return ast::FunctionArgs::Parentheses {
                    parentheses,
                    arguments: punctuated
                }
            }

            _ => {}
        }

        node
    }
}

impl VisitorMut for TypeRemover {
    fn visit_local_function(&mut self, node: ast::LocalFunction) -> ast::LocalFunction {
        println!("Attempting to optimize LocalFunction");
        self.remove_local_fn_types(node)
    }

    fn visit_function_declaration(&mut self, node: ast::FunctionDeclaration) -> ast::FunctionDeclaration {
        println!("Attempting to optimize FunctionDeclaration");
        self.remove_global_fn_types(node)
    }

    fn visit_function_args(&mut self, node: ast::FunctionArgs) -> ast::FunctionArgs {
        println!("Attempting to optimize FunctionArgs");
        self.remove_arg_types(node)
    }

    fn visit_block(&mut self, node: ast::Block) -> ast::Block {
        println!("Attempting to optimize Block");
        self.remove_type_declarations(node)
    }

    fn visit_expression(&mut self, node: ast::Expression) -> ast::Expression {
        println!("Attempting to optimize Expression");
        self.remove_expression_types(node)
    }

    fn visit_local_assignment(&mut self, node: ast::LocalAssignment) -> ast::LocalAssignment {
        println!("Attempting to optimize LocalAssignment");
        self.remove_local_assignment_types(node)
    }

    fn visit_generic_for(&mut self, node: ast::GenericFor) -> ast::GenericFor {
        println!("Attempting to optimize GenericFor");
        self.remove_forvar_types(node)
    }
}

fn main() -> Result<()> {
    if env::args().count() - 1 < 1 {
        return Err(anyhow!("Input file expected as the first argument!"));
    }

    let builder = Builder::new().stack_size(1024 * 1024 * 1024);
    let handle = builder.spawn(|| -> Result<(), Box<dyn Error + Send + Sync>> { 
        let args: Vec<String> = env::args().collect();
        let path = args.get(1).unwrap();// "./tests/sum.lua";

        let mut input_file = File::open(path)?;
        let mut input_source = String::new();
        input_file.read_to_string(&mut input_source)?;

        let ast = full_moon::parse(&input_source)?;
        
        let mut type_remover = TypeRemover;
        let new_ast = type_remover.visit_ast(ast);

        let mut output_file = File::create(format!("{}{}", path, ".stripped.lua"))?;
        let output_source = full_moon::print(&new_ast);
        
        output_file.write_all(output_source.as_bytes())?;

        Ok(())
    }).unwrap();

    if let Err(e) = handle.join() {
        println!("Failed to join thread: {:#?}", e);
        return Err(anyhow!("Something went wrong when launching the thread"));
    }

    println!("Removed all types successfully! Press any key to exit...");

    io::stdin().read_line(&mut String::new()).unwrap();

    Ok(())
}