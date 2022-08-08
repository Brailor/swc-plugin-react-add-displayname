use swc_ecma_parser::{EsConfig, Syntax};
use swc_ecma_transforms_testing::Tester;
use swc_plugin::syntax_pos::DUMMY_SP;
use swc_plugin::{metadata::TransformPluginProgramMetadata, plugin_transform};

use swc_plugin::ast::*;
// use swc_plugin::utils::swc_common::Spanned;

// impl VisitMut for TransformVisitor {
// Implement necessary visit_mut_* methods for actual custom transform.
// A comprehensive list of possible visitor methods can be found here:
// https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html
// }
pub struct TransformVisitor;

impl TransformVisitor {
    pub fn new() -> Self {
        Self
    }
}
impl VisitMut for TransformVisitor {
    fn visit_mut_class_decl(&mut self, decl: &mut ClassDecl) {
        let ident = &decl.ident;
        let class = &mut decl.class;
        let body = &class.body;
        let jsword = JsWord::from("displayName");

        println!("The name of the class is: {}", ident.sym);
        if let None = body.iter().find(|member| {
            if let Some(prop) = member.as_class_prop() {
                let prop_ident = prop.key.as_ident().expect("expected to have ident");

                return prop_ident.sym == jsword;
            }
            false
        }) {
            println!("The displayName prop is not on the class");
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
                    //TODO: add correct value
                    value: None,
                }),
            )
        }
    }
}
#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    let folder = program.fold_with(&mut as_folder(TransformVisitor::new()));

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
        let expected = tester.apply_transform(
            as_folder(TransformVisitor::new()),
            "output.js",
            syntax,
            source,
        )?;

        println!("output: {:?}", expected);

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
        basic,
        "class Alert extends React.Component {
            static componentId = 'Alert';

            render() {
                return(<div>
                        Hello from Alert!
                    </div>)
            }
        }",
        "class Alert extends React.Component {
            static displayName
            static componentId = 'Alert';

            render() {
                return <div>
                        Hello from Alert!
                    </div>
            }
        }"
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
}
