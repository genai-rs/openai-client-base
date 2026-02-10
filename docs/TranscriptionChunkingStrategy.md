# TranscriptionChunkingStrategy

## Enum Variants

| Name | Description |
|---- | -----|
| String | Controls how the audio is cut into chunks. When set to &#x60;\&quot;auto\&quot;&#x60;, the server first normalizes loudness and then uses voice activity detection (VAD) to choose boundaries. &#x60;server_vad&#x60; object can be provided to tweak VAD detection parameters manually. If unset, the audio is transcribed as a single block.  |
| VadConfig | Controls how the audio is cut into chunks. When set to &#x60;\&quot;auto\&quot;&#x60;, the server first normalizes loudness and then uses voice activity detection (VAD) to choose boundaries. &#x60;server_vad&#x60; object can be provided to tweak VAD detection parameters manually. If unset, the audio is transcribed as a single block.  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


