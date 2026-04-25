# Readwise review

Review Readwise highligts in neovim, fast and easy.

## How it works

1. Fetch recent highlights incrementally from Readwise's export API using `updatedAfter`, paginating with `nextPageCursor`
2. Filter out previously reviewed highlights using a local done list of highlight IDs
3. Generate a single Markdown inbox file with the remaining highlights as tasks
4. Open the file in Neovim
5. Toggle a highlight to `[x]` and save — reviewed IDs are collected into the done list
6. Next launch filters them out automatically

## Why the export API

The Readwise public CLI docs emphasize search/read/export flows. The export API gives exact incremental sync primitives and stable highlight IDs, which is what you need for a review-and-dismiss workflow.

