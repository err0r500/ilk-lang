---
title: Playground
---

# Playground

Try the ilk language directly in your browser — no installation needed. Edit the code on the left; the compiler runs live and shows results on the right.

<Playground />

The example defines an `HttpResponse` type and three valid instances. Try breaking something:

- Remove `status 201` from `creationSuccess` to see a missing required field error
- Change `status 201` to `status "ok"` to trigger a type mismatch
- Add an unknown field to experiment with closed struct validation
