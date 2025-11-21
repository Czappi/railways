use std::error::Error;

use opentelemetry::{
    global::BoxedTracer,
    trace::{SpanContext, TraceContextExt, Tracer},
    Context, KeyValue,
};

pub struct TaskScope<'a> {
    context: Context,
    tracer: &'a BoxedTracer,
}

impl<'a> Drop for TaskScope<'a> {
    fn drop(&mut self) {
        self.context.span().end();
    }
}

impl<'a> TaskScope<'a> {
    pub fn new(context: Context, tracer: &'a BoxedTracer) -> Self {
        Self { context, tracer }
    }

    pub fn add_link(&self, span_context: SpanContext, attributes: Vec<KeyValue>) {
        self.context.span().add_link(span_context, attributes);
    }

    pub fn record_error(&self, err: &dyn Error) {
        self.context.span().record_error(err);
    }

    pub fn add_event<T>(&self, name: T, attributes: Vec<KeyValue>)
    where
        T: Into<std::borrow::Cow<'static, str>>,
    {
        self.context.span().add_event(name, attributes);
    }

    pub fn span_context(&self) -> SpanContext {
        self.context.span().span_context().clone()
    }

    pub fn child<'b, T>(&self, name: T) -> TaskScope<'b>
    where
        T: Into<std::borrow::Cow<'static, str>>,
        'a: 'b,
    {
        let span = self.tracer.start_with_context(name, &self.context);
        TaskScope::new(Context::current_with_span(span), self.tracer)
    }

    pub fn set_attributes(&self, attributes: impl Into<Vec<KeyValue>>) {
        self.context.span().set_attributes(attributes.into());
    }
}
