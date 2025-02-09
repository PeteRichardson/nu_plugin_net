use nu_plugin::{serve_plugin, EvaluatedCall, JsonSerializer};
use nu_plugin::{EngineInterface, Plugin, PluginCommand, SimplePluginCommand};
use nu_protocol::{Category, LabeledError, Signature, Span, Type, Value};

struct NetPlugin;

impl Plugin for NetPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(Net)]
    }
}

struct Net;

impl SimplePluginCommand for Net {
    type Plugin = NetPlugin;

    fn name(&self) -> &str {
        "net"
    }

    fn description(&self) -> &str {
        "network info on a Mac"
    }

    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self))
            .switch(
                "all",
                "show all hw ports, not just ones with ip addresses",
                Some('a'),
            )
            .category(Category::Network)
            .input_output_type(Type::Nothing, Type::String)
    }

    fn run(
        &self,
        _plugin: &NetPlugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        let all = call.has_flag("all")?;
        eprintln!("all: {}", all);

        if all {
            Ok(Value::String {
                val: "All".to_string(),
                internal_span: Span::unknown(),
            })
        } else {
            Ok(Value::String {
                val: "Pete".to_string(),
                internal_span: Span::unknown(),
            })
        }
    }
}

fn main() {
    serve_plugin(&NetPlugin, JsonSerializer) // change to MsgPackSerializer later
}
