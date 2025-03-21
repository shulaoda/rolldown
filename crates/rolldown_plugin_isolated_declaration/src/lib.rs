use std::{borrow::Cow, path::Path};

use oxc::{
  allocator::IntoIn,
  ast_visit::VisitMut,
  codegen::{CodeGenerator, CodegenOptions},
  isolated_declarations::{IsolatedDeclarations, IsolatedDeclarationsOptions},
};
use rolldown_common::ModuleType;
use rolldown_plugin::{Plugin, PluginHookMeta, PluginOrder};
use sugar_path::SugarPath;
use type_import_visitor::TypeImportVisitor;

mod type_import_visitor;

#[derive(Debug)]
pub struct IsolatedDeclarationPlugin {}

impl Plugin for IsolatedDeclarationPlugin {
  fn name(&self) -> Cow<'static, str> {
    Cow::Borrowed("builtin:isolated-declaration")
  }

  async fn transform_ast(
    &self,
    ctx: &rolldown_plugin::PluginContext,
    mut args: rolldown_plugin::HookTransformAstArgs<'_>,
  ) -> rolldown_plugin::HookTransformAstReturn {
    if matches!(args.module_type, ModuleType::Ts | ModuleType::Tsx) {
      let type_import_specifiers = args.ast.program.with_mut(|fields| {
        let mut visitor = TypeImportVisitor { imported: vec![].into_in(fields.allocator) };
        visitor.visit_program(fields.program);
        visitor.imported
      });

      let importer_path = args.cwd.join(args.id);
      let importer = importer_path.to_string_lossy();
      for specifier in type_import_specifiers {
        // TODO load typing importers module, because it will not load after strip typing.
        let _ = ctx.resolve(&specifier, Some(&importer), None).await??;
        // ctx.load(&resolved_id.id, None, load_callback_fn)
      }

      let ret = args.ast.program.with_mut(|fields| {
        IsolatedDeclarations::new(fields.allocator, IsolatedDeclarationsOptions::default())
          .build(fields.program)
      });

      // TODO BuildDiagnostic error
      if !ret.errors.is_empty() {
        return Err(anyhow::anyhow!("IsolatedDeclarations error"));
      }

      let codegen_ret =
        CodeGenerator::new().with_options(CodegenOptions::default()).build(&ret.program);

      let mut emit_dts_path = Path::new(args.id).to_path_buf();
      emit_dts_path.set_extension("d.ts");
      ctx.emit_file(
        rolldown_common::EmittedAsset {
          name: None,
          original_file_name: None,
          // TODO make sure to the .d.ts file relative to the output entry file
          file_name: Some(emit_dts_path.to_slash_lossy().into()),
          source: codegen_ret.code.into(),
        },
        None,
        None,
      );
    }
    Ok(args.ast)
  }

  // The rolldown strip types at the end of the build process, make sure to run this plugin before that.
  fn transform_ast_meta(&self) -> Option<PluginHookMeta> {
    Some(PluginHookMeta { order: Some(PluginOrder::Post) })
  }
}
