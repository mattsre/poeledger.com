import { withEmotionCache } from "@emotion/react";
import { MantineProvider } from "@mantine/core";
import {
	Links,
	LiveReload,
	Meta,
	Outlet,
	Scripts,
	ScrollRestoration,
	V2_MetaFunction,
} from "@remix-run/react";
import { useContext, useEffect } from "react";
import { ServerStyleContext, ClientStyleContext } from "./context";

export const meta: V2_MetaFunction = () => {
	return [
		{ title: "PoE Ledger" },
		{
			name: "description",
			content: "A historical price tracker for Path of Exile",
		},
	];
};

interface DocumentProps {
	children: React.ReactNode;
}

const Document = withEmotionCache(
	({ children }: DocumentProps, emotionCache) => {
		const serverStyleData = useContext(ServerStyleContext);
		const clientStyleData = useContext(ClientStyleContext);

		// Only executed on client
		useEffect(() => {
			// re-link sheet container
			emotionCache.sheet.container = document.head;
			// re-inject tags
			const tags = emotionCache.sheet.tags;
			emotionCache.sheet.flush();
			tags.forEach((tag) => {
				(emotionCache.sheet as any)._insertTag(tag);
			});
			// reset cache to reapply global styles
			clientStyleData?.reset();
		}, []);

		return (
			<html lang="en">
				<head>
					<Meta />
					<Links />
					{serverStyleData?.map(({ key, ids, css }) => (
						<style
							key={key}
							data-emotion={`${key} ${ids.join(" ")}`}
							dangerouslySetInnerHTML={{ __html: css }}
						/>
					))}
				</head>
				<body>
					{children}
					<ScrollRestoration />
					<Scripts />
					<LiveReload />
				</body>
			</html>
		);
	},
);

export default function App() {
	return (
		<Document>
			<MantineProvider
				theme={{ colorScheme: "dark" }}
				withGlobalStyles
				withNormalizeCSS
			>
				<Outlet />
			</MantineProvider>
		</Document>
	);
}
