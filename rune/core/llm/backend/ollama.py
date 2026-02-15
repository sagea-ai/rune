from __future__ import annotations

from collections.abc import AsyncGenerator
import json
import os
from typing import TYPE_CHECKING, Any

from ollama import AsyncClient, ResponseError
import httpx

from rune.core.llm.exceptions import BackendErrorBuilder
from rune.core.types import (
    AvailableTool,
    FunctionCall,
    LLMChunk,
    LLMMessage,
    LLMUsage,
    Role,
    StrToolChoice,
    ToolCall,
)

if TYPE_CHECKING:
    from rune.core.config import ModelConfig, ProviderConfig


class OllamaBackend:
    def __init__(self, provider: ProviderConfig, timeout: float = 720.0) -> None:
        self._client: AsyncClient | None = None
        self._provider = provider
        self._timeout = timeout
        self._api_base = provider.api_base

    async def __aenter__(self) -> OllamaBackend:
        self._client = AsyncClient(host=self._api_base, timeout=self._timeout)
        return self

    async def __aexit__(self, exc_type, exc_val, exc_tb) -> None:
        if self._client:
            # AsyncClient doesn't strictly need close, but good practice if it did
            pass

    def _get_client(self) -> AsyncClient:
        if self._client is None:
            self._client = AsyncClient(host=self._api_base, timeout=self._timeout)
        return self._client

    def _prepare_messages(self, messages: list[LLMMessage]) -> list[dict[str, Any]]:
        json_messages = []
        for msg in messages:
            m = {"role": msg.role.value, "content": msg.content or ""}
            if msg.tool_calls:
                 m["tool_calls"] = [
                     {
                         "function": {
                             "name": tc.function.name,
                             "arguments": json.loads(tc.function.arguments) if isinstance(tc.function.arguments, str) else tc.function.arguments
                         }
                     } for tc in msg.tool_calls
                 ]
            json_messages.append(m)
        return json_messages

    def _prepare_tools(self, tools: list[AvailableTool] | None) -> list[dict[str, Any]] | None:
        if not tools:
            return None
        return [
            {
                "type": "function",
                "function": {
                    "name": tool.function.name,
                    "description": tool.function.description,
                    "parameters": tool.function.parameters,
                }
            } for tool in tools
        ]

    async def complete(
        self,
        *,
        model: ModelConfig,
        messages: list[LLMMessage],
        temperature: float,
        tools: list[AvailableTool] | None,
        max_tokens: int | None,
        tool_choice: StrToolChoice | AvailableTool | None,
        extra_headers: dict[str, str] | None,
    ) -> LLMChunk:
        try:
            options = {"temperature": temperature}
            if max_tokens:
                options["num_predict"] = max_tokens

            response = await self._get_client().chat(
                model=model.name,
                messages=self._prepare_messages(messages),
                tools=self._prepare_tools(tools),
                options=options,
                stream=False,
            )

            message = response.message
            content = message.content
            tool_calls = []
            if message.tool_calls:
                for i, tc in enumerate(message.tool_calls):
                    tool_calls.append(ToolCall(
                        id=f"call_{i}",
                        function=FunctionCall(
                            name=tc.function.name,
                            arguments=json.dumps(tc.function.arguments)
                        ),
                        index=i
                    ))

            return LLMChunk(
                message=LLMMessage(
                    role=Role.assistant,
                    content=content,
                    tool_calls=tool_calls if tool_calls else None,
                ),
                usage=LLMUsage(
                    prompt_tokens=response.prompt_eval_count or 0,
                    completion_tokens=response.eval_count or 0,
                )
            )

        except Exception as e:
             raise BackendErrorBuilder.build_request_error(
                provider=self._provider.name,
                endpoint=self._api_base,
                error=e,
                model=model.name,
                messages=messages,
                temperature=temperature,
                has_tools=bool(tools),
                tool_choice=tool_choice,
            ) from e

    async def complete_streaming(
        self,
        *,
        model: ModelConfig,
        messages: list[LLMMessage],
        temperature: float,
        tools: list[AvailableTool] | None,
        max_tokens: int | None,
        tool_choice: StrToolChoice | AvailableTool | None,
        extra_headers: dict[str, str] | None,
    ) -> AsyncGenerator[LLMChunk, None]:
        try:
            options = {"temperature": temperature}
            if max_tokens:
                options["num_predict"] = max_tokens

            async for chunk in await self._get_client().chat(
                model=model.name,
                messages=self._prepare_messages(messages),
                tools=self._prepare_tools(tools),
                options=options,
                stream=True,
            ):
                message = chunk.message
                content = message.content

                tool_calls = []
                if message.tool_calls:
                    for i, tc in enumerate(message.tool_calls):
                         tool_calls.append(ToolCall(
                            id=f"call_{i}",
                            function=FunctionCall(
                                name=tc.function.name,
                                arguments=json.dumps(tc.function.arguments)
                            ),
                            index=i
                        ))

                yield LLMChunk(
                    message=LLMMessage(
                        role=Role.assistant,
                        content=content,
                        tool_calls=tool_calls if tool_calls else None,
                    ),
                    usage=LLMUsage(
                        prompt_tokens=chunk.prompt_eval_count or 0,
                        completion_tokens=chunk.eval_count or 0,
                    )
                )

        except Exception as e:
            raise BackendErrorBuilder.build_request_error(
                provider=self._provider.name,
                endpoint=self._api_base,
                error=e,
                model=model.name,
                messages=messages,
                temperature=temperature,
                has_tools=bool(tools),
                tool_choice=tool_choice,
            ) from e

    async def count_tokens(
        self,
        *,
        model: ModelConfig,
        messages: list[LLMMessage],
        temperature: float = 0.0,
        tools: list[AvailableTool] | None = None,
        tool_choice: StrToolChoice | AvailableTool | None = None,
        extra_headers: dict[str, str] | None = None,
    ) -> int:
        # Ollama doesn't have a direct count_tokens endpoint easily accessible via python client usually,
        # but we can just return 0 or estimate.
        # Logic: prompt_tokens is returned in response.
        # We can do a dummy generation with max_tokens=1?
        # Or just rely on previous functionality?
        # Mistral backend did a complete(max_tokens=1).

        result = await self.complete(
            model=model,
            messages=messages,
            temperature=temperature,
            tools=tools,
            max_tokens=1,
            tool_choice=tool_choice,
            extra_headers=extra_headers,
        )
        if result.usage is None:
            return 0 # Should not happen
        return result.usage.prompt_tokens
