# Performance run 2026-11-25T19:57:17+00:00

- Label: baseline (before any fix)
- Target: local (http://localhost:4848)
- Commit: c621a15
- Runs per page/profile: 3 (median kept)
- Lighthouse: 12.8.2, Mozilla/5.0  AppleWebKit/537.36 (KHTML, like Gecko) HeadlessChrome/158.0.0.0 Safari/537.36

| Page | Profile | Perf | FCP | LCP | TBT | CLS | SI | Weight | Requests | search-data.json | Report |
|---|---|---|---|---|---|---|---|---|---|---|---|
| home | mobile | 50 | 1.53s | 5.34s | 5844 ms | 0 | 1.53s | 2865 KiB | 22 | 1584 KiB | [html](home_mobile.html) |
| home | desktop | 69 | 0.44s | 0.99s | 1699 ms | 0 | 0.45s | 2865 KiB | 22 | 1584 KiB | [html](home_desktop.html) |
| article | mobile | 69 | 1.69s | 1.75s | 7664 ms | 0 | 1.69s | 2106 KiB | 22 | 1584 KiB | [html](article_mobile.html) |
| article | desktop | 70 | 0.48s | 0.48s | 1983 ms | 0 | 0.48s | 2106 KiB | 22 | 1584 KiB | [html](article_desktop.html) |
