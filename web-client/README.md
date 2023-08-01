# web-client

## Development

From your terminal:

```sh
npm run dev
```

This starts your app in development mode, rebuilding assets on file changes.

## Deployment

Deployed on Fly.io as a single-instance server using the Remix server. Contains no internal state, so can safely be horizontally scaled. Requires the following env vars to function:
- `BACKEND_HOST`: Hostname of the `economy-data-api` to use for getting price information
