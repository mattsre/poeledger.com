import { MantineProvider, createEmotionCache } from "@mantine/core";
import {
  Links,
  LiveReload,
  Meta,
  Outlet,
  Scripts,
  ScrollRestoration,
  V2_MetaFunction,
} from "@remix-run/react";

export const meta: V2_MetaFunction = () => {
  return [
    { title: "PoE Ledger" },
    { name: "description", content: "A historical price tracker for Path of Exile" },
  ];
};

createEmotionCache({ key: "mantine" });

export default function App() {
  return (
    <MantineProvider theme={{ colorScheme: "dark" }} withGlobalStyles withNormalizeCSS>
      <html lang="en">
        <head>
          <meta charSet="utf-8" />
          <meta name="viewport" content="width=device-width,initial-scale=1" />
          <Meta />
          <Links />
        </head>
        <body>
          <Outlet />
          <ScrollRestoration />
          <Scripts />
          <LiveReload />
        </body>
      </html>
    </MantineProvider>
  );
}
