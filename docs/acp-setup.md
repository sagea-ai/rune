# ACP Setup

Rune can be used in text editors and IDEs that support [Agent Client Protocol](https://agentclientprotocol.com/overview/clients). Rune includes the `rune-acp` tool.
Once you have set up `rune` with the API keys, you are ready to use `rune-acp` in your editor. Below are the setup instructions for some editors that support ACP.

## Zed

For usage in Zed, we recommend using the [Rune Zed's extension](https://zed.dev/extensions/rune-cli). Alternatively, you can set up a local install as follows:

1. Go to `~/.config/zed/settings.json` and, under the `agent_servers` JSON object, add the following key-value pair to invoke the `rune-acp` command. Here is the snippet:

```json
{
   "agent_servers": {
      "Rune": {
         "type": "custom",
         "command": "rune-acp",
         "args": [],
         "env": {}
      }
   }
}
```

2. In the `New Thread` pane on the right, select the `rune` agent and start the conversation.

## JetBrains IDEs

1. Add the following snippet to your JetBrains IDE acp.json ([documentation](https://www.jetbrains.com/help/ai-assistant/acp.html)):

```json
{
  "agent_servers": {
    "Rune": {
      "command": "rune-acp",
    }
  }
}
```

2. In the AI Chat agent selector, select the new Rune agent and start the conversation.

## Neovim (using avante.nvim)

Add Rune in the acp_providers section of your configuration

```lua
{
  acp_providers = {
    ["rune-cli"] = {
      command = "rune-acp",
      env = {
         RUNE_API_KEY = os.getenv("RUNE_API_KEY"), -- necessary if you setup Rune manually
      },
    }
  }
}
```
