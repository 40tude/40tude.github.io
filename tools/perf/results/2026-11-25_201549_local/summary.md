# Performance run 2026-11-25T20:15:49+00:00

- Label: lazy search index
- Target: local (http://localhost:4848)
- Commit: 22ac8d7+dirty
- Runs per page/profile: 3 (median kept)
- Lighthouse: 12.8.2, Mozilla/5.0  AppleWebKit/537.36 (KHTML, like Gecko) HeadlessChrome/158.0.0.0 Safari/537.36

| Page | Profile | Perf | FCP | LCP | TBT | CLS | SI | Weight | Requests | search-data.json | Report |
|---|---|---|---|---|---|---|---|---|---|---|---|
| home | mobile | 99 | 1.53s | 1.60s | 13 ms | 0 | 1.53s | 1281 KiB | 21 | 0 KiB | [html](home_mobile.html) |
| home | desktop | 99 | 0.43s | 0.97s | 0 ms | 0 | 0.44s | 1281 KiB | 21 | 0 KiB | [html](home_desktop.html) |
| article | mobile | 99 | 1.69s | 1.85s | 0 ms | 0 | 1.69s | 522 KiB | 21 | 0 KiB | [html](article_mobile.html) |
| article | desktop | 100 | 0.48s | 0.49s | 0 ms | 0 | 0.54s | 522 KiB | 21 | 0 KiB | [html](article_desktop.html) |
