# LLM Matcher

Read the `GitmojiMatcher` interface in `matcher/mod.rs`.
And implement the `LLMMatcher` struct that uses LLM to match the commit message to a gitmoji.

Here are some problems you need to solve:

## LLM Provider
There are many LLM providers, so you should consider aggregation of different apis.
Almost all the apis need to provide api key, so you should consider how to manage the api keys.
Here is an example:
I use `siliconflow` as my LLM provider. And I apply the api key from its website.
Then I can use the api key to call the api.
```bash
curl --request POST \
  --url https://api.siliconflow.cn/v1/chat/completions \
  --header 'Authorization: Bearer <api_key>' \
  --header 'Content-Type: application/json' \
  --data '{
  "model": "Qwen/Qwen2-7B-Instruct",
  "messages": [
    {
      "role": "user",
      "content": "xxx"
    }
  ]
}'
```
And also, there are methods like openai api, so you should consider how to manage the api keys and abstract the api calls.

## LLM Model
Set a config list for the supportedLLM models.

## Prompt
You need to design the prompt for the LLM.
1. The prompt should be concise and clearly.
2. The prompt should contains all the information of the gitmoji.(./fixtures/gitmojis.json, just need "code" and "description")
3. The prompt should ask the LLM to match the commit message to a gitmoji, and format the response which must only contain the final
commit message in format of ":emoji_code: commit_message". For example:
```
User Commit Message:
    feat: add a new feature

LLM Response:
    :sparkles: add a new feature
```
The LLM also should improve the commit message if it is not good.

## Response Handling
The response from the LLM should be handled although it is asked to generate the standard commit message.
You need to find the pattern of the commit message with gitmoji. If not, just return the original commit message with an error.
And in the main logic, you should consider `SimpleMatcher` when the `LLMMatcher` is failed.

## Structures
```rust
pub enum LLMProvider {
    SiliconFlow,
    OpenAI,
}

pub enum LLMModel {
    Qwen2_7B_Instruct("Qwen/Qwen2-7B-Instruct"),
    Qwen25_7B_Instruct("Qwen/Qwen2.5-7B-Instruct"),
}

pub struct LLMConfig {
    pub provider: LLMProvider,
    pub api_key: String,
    pub model: LLMModel,
}

```
