# Configuration

For basic configuration instructions, see [this documentation](https://developers.openai.com/codex/config-basic).

For advanced configuration instructions, see [this documentation](https://developers.openai.com/codex/config-advanced).

For a full configuration reference, see [this documentation](https://developers.openai.com/codex/config-reference).

## Fork WebSocket keepalive

Set this near the top of `~/.codex/config.toml`, before any table headers:

```toml
websocket_keepalive_timeout_ms = 60000
```

The default is one minute. This setting works with the built-in OpenAI provider.
A custom provider can override it under `[model_providers.<id>]`. Values must be
positive. Client pings still run every ten seconds, and any inbound frame,
including Pong, resets the keepalive timeout. The watchdog checks the deadline
on each ping tick, so failure can occur up to ten seconds after the threshold.
The separate application stream idle timeout still applies.

Use a fork build containing this setting and restart the process making model
requests after changing it, including the background daemon or T3 app-server.

## Lifecycle hooks

Admins can set top-level `allow_managed_hooks_only = true` in
`requirements.toml` to ignore user, project, and session hook configs while
still allowing managed hooks from requirements and managed config layers. This
setting is only supported in `requirements.toml`; putting it in `config.toml`
does not enable managed-hooks-only mode.
