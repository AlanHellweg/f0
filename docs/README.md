# f0 docs

Documentation site for f0, built with [Starlight](https://starlight.astro.build).

| Command | Action |
|---|---|
| `pnpm install` | Install dependencies |
| `pnpm dev` | Start the dev server at `localhost:4321` |
| `pnpm build` | Build the site into `./dist/` |
| `pnpm preview` | Preview the build |

English pages live in `src/content/docs/`, Spanish ones in `src/content/docs/es/` and Simplified Chinese ones in `src/content/docs/zh-cn/`, with the same paths. Starlight UI strings that need overriding go in `src/content/i18n/<lang>.json`. Sidebar order comes from `sidebar.order` in each page's frontmatter.
