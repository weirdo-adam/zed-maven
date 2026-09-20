# Third-party notices

This extension downloads and runs the following third-party components at
runtime (they are not bundled in this repository):

## Eclipse LemMinX (org.eclipse.lemminx)

- Source: https://github.com/eclipse-lemminx/lemminx
- License: EPL-2.0
- Built from tag `0.31.2` by our CI with
  [`patches/lemminx-resolve-null.patch`](../patches/lemminx-resolve-null.patch)
  (a one-line fix: `completionItem/resolve` returns the original item instead
  of `null` when the item carries no `data`).

## lemminx-maven

- Source: https://github.com/eclipse-lemminx/lemminx-maven
- License: EPL-2.0
- Built from `master` by our CI, packaged together with its Maven/Java
  dependency jars (all available from Maven Central / Eclipse repositories,
  under their respective licenses — AL-2.0, EPL-1.0, MIT, BSD, W3C, …).
