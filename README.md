<div align="center">
  <h1><code>teloxide-inline-widgets</code></h1>

  <a href="https://github.com/syrtcevvi/teloxide-inline-widgets/actions/workflows/ci.yml">
    <img alt="GitHub Actions Workflow Status" src="https://github.com/syrtcevvi/teloxide-inline-widgets/actions/workflows/ci.yml/badge.svg">
  </a>
  <a href="https://docs.rs/teloxide-inline-widgets/latest/teloxide_inline_widgets/">
    <img alt="docs.rs" src="https://img.shields.io/docsrs/teloxide-inline-widget">
  </a>
  <a href="https://crates.io/crates/teloxide-inline-widgets">
    <img alt="Crates.io Version" src="https://img.shields.io/crates/v/teloxide-inline-widgets">
  </a>
  <img alt="Crates.io Total Downloads" src="https://img.shields.io/crates/d/teloxide-inline-widgets">
</div>

The library for (*easy*?) creation of the inline-keyboard widgets for the [`teloxide`](https://github.com/teloxide/teloxide) framework.

There're some [examples](examples/README.md) available!

## Available widgets
### Radio List
![radio list example](examples/media/radio_list.gif)

### Checkbox List
![checkbox list example](examples/media/checkbox_list.gif)

## Others
One of the most desired thing is to combine widgets together. Currently, the library supports the simple `Layout` method which allows to combine `base-widgets` within single keyboard.

![multiple widgets](examples/media/multiple_widgets.gif)

## Roadmap
- [ ] Simplify API with `proc-macros`
- [ ] Pagination mechanism
- [ ] Add `date-picker` widget
- [ ] Add custom callbacks to some actions
- [ ] Add ability to allow custom limitations to widgets