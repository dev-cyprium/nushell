use nu_engine::get_full_help;
use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{Category, IntoPipelineData, PipelineData, ShellError, Signature, Type, Value};

#[derive(Clone)]
pub struct To;

impl Command for To {
    fn name(&self) -> &str {
        "to"
    }

    fn usage(&self) -> &str {
        "Translate structured data to a format."
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("to")
            .category(Category::Formats)
            .input_output_types(vec![(Type::Nothing, Type::String)])
    }

    fn extra_usage(&self) -> &str {
        "You must use one of the following subcommands. Using this command as-is will only produce this help message."
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        Ok(Value::String {
            val: get_full_help(
                &To.signature(),
                &To.examples(),
                engine_state,
                stack,
                self.is_parser_keyword(),
            ),
            span: call.head,
        }
        .into_pipeline_data())
    }
}
