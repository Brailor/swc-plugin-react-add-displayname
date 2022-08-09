use swc_ecma_parser::{EsConfig, Syntax};
use swc_ecma_transforms_testing::Tester;
use swc_plugin::syntax_pos::DUMMY_SP;
use swc_plugin::{metadata::TransformPluginProgramMetadata, plugin_transform};

use swc_plugin::ast::*;
pub struct TransformVisitor;

impl TransformVisitor {
    pub fn new() -> Self {
        Self
    }
}

impl TransformVisitor {
    fn add_displayname(&mut self, class: &mut Class, ident: &Ident) {
        let body = &class.body;
        let jsword = JsWord::from("displayName");
        // println!("Ident: {}", decl.ident.expect("to have ident"));

        if let None = body.iter().find(|member| {
            if let Some(prop) = member.as_class_prop() {
                let prop_ident = prop.key.as_ident().expect("expected to have ident");

                return prop_ident.sym == jsword;
            }
            false
        }) {
            println!("adding displayName");

            class.body.insert(
                0,
                ClassMember::ClassProp(ClassProp {
                    span: DUMMY_SP,
                    key: PropName::Ident(Ident {
                        span: DUMMY_SP,
                        sym: jsword,
                        optional: false,
                    }),
                    accessibility: None,
                    declare: false,
                    decorators: vec![],
                    definite: false,
                    is_static: true,
                    is_abstract: false,
                    is_optional: false,
                    is_override: false,
                    readonly: false,
                    type_ann: None,
                    value: Some(Box::new(Expr::Lit(Lit::Str(Str {
                        span: DUMMY_SP,
                        value: ident.sym.clone(),
                        raw: None,
                    })))),
                }),
            )
        }
    }
}
impl VisitMut for TransformVisitor {
    fn visit_mut_decl(&mut self, decl: &mut Decl) {
        if let Decl::Var(decl) = decl {
            println!("decl is varible ");
            // let decls = &mut decl.decls;
            for dec in &mut decl.decls {
                if let Some(c) = &mut dec.init {
                    if let Some(c) = c.as_mut_class() {
                        self.add_displayname(&mut c.class, &c.ident.as_ref().expect("sas"));
                    }
                }
            }
        } else if let Decl::Class(decl) = decl {
            // println!("decl is class: {:?}", decl);
            self.add_displayname(&mut decl.class, &decl.ident);
        }
    }
    // fn visit_mut_class_decl(&mut self, decl: &mut ClassDecl) {
    // }
}
#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    println!("--------------------------------");
    // println!("Program: {:?}", program);

    let folder = program.fold_with(&mut as_folder(TransformVisitor::new()));
    println!("--------------------------------");
    // println!("Output: {:?}", folder);

    folder
}

pub fn run() {
    let source = "class Alert extends React.Component {
        static componentId = 'Alert';

        render() {
            return(
                <div>
                    Hello from Alert!
                </div>
                )
        }
    }";
    let syntax = Syntax::Es(EsConfig {
        jsx: true,
        ..Default::default()
    });
    Tester::run(|tester| {
        let output = tester.apply_transform(
            as_folder(TransformVisitor::new()),
            "output.js",
            syntax,
            source,
        )?;

        println!("output: {:?}", output);

        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use swc_ecma_parser::*;
    use swc_ecma_transforms_base::resolver;
    use swc_ecma_transforms_testing::{test, test_fixture};
    use swc_plugin::{syntax_pos::Mark, *};

    use super::*;

    fn transform() -> impl Fold {
        chain!(
            resolver(Mark::new(), Mark::new(), false),
            as_folder(TransformVisitor::new())
        )
    }
    test!(
        Syntax::Es(EsConfig {
            jsx: true,

            ..Default::default()
        }),
        |_| transform(),
        adds_displayname_to_class,
        "class Alert extends React.Component {
            static componentId = 'Alert';

            render() {
                return(<div>
                        Hello from Alert!
                    </div>)
            }
        }",
        r#"class Alert extends React.Component {
            static displayName = "Alert";
            static componentId = 'Alert';

            render() {
                return <div>
                        Hello from Alert!
                    </div>
            }
        }"#
    );

    use std::path::PathBuf;

    #[testing_macros::fixture("src/tests/input.tsx")]
    fn fixture(input: PathBuf) {
        let output = input.with_file_name("output.js");
        test_fixture(
            Syntax::Typescript(TsConfig {
                tsx: true,
                ..Default::default()
            }),
            &|_| transform(),
            &input,
            &output,
        );
    }

    #[testing_macros::fixture("src/tests/input_decorators.tsx")]
    fn fixture_decorators(input: PathBuf) {
        let output = input.with_file_name("output_decorators.js");
        test_fixture(
            Syntax::Typescript(TsConfig {
                tsx: true,
                decorators: true,
                ..Default::default()
            }),
            &|_| transform(),
            &input,
            &output,
        );
    }
}
