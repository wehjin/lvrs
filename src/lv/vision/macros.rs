macro_rules! vision {

	(< $($tail:tt)*) => {
		{
			use $crate::lv::vision::slice::{Slice};
			use $crate::lv::vision::Vision;
			let slices = vision!(1 < $($tail)*);
			Vision::from(slices)
		}
	};
	(1) => {Vec::new()};
	(1 </ $name:ident > $($tail:tt)*) => {
		{
			let mut vec = vec![Slice::CloseElement(stringify!($name).to_string())];
			vec.extend(vision!(1 $($tail)*));
			vec
		}
	};
	(1 $literal:literal $($tail:tt)*) => {
		{
			let mut vec = vec![Slice::AddText($literal.to_string())];
			vec.extend(vision!(1 $($tail)*));
			vec
		}
	};
	(1 $block:block $($tail:tt)*) => {
		{
			let mut lv_macros_vec = vec![Slice::AddBlock($block.to_string())];
			lv_macros_vec.extend(vision!(1 $($tail)*));
			lv_macros_vec
		}
	};
	(
		1
		< $name:ident $($( $attr_name:ident )-+ = $lit:literal)*>
		$($tail:tt)*
	) => {
		{
			let mut temp_slices = Vec::new();
			temp_slices.push(Slice::OpenElement(stringify!($name).to_string()));
			$(
				temp_slices.push(
					Slice::AddAttribute(
						{
							let mut temp_names = Vec::new();
							$(
								temp_names.push(stringify!($attr_name).to_string());
							)+
							temp_names.join("-")
						},
						$lit.to_string()
					)
				);
			)*
			temp_slices.extend(vision!(1 $($tail)*));
			temp_slices
		}
	};
	(1) => {};
}

#[cfg(test)]
pub(crate) mod tests {
	use crate::lv::vision::slice::{Slice};
	use crate::lv::vision::Vision;

	#[test]
	fn success() {
		let use_emoji = true;
		let fills = emoji_fills();
		let hello1 = &fills[0];
		let name = &fills[1];
		let vision = vision! {
			<div class="flex flex-col items-center space-y-10 pt-10" >
			<div class="flex flex-col items-center space-y-5" >
			<h1 class="text-2xl font-bold" >{hello1} {name}</h1>
			<button class = "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" phx-click="toggle" >
			{if use_emoji { "Use Text"} else {"Use Emoji"}}
			</button>
			</div>
			<div class = "text-center max-w-[200px]" >
			"\n          More documentation and examples at\n          "
			<a class = "text-blue-500" href = "https://liveviewjs.com" target ="_blank" rel = "noopener noreferrer" >
			"LiveViewJS.com"
			</a >
			</div >
			</div >
		};
		let expected_vision = Vision::from(build_slices(fills));
		assert_eq!(&expected_vision, &vision);
	}

	fn emoji_fills() -> Vec<String> {
		vec![
			"👋".to_string(),
			"Liveviewjstest".to_string(),
			"Use Text".to_string(),
		]
	}

	fn build_slices(fills: Vec<String>) -> Vec<Slice> {
		let mut builder = SliceListBuilder::new();
		builder.add_open_with_attrs("div", vec![("class", "flex flex-col items-center space-y-10 pt-10")]);
		builder.add_open_with_attrs("div", vec![("class", "flex flex-col items-center space-y-5")]);
		builder.add_open_with_attrs("h1", vec![("class", "text-2xl font-bold")]);
		builder.add_block(fills[0].to_string());
		builder.add_text(" ");
		builder.add_block(fills[1].to_string());
		builder.add_close("h1");
		builder.add_open_with_attrs("button", vec![
			("class", "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"),
			("phx-click", "toggle"),
		]);
		builder.add_block(fills[2].to_string());
		builder.add_close("button");
		builder.add_close("div");
		builder.add_open_with_attrs("div", vec![("class", "text-center max-w-[200px]")]);
		builder.add_text("\n          More documentation and examples at\n          ");
		builder.add_open_with_attrs("a", vec![
			("class", "text-blue-500"),
			("href", "https://liveviewjs.com"),
			("target", "_blank"),
			("rel", "noopener noreferrer"),
		]);
		builder.add_text("LiveViewJS.com");
		builder.add_close("a");
		builder.add_close("div");
		builder.add_close("div");
		builder.build()
	}

	struct SliceListBuilder {
		slices: Vec<Slice>,
	}

	impl SliceListBuilder {
		pub fn add_open(&mut self, name: &str) -> &mut Self {
			self.slices.push(Slice::OpenElement(name.to_string()));
			self
		}
		pub fn add_attr(&mut self, name: &str, value: &str) -> &mut Self {
			self.slices.push(Slice::AddAttribute(name.to_string(), value.to_string()));
			self
		}
		pub fn add_open_with_attrs(&mut self, name: &str, attrs: Vec<(&str, &str)>) -> &mut Self {
			self.add_open(name);
			for (name, value) in attrs {
				self.add_attr(name, value);
			}
			self
		}
		pub fn add_block(&mut self, block_text: String) -> &mut Self {
			self.slices.push(Slice::AddBlock(block_text));
			self
		}
		pub fn add_text(&mut self, text: &str) -> &mut Self {
			self.slices.push(Slice::AddText(text.to_string()));
			self
		}
		pub fn add_close(&mut self, name: &str) -> &mut Self {
			self.slices.push(Slice::CloseElement(name.to_string()));
			self
		}
		pub fn build(self) -> Vec<Slice> { self.slices }
		pub fn new() -> Self {
			SliceListBuilder { slices: Vec::new() }
		}
	}
}