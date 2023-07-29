pub(crate) enum Slice {
	OpenElement(String),
	CloseElement(String),
	AddAttribute(String, String),
	AddBlock(usize),
	AddText(String),
}

pub(crate) fn slices() -> Vec<Slice> {
	fn tag_in(name: &str) -> Slice { Slice::OpenElement(name.into()) }
	fn tag_out(name: &str) -> Slice { Slice::CloseElement(name.into()) }
	fn attr(name: &str, value: &str) -> Slice { Slice::AddAttribute(name.into(), value.into()) }
	fn block(index: usize) -> Slice { Slice::AddBlock(index) }
	fn text(text: &str) -> Slice { Slice::AddText(text.into()) }
	let slices = vec![
		tag_in("div"),
		attr("class", "flex flex-col items-center space-y-10 pt-10"),
		tag_in("div"),
		attr("class", "flex flex-col items-center space-y-5"),
		tag_in("h1"),
		attr("class", "text-2xl font-bold"),
		block(0),
		text(" "),
		block(1),
		tag_out("h1"),
		tag_in("button"),
		attr("class", "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"),
		attr("phx-click", "toggle"),
		block(2),
		tag_out("button"),
		tag_out("div"),
		tag_in("div"),
		attr("class", "text-center max-w-[200px]"),
		text("\n          More documentation and examples at\n          "),
		tag_in("a"),
		attr("class", "text-blue-500"),
		attr("href", "https://liveviewjs.com"),
		attr("target", "_blank"),
		attr("rel", "noopener noreferrer"),
		text("LiveViewJS.com"),
		tag_out("a"),
		tag_out("div"),
		tag_out("div"),
	];
	slices
}
