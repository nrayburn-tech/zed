---
title: LLM Providers - Use Your Own API Keys in Zed
description: Bring your own API keys to Zed. Set up Anthropic, OpenAI, Google AI, Ollama, Mistral, OpenRouter, Vercel AI Gateway, and more.
---

# LLM Providers

To use AI in Zed, you need to have at least one large language model provider set up. Once configured, providers are
available in the [Agent Panel](./agent-panel.md) and [Inline Assistant](./inline-assistant.md).

You can do that by either subscribing to [one of Zed's plans](./plans-and-usage.md), or by using API keys you already
have for the supported providers. For general AI setup, see [Configuration](./configuration.md).

## Use Your Own Keys {#use-your-own-keys}

If you already have an API key for a provider like Anthropic or OpenAI, you can add it to Zed. No Zed subscription
required.

To add an existing API key to a given provider, go to the Agent Panel settings ({#action agent::OpenSettings}), look for
the desired provider, paste the key into the input, and hit enter.

> Note: API keys are _not_ stored as plain text in your settings file, but rather in your OS's secure credential
> storage.

## Supported Providers

Zed supports these providers with your own API keys:

- [Anthropic](#anthropic)
- [ChatGPT Subscription](#chatgpt-subscription)
- [GitHub Copilot Chat](#github-copilot-chat)
- [Google AI](#google-ai)
- [LM Studio](#lmstudio)
- [Mistral](#mistral)
- [Ollama](#ollama)
- [OpenAI](#openai)
- [OpenAI API Compatible](#openai-api-compatible)
- [OpenCode](#opencode)
- [OpenRouter](#openrouter)
- [Vercel AI Gateway](#vercel-ai-gateway)

### Anthropic {#anthropic}

You can use Anthropic models by choosing them via the model dropdown in the Agent Panel.

1. Sign up for Anthropic and [create an API key](https://console.anthropic.com/settings/keys)
2. Make sure that your Anthropic account has credits
3. Open the settings view ({#action agent::OpenSettings}) and go to the Anthropic section
4. Enter your Anthropic API key

Even if you pay for Claude Pro, you will still have
to [pay for additional credits](https://console.anthropic.com/settings/plans) to use it via the API.

Zed will also use the `ANTHROPIC_API_KEY` environment variable if it's defined.

#### Custom Models {#anthropic-custom-models}

You can add custom models to the Anthropic provider by adding the following to your Zed settings file
([how to edit](../configuring-zed.md#settings-files)):

```json [settings]
{
  "language_models": {
    "anthropic": {
      "available_models": [
        {
          "name": "claude-3-5-sonnet-20240620",
          "display_name": "Sonnet 2024-June",
          "max_tokens": 128000,
          "max_output_tokens": 2560,
          "tool_override": "some-model-that-supports-toolcalling"
        }
      ]
    }
  }
}
```

Custom models will be listed in the model dropdown in the Agent Panel.

You can configure a model to
use [extended thinking](https://docs.anthropic.com/en/docs/about-claude/models/extended-thinking-models) (if it supports
it) by changing the mode in your model's configuration to `thinking`, for example:

```json
{
  "name": "claude-sonnet-4-latest",
  "display_name": "claude-sonnet-4-thinking",
  "max_tokens": 200000,
  "mode": {
    "type": "thinking",
    "budget_tokens": 4096
  }
}
```

### ChatGPT Subscription {#chatgpt-subscription}

Use your existing ChatGPT Plus or Pro subscription to access OpenAI models directly in Zed — no separate API key
required.

1. Open the settings view ({#action agent::OpenSettings}) and go to the ChatGPT Subscription section
2. Click **Sign in** and complete the OpenAI authentication in your browser
3. Once signed in, models appear in the model dropdown, including GPT-5.5 and GPT-5.3 Codex

To sign out, click **Sign Out** in the ChatGPT Subscription settings.

> **Note:** Model availability depends on your ChatGPT subscription tier. Some models may require ChatGPT Pro.

### GitHub Copilot Chat {#github-copilot-chat}

You can use GitHub Copilot Chat with the Zed agent by choosing it via the model dropdown in the Agent Panel.

1. Open the settings view ({#action agent::OpenSettings}) and go to the GitHub Copilot Chat section
2. Click on `Sign in to use GitHub Copilot`, follow the steps shown in the modal.

Alternatively, you can provide an OAuth token via the `GH_COPILOT_TOKEN` environment variable.

> **Note**: If you don't see specific models in the dropdown, you may need to enable them in
> your [GitHub Copilot settings](https://github.com/settings/copilot/features).

To use Copilot Enterprise with Zed (for both agent and completions), you must configure your enterprise endpoint as
described in [Configuring GitHub Copilot Enterprise](./edit-prediction.md#using-github-copilot-enterprise).

### Google AI {#google-ai}

You can use Gemini models with the Zed agent by choosing it via the model dropdown in the Agent Panel.

1. Go to the Google AI Studio site and [create an API key](https://aistudio.google.com/app/apikey).
2. Open the settings view ({#action agent::OpenSettings}) and go to the Google AI section
3. Enter your Google AI API key and press enter.

The Google AI API key will be saved in your keychain.

Zed will also use the `GEMINI_API_KEY` environment variable if it's defined.
See [Using Gemini API keys](https://ai.google.dev/gemini-api/docs/api-key) in the Gemini docs for more.

#### Custom Models {#google-ai-custom-models}

By default, Zed will use `stable` versions of models, but you can use specific versions of models,
including [experimental models](https://ai.google.dev/gemini-api/docs/models/experimental-models). You can configure a
model to use [thinking mode](https://ai.google.dev/gemini-api/docs/thinking) (if it supports it) by adding a `mode`
configuration to your model. This is useful for controlling reasoning token usage and response speed. If not specified,
Gemini will automatically choose the thinking budget.

Here is an example of a custom Google AI model you could add to your Zed settings file
([how to edit](../configuring-zed.md#settings-files)):

```json [settings]
{
  "language_models": {
    "google": {
      "available_models": [
        {
          "name": "gemini-3.1-pro-preview",
          "display_name": "Gemini 3.1 Pro",
          "max_tokens": 1000000,
          "mode": {
            "type": "thinking",
            "budget_tokens": 24000
          }
        },
        {
          "name": "gemini-3-flash-preview",
          "display_name": "Gemini 3 Flash (Thinking)",
          "max_tokens": 1000000,
          "mode": {
            "type": "thinking",
            "budget_tokens": 24000
          }
        }
      ]
    }
  }
}
```

Custom models will be listed in the model dropdown in the Agent Panel.

### LM Studio {#lmstudio}

1. Download and install [the latest version of LM Studio](https://lmstudio.ai/download)
2. In the app press `cmd/ctrl-shift-m` and download at least one model (e.g., qwen2.5-coder-7b). Alternatively, you can
   get models via the LM Studio CLI:

   ```sh
   lms get qwen2.5-coder-7b
   ```

3. Make sure the LM Studio API server is running by executing:

   ```sh
   lms server start
   ```

Tip: Set [LM Studio as a login item](https://lmstudio.ai/docs/advanced/headless#run-the-llm-service-on-machine-login) to
automate running the LM Studio server.

### Mistral {#mistral}

1. Visit the Mistral platform and [create an API key](https://console.mistral.ai/api-keys/)
2. Open the configuration view ({#action agent::OpenSettings}) and navigate to the Mistral section
3. Enter your Mistral API key

The Mistral API key will be saved in your keychain.

Zed will also use the `MISTRAL_API_KEY` environment variable if it's defined.

#### Custom Models {#mistral-custom-models}

The Zed agent comes pre-configured to use the latest version for common Mistral models (Large, Medium, Small, Codestral,
Devstral, and others). All the default models support tool use. If you wish to use alternate models or customize their
parameters, you can do so by adding the following to your Zed settings file
([how to edit](../configuring-zed.md#settings-files)):

```json [settings]
{
  "language_models": {
    "mistral": {
      "api_url": "https://api.mistral.ai/v1",
      "available_models": [
        {
          "name": "mistral-tiny-latest",
          "display_name": "Mistral Tiny",
          "max_tokens": 32000,
          "max_output_tokens": 4096,
          "max_completion_tokens": 1024,
          "supports_tools": true,
          "supports_images": false
        }
      ]
    }
  }
}
```

Custom models will be listed in the model dropdown in the Agent Panel.

### Ollama {#ollama}

Download and install Ollama from [ollama.com/download](https://ollama.com/download) (Linux or macOS) and ensure it's
running with `ollama --version`.

1. Download one of the [available models](https://ollama.com/models), for example, for `mistral`:

   ```sh
   ollama pull mistral
   ```

2. Make sure that the Ollama server is running. You can start it either via running Ollama.app (macOS) or launching:

   ```sh
   ollama serve
   ```

3. In the Agent Panel, select one of the Ollama models using the model dropdown.

#### Ollama Autodiscovery

Zed will automatically discover models that Ollama has pulled. You can turn this off by setting the `auto_discover`field
in the Ollama settings. If you do this, you should manually specify which models are available.

```json [settings]
{
  "language_models": {
    "ollama": {
      "api_url": "http://localhost:11434",
      "auto_discover": false,
      "available_models": [
        {
          "name": "qwen2.5-coder",
          "display_name": "qwen 2.5 coder",
          "max_tokens": 32768,
          "supports_tools": true,
          "supports_thinking": true,
          "supports_images": true
        }
      ]
    }
  }
}
```

#### Ollama Context Length {#ollama-context}

Zed API requests to Ollama include the context length as the `num_ctx` parameter. By default, Zed uses a context length
of `4096` tokens for all Ollama models.

> **Note**: Token counts displayed in the Agent Panel are only estimates and will differ from the model's native
> tokenizer.

You can set a context length for all Ollama models using the `context_window` setting. This can also be configured in
the Ollama provider settings UI:

```json [settings]
{
  "language_models": {
    "ollama": {
      "context_window": 8192
    }
  }
}
```

Alternatively, you can configure the context length per-model using the `max_tokens` field in `available_models`:

```json [settings]
{
  "language_models": {
    "ollama": {
      "api_url": "http://localhost:11434",
      "available_models": [
        {
          "name": "qwen2.5-coder",
          "display_name": "qwen 2.5 coder 32K",
          "max_tokens": 32768,
          "supports_tools": true,
          "supports_thinking": true,
          "supports_images": true
        }
      ]
    }
  }
}
```

> **Note**: If `context_window` is set, it overrides any per-model `max_tokens` values.

If you specify a context length that is too large for your hardware, Ollama will log an error. You can watch these logs
by running: `tail -f ~/.ollama/logs/ollama.log` (macOS) or `journalctl -u ollama -f` (Linux). Depending on the memory
available on your machine, you may need to adjust the context length to a smaller value.

You may also optionally specify a value for `keep_alive` for each available model. This can be an integer (seconds) or
alternatively a string duration like "5m", "10m", "1h", "1d", etc. For example, `"keep_alive": "120s"` will allow the
remote server to unload the model (freeing up GPU VRAM) after 120 seconds.

The `supports_tools` option controls whether the model will use additional tools. If the model is tagged with `tools` in
the Ollama catalog, this option should be supplied, and the built-in profiles `Ask` and `Write` can be used. If the
model is not tagged with `tools` in the Ollama catalog, this option can still be supplied with the value `true`;
however, be aware that only the `Minimal` built-in profile will work.

The `supports_thinking` option controls whether the model will perform an explicit "thinking" (reasoning) pass before
producing its final answer. If the model is tagged with `thinking` in the Ollama catalog, set this option and you can
use it in Zed.

The `supports_images` option enables the model's vision capabilities, allowing it to process images included in the
conversation context. If the model is tagged with `vision` in the Ollama catalog, set this option and you can use it in
Zed.

#### Ollama Authentication

In addition to running Ollama on your own hardware, which generally does not require authentication, Zed also supports
connecting to remote Ollama instances. API keys are required for authentication.

One such service is [Ollama Turbo](https://ollama.com/turbo). To configure Zed to use Ollama Turbo:

1. Sign in to your Ollama account and subscribe to Ollama Turbo
2. Visit [ollama.com/settings/keys](https://ollama.com/settings/keys) and create an API key
3. Open the settings view ({#action agent::OpenSettings}) and go to the Ollama section
4. Paste your API key and press enter.
5. For the API URL enter `https://ollama.com`

Zed will also use the `OLLAMA_API_KEY` environment variables if defined.

### OpenAI {#openai}

1. Visit the OpenAI platform and [create an API key](https://platform.openai.com/account/api-keys)
2. Make sure that your OpenAI account has credits
3. Open the settings view ({#action agent::OpenSettings}) and go to the OpenAI section
4. Enter your OpenAI API key

The OpenAI API key will be saved in your keychain.

Zed will also use the `OPENAI_API_KEY` environment variable if it's defined.

#### Custom Models {#openai-custom-models}

The Zed agent comes pre-configured to use the latest version for common OpenAI models (GPT-5.2, GPT-5 mini, GPT-5.2
Codex, and others). To use alternate models, perhaps a preview release, or if you wish to control the request
parameters, you can do so by adding the following to your Zed settings file
([how to edit](../configuring-zed.md#settings-files)):

```json [settings]
{
  "language_models": {
    "openai": {
      "available_models": [
        {
          "name": "gpt-5.2",
          "display_name": "gpt-5.2 high",
          "reasoning_effort": "high",
          "max_tokens": 272000,
          "max_completion_tokens": 20000
        },
        {
          "name": "gpt-5-nano",
          "display_name": "GPT-5 Nano",
          "max_tokens": 400000
        },
        {
          "name": "gpt-5.2-codex",
          "display_name": "GPT-5.2 Codex",
          "max_tokens": 128000,
          "capabilities": {
            "chat_completions": false
          }
        }
      ]
    }
  }
}
```

You must provide the model's context window in the `max_tokens` parameter; this can be found in
the [OpenAI model documentation](https://platform.openai.com/docs/models).

For reasoning-focused models, set `max_completion_tokens` as well to avoid incurring high reasoning token costs.

If a model does not support the `/chat/completions` endpoint (for example `gpt-5.2-codex`), disable it by setting
`capabilities.chat_completions` to `false`. Zed will use the Responses endpoint instead.

Custom models will be listed in the model dropdown in the Agent Panel.

### OpenAI API Compatible {#openai-api-compatible}

Zed supports using [OpenAI compatible APIs](https://platform.openai.com/docs/api-reference/chat) by specifying a custom
`api_url` and `available_models` for the OpenAI provider. This is useful for connecting to other hosted services (like
Together AI, Anyscale, etc.) or local models.

You can add a custom, OpenAI-compatible model either via the UI or by editing your settings file.

To do it via the UI, go to the Agent Panel settings ({#action agent::OpenSettings}) and look for the "Add Provider"
button to the right of the "LLM Providers" section title. Then, fill up the input fields available in the modal.

To do it via your settings file ([how to edit](../configuring-zed.md#settings-files)), add the following snippet under
`language_models`:

```json [settings]
{
  "language_models": {
    "openai_compatible": {
      // Using Together AI as an example
      "Together AI": {
        "api_url": "https://api.together.xyz/v1",
        "available_models": [
          {
            "name": "mistralai/Mixtral-8x7B-Instruct-v0.1",
            "display_name": "Together Mixtral 8x7B",
            "max_tokens": 32768,
            "capabilities": {
              "tools": true,
              "images": false,
              "parallel_tool_calls": false,
              "prompt_cache_key": false
            }
          }
        ]
      }
    }
  }
}
```

By default, OpenAI-compatible models inherit the following capabilities:

- `tools`: true (supports tool/function calling)
- `images`: false (does not support image inputs)
- `parallel_tool_calls`: false (does not support `parallel_tool_calls` parameter)
- `prompt_cache_key`: false (does not support `prompt_cache_key` parameter)
- `chat_completions`: true (calls the `/chat/completions` endpoint)
- `interleaved_reasoning`: false (thinking tokens are sent inline in message text; set to true to send them as a
  dedicated `reasoning_content` field for models that expect it)

If a provider exposes models that only work with the Responses API, set `chat_completions` to `false` for those entries.
Zed uses the Responses endpoint for these models.

Note that LLM API keys aren't stored in your settings file. So, ensure you have it set in your environment variables
(`<PROVIDER_NAME>_API_KEY=<your api key>`) so your settings can pick it up. In the example above, it would be
`TOGETHER_AI_API_KEY=<your api key>`.

### OpenCode {#opencode}

OpenCode offers multiple ways to access AI models:

- [OpenCode Zen](https://opencode.ai/zen/): a pay-as-you-go subscription with access to a large number of tested and
  verified models
- [OpenCode Zen Free](https://opencode.ai/docs/zen/#pricing): free access to a limited set of models, with data and
  feedback collected to improve the models
- [OpenCode Go](https://opencode.ai/go): a low-cost monthly subscription with access to a validated set of open coding
  models

1. Visit [OpenCode Console](https://opencode.ai/auth) and create an account
2. Free models are available without payment. To use Zen or Go models, make sure you have enough credits or an active
   subscription
3. Generate an API key from the "API Keys" section in the OpenCode Console
4. Open the settings view ({#action agent::OpenSettings}) and go to the OpenCode section
5. Enter your OpenCode API key

The OpenCode API key will be saved in your keychain.

Zed will also use the `OPENCODE_API_KEY` environment variable if it's defined.

By default, models from all subscription types are shown. Optionally, you can hide subscriptions that are not relevant
to you by clicking the toggles or by adding the following to your settings:

```json [settings]
{
  "language_models": {
    "opencode": {
      "show_zen_models": true,
      "show_go_models": false,
      "show_free_models": false
    }
  }
}
```

**Note:** Zed only bundles configuration for long-term OpenCode Free models! Free models that are only available for a
limited time are not included in Zed. To use such models, create a Custom Model using the configuration settings
published on [the OpenCode website](https://opencode.ai/docs/zen#pricing) and
on [models.dev](https://github.com/anomalyco/models.dev/tree/dev/providers/opencode/models).

#### Custom Models {#opencode-custom-models}

The Zed agent comes pre-configured with OpenCode models. If you wish to use newer models or models with custom
endpoints, you can do so by adding the following to your Zed settings file
([how to edit](../configuring-zed.md#settings-files)):

```json [settings]
{
  "language_models": {
    "opencode": {
      "available_models": [
        {
          "name": "my-custom-model",
          "display_name": "My Custom Model",
          "max_tokens": 123456,
          "max_output_tokens": 98765,
          "protocol": "openai_chat",
          "reasoning_effort_levels": [
            "low",
            "medium",
            "high"
          ],
          "interleaved_reasoning": false,
          "subscription": "go",
          "custom_model_api_url": "https://example.com/zen"
        }
      ]
    }
  }
}
```

The available configuration options for custom models are:

- `name` (required): model id used by OpenCode, for example `glm-9000`
- `display_name` (optional): human-readable model name shown in the UI, for example `Custom GLM 9000`
- `max_tokens` (required): maximum model context window size, for example `1000000`
- `max_output_tokens` (optional): maximum tokens the model can generate, for example `64000`
- `protocol` (required): model API protocol, one of `"anthropic"`, `"openai_responses"`, `"openai_chat"`, or `"google"`
- `reasoning_effort_levels` (optional): list of supported reasoning effort levels, for example
  `["low", "medium", "high"]`. The latest value in the list is used as the default
- `interleaved_reasoning` (optional, default `false`): if thinking tokens are sent as a dedicated `reasoning_content`
  field (`true`) or inline in message text (`false`). Applies only when using the `openai_chat` protocol
- `subscription` (optional): `"zen"`, `"go"`, or `"free"` (defaults to `"zen"`)
- `custom_model_api_url` (optional): custom API base URL to use instead of the default OpenCode API

Custom models will be listed in the model dropdown in the Agent Panel.

### OpenRouter {#openrouter}

OpenRouter provides access to multiple AI models through a single API. It supports tool use for compatible models.

1. Visit [OpenRouter](https://openrouter.ai) and create an account
2. Generate an API key from your [OpenRouter keys page](https://openrouter.ai/keys)
3. Open the settings view ({#action agent::OpenSettings}) and go to the OpenRouter section
4. Enter your OpenRouter API key

The OpenRouter API key will be saved in your keychain.

Zed will also use the `OPENROUTER_API_KEY` environment variable if it's defined.

When using OpenRouter as your assistant provider, you must explicitly select a model in your settings. OpenRouter no
longer provides a default model selection.

Configure your preferred OpenRouter model in `settings.json`:

```json [settings]
{
  "agent": {
    "default_model": {
      "provider": "openrouter",
      "model": "openrouter/auto"
    }
  }
}
```

The `openrouter/auto` model automatically routes your requests to the most appropriate available model. You can also
specify any model available through OpenRouter's API.

#### Custom Models {#openrouter-custom-models}

You can add custom models to the OpenRouter provider by adding the following to your Zed settings file
([how to edit](../configuring-zed.md#settings-files)):

```json [settings]
{
  "language_models": {
    "open_router": {
      "api_url": "https://openrouter.ai/api/v1",
      "available_models": [
        {
          "name": "google/gemini-2.0-flash-thinking-exp",
          "display_name": "Gemini 2.0 Flash (Thinking)",
          "max_tokens": 200000,
          "max_output_tokens": 8192,
          "supports_tools": true,
          "supports_images": true,
          "mode": {
            "type": "thinking",
            "budget_tokens": 8000
          }
        }
      ]
    }
  }
}
```

The available configuration options for each model are:

- `name` (required): The model identifier used by OpenRouter
- `display_name` (optional): A human-readable name shown in the UI
- `max_tokens` (required): The model's context window size
- `max_output_tokens` (optional): Maximum tokens the model can generate
- `max_completion_tokens` (optional): Maximum completion tokens
- `supports_tools` (optional): Whether the model supports tool/function calling
- `supports_images` (optional): Whether the model supports image inputs
- `mode` (optional): Special mode configuration for thinking models

You can find available models and their specifications on the [OpenRouter models page](https://openrouter.ai/models).

Custom models will be listed in the model dropdown in the Agent Panel.

#### Provider Routing

You can optionally control how OpenRouter routes a given custom model request among underlying upstream providers via
the `provider` object on each model entry.

Supported fields (all optional):

- `order`: Array of provider slugs to try first, in order (e.g. `["anthropic", "openai"]`)
- `allow_fallbacks` (default: `true`): Whether fallback providers may be used if preferred ones are unavailable
- `require_parameters` (default: `false`): Only use providers that support every parameter you supplied
- `data_collection` (default: `allow`): `"allow"` or `"disallow"` (controls use of providers that may store data)
- `only`: Whitelist of provider slugs allowed for this request
- `ignore`: Provider slugs to skip
- `quantizations`: Restrict to specific quantization variants (e.g. `["int4","int8"]`)
- `sort`: Sort strategy for candidate providers (e.g. `"price"` or `"throughput"`)

Example adding routing preferences to a model:

```json [settings]
{
  "language_models": {
    "open_router": {
      "api_url": "https://openrouter.ai/api/v1",
      "available_models": [
        {
          "name": "openrouter/auto",
          "display_name": "Auto Router (Tools Preferred)",
          "max_tokens": 2000000,
          "supports_tools": true,
          "provider": {
            "order": [
              "anthropic",
              "openai"
            ],
            "allow_fallbacks": true,
            "require_parameters": true,
            "only": [
              "anthropic",
              "openai",
              "google"
            ],
            "ignore": [
              "cohere"
            ],
            "quantizations": [
              "int8"
            ],
            "sort": "price",
            "data_collection": "allow"
          }
        }
      ]
    }
  }
}
```

These routing controls let you fine‑tune cost, capability, and reliability trade‑offs without changing the model name
you select in the UI.

## Custom Provider Endpoints {#custom-provider-endpoint}

You can use a custom API endpoint for different providers, as long as it's compatible with the provider's API structure.
To do so, add the following to your settings file ([how to edit](../configuring-zed.md#settings-files)):

```json
{
  "language_models": {
    "some-provider": {
      "api_url": "http://localhost:11434"
    }
  }
}
```

Currently, `some-provider` can be any of the following values: `anthropic`, `google`, `ollama`, `openai`.

This is the same infrastructure that powers models that are, for example, [OpenAI-compatible](#openai-api-compatible).
