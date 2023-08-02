use std::error::Error;
use actix::Addr;
use uuid::Uuid;
use crate::live::agent;
use crate::live::agent::{LiveAgent, ToHtmlString};
use crate::LiveView;
use crate::vision::{Vision};

#[derive(Debug)]
pub struct LiveLink<T: LiveView + 'static> {
	addr: Addr<LiveAgent<T>>,
}

impl<T: LiveView + 'static> LiveLink<T> {
	pub fn start(params: T::Params) -> Self {
		let addr = agent::start(params);
		LiveLink { addr }
	}

	pub(crate) fn as_agent(&self) -> &Addr<LiveAgent<T>> { &self.addr }

	pub async fn html_string(&self) -> Result<String, Box<dyn Error>> {
		// TODO Find proper home for generating csrf.
		let csrf = Uuid::new_v4();
		let content = self.addr.send(ToHtmlString).await?;
		Ok(full_html(content, csrf.to_string()))
	}
}

fn full_html(inner_html: impl AsRef<str>, csrf: impl AsRef<str>) -> String {
	// TODO deal with session data
	// TODO deal with page title
	let vision: Vision = vision! {
		<!"DOCTYPE html">
		<html lang="en" class="h-full bg-white">
		<head>
	        <meta charset="utf-8" />
	        <meta http-equiv="X-UA-Compatible" content="IE=edge" />
	        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
	        <meta name="csrf-token" content={csrf.as_ref()} />
	        <title data-suffix=" · lvrs">"lvrstest · lvrs"</title>
	        <!"-- LiveViewJS Client Javascript - compiled from src/client/index.ts --">
	        <script defer type="text/javascript" src="/index.js"></script>
			<!"-- Tailwind CSS: we recommend replacing this with your own CSS --">
	        <script src="https://cdn.tailwindcss.com"></script>
		</head>
		<body>
			<!"-- Embedded LiveView --">
			<div
		        data-phx-main="true"
                data-phx-session="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJjb29raWUiOnsib3JpZ2luYWxNYXhBZ2UiOjYwNDgwMDAwMCwiZXhwaXJlcyI6IjIwMjMtMDgtMDNUMTc6MjI6MTguMTA4WiIsInNlY3VyZSI6ZmFsc2UsImh0dHBPbmx5Ijp0cnVlLCJwYXRoIjoiLyIsInNhbWVTaXRlIjoic3RyaWN0In0sIl9jc3JmX3Rva2VuIjoianhLY040NGJuejRUbzVFUC0yZEZOIiwiaWF0IjoxNjkwNDc4NTUwfQ.L9toxrjeVM3ONgIfaD_uFpACSbMxAtY93IhYrSvjTQE"
                data-phx-static="" id="phx-uOOXONR96Nmu181A2NfpR">
			{inner_html.as_ref()}
			</div>
		</body>
		</html>
	};
	let outer_html = vision.to_html_string();
	format!("{}", outer_html)
}

#[cfg(test)]
mod tests {
	use crate::live::link::full_html;
	use crate::server::assets;

	#[test]
	fn check_full_html() {
		let inner = "Hello";
		let csrf = "jxKcN44bnz4To5EP-2dFN";
		let expected = {
			let prefix = assets::tests::prefix().replace("\n    ", "\n");
			let postfix = assets::tests::postfix();
			format!("{}{}{}", &prefix.trim(), &inner, &postfix.trim())
				.replace("\n\n", "\n")
				.replace("\n    ", " ")
		};
		let produced = full_html(inner, csrf).replace("\n\n", "\n");
		assert_eq!(expected.trim(), produced.trim());
	}
}