# Railways

Flutter State-management library (A superset of Bloc)

## Why yet another state-management library

I don't want to reinvent the wheel, but I was always dissatisfied with how Bloc handles errors (basically have to have it as part of the state).
So I'm going to create my really opinionated library built upon Bloc which handles errors outside of the state.

## Packages

### railways

Main Flutter package

### railways_riverpod

Riverpod mixins for Railways classes

### railways_viaduct

Bridge for the Railways flutter package and the Viaduct Rust-Flutter bindings generator.
