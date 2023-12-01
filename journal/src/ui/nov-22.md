# Building a screen

1. Add a new file or folder in src/screens
2. Import `use super::*` to get all the screen related traits.
3. Impl `State` for your screen type, which can be a struct.
4. Add the screen to the `view::Page` enum.
5. In the view function, make sure to wrap the content in app_layout.
6. Add the screen to the `app.rs` switch_window function match statement.
7. Add the screen to the list of pages in `page_menu` function.