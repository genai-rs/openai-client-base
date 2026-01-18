## Codex Notes

- This library must mirror the upstream OpenAI/Stainless specification; do not invent or backfill missing schemas. If upstream drops or omits schemas, prefer to fail fast and surface the upstream issue rather than adding local definitions.  
- Keep generation steps reproducible; avoid caching or pinning bespoke specs unless explicitly approved.  

When upstream specs are missing pieces, document the issue in PRs and link to the upstream source rather than patching the spec locally.
